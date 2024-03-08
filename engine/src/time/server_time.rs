use std::{cell::RefCell, mem::zeroed, rc::Rc};
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};

use super::{aliases::Miliseconds, BasicTimer, Count, GameDuration, PerSecond, Seconds, Timer};

//  Refactor:
// -The timers should all decrement each time the game logic changes
// -Interpolation factor is calculated wrong
// -Interpolation factor documentation/definition is wrong
// -Document what the tick function is doing
// -Why do I have the additional if check in the render decrement function?
// -Is it better to set the time_since_last measurements to 0.0 after decementing or let them accumulate by just subtracting the frequency?
// -Add logic for deleting timers and updating their duration
// -See if updating so it does not decrement the timers is a performance gain

#[derive(Debug, Clone)]
pub struct ServerTime {
  //Number of the CPU count the game started on
  start_count: Count,
  //Number of the current CPU count
  current_count: Count,
  //Number of the previous CPU count
  previous_count: Count,
  //Measurement of the CPU's count frequency
  counts_per_second: PerSecond,
  //Measurement of the CPU's count frequency
  game_duration: GameDuration,
  //Time left over if the time between render frames was bigger than a tick
  seconds_since_render: Seconds,
  //Time left over if the time between logic frames was bigger than a tick
  seconds_since_update: Seconds,
  tick_frequency: PerSecond,
  render_frequency: PerSecond,
  //Bit masks of a timer, used to find if a timer's index is free for use
  timermap: Vec<bool>,
  timers: Vec<Rc<RefCell<BasicTimer>>>,
}

impl ServerTime {
  pub fn new() -> Self {
    let start = Self::get_system_count_current();
    let counts_per_second = Self::get_counts_per_second();

    ServerTime {
      start_count: start,
      current_count: start,
      previous_count: start,
      game_duration: GameDuration::new(),
      seconds_since_render: 0.0,
      seconds_since_update: 0.0,
      counts_per_second,
      tick_frequency: 1.0 / 60.0,
      render_frequency: 1.0 / 240.0,
      timermap: Vec::default(),
      timers: Vec::default(),
    }
  }

  pub fn tick(&mut self) {
    self.previous_count = self.current_count;
    self.current_count = Self::get_system_count_current();

    self.update_seconds_since_last_count();
    self.update_game_duration();
  }

  ///Sets the game's `ticks_per_second`. Default value is 60.
  pub fn with_ticks_per_seconds(&mut self, ticks_per_second: Seconds) {
    self.tick_frequency = 1.0 / ticks_per_second
  }

  /// Updates the [`ServerTime`]'s `seconds_since_last_update` and
  /// `unrendered_seconds` fields. Returns the "time" passed since this
  /// `update_seconds_since_last_count()` was last called. Calculates time by
  /// subtracting the previously registered count from the newly queried count
  /// and dividing the result by the system's counts per second. Must execute
  /// first in a game loop.
  fn update_seconds_since_last_count(&mut self) {
    let seconds_since_last_count = (self.current_count - self.previous_count) / self.counts_per_second;
    self.seconds_since_update += seconds_since_last_count;
    self.seconds_since_render += seconds_since_last_count;
  }

  ///Calculates the time since the game started by subtracting the start count
  /// from the current count and dividing by the system's frequency.
  fn update_game_duration(&mut self) {
    let seconds_since_start = (self.current_count - self.start_count) / self.counts_per_second;
    self.game_duration.update(seconds_since_start);
  }

  /**
  Compares the amount of unrendered time to engine's ticks per second and returns a boolean whose `true` value indicates the system should render/update.
  Use in a while loop with `Timer::decrement_unrendered_time()` to render an amount of time from the unrendered time equal to the value of one game engine tick.

  # Examples

  ```
  use time::ServerTime;

  let mut server_time = ServerTime::new();
  let mut server_time = ServerTime::new();
  let mut current_time = 0.0;
  let ticks_per_second = 1.0/60.0;
  let mut number_of_ticks = 0;

  loop{
    server_time.tick();

    if server_time.should_update(){
      current_time = server_time.get_game_duration().get_seconds_since_start();
      number_of_ticks+=1;

      server_time.decrement_seconds_since_update();
    }

    if current_time >= 5.0{
      assert!(number_of_ticks>=300);
      dbg!(current_time);
      break;
    }
  }
  ```
  */
  pub fn should_update(&self) -> bool {
    if self.seconds_since_update >= self.tick_frequency {
      true
    } else {
      false
    }
  }

  pub fn should_render(&self) -> bool {
    if self.seconds_since_render >= self.render_frequency {
      true
    } else {
      false
    }
  }

  pub fn get_game_duration(&self) -> &GameDuration {
    &self.game_duration
  }

  ///Use at the end of a loop.
  /// Decrements the unrendered time by the time render frequency.
  /// Decrements the displayed remaining time for any timers in the game.
  pub fn decrement_seconds_since_render(&mut self) {
    if self.seconds_since_render != 0.0 {
      self.seconds_since_render -= self.render_frequency;
      self.decrement_display_remaining()
    }
  }

  ///Use at the end of a loop.
  /// Decrements the game logic time by the time logic tick frequency.
  /// Decrements the real remaining time for any timers in the game.
  pub fn decrement_seconds_since_update(&mut self) {
    self.seconds_since_update -= self.tick_frequency;
    self.decrement_real_remaining()
  }

  ///Returns the amount of time to render.
  pub fn get_interpolation_factor(&self) -> Seconds {
    let interpolation_factor = self.seconds_since_render / self.tick_frequency;
    interpolation_factor
  }

  pub fn update_render_frequency(&mut self, hz: u32) {
    self.render_frequency = 1.0 / (hz as f64)
  }

  pub fn update_tick_frequency(&mut self, hz: u32) {
    self.tick_frequency = 1.0 / (hz as f64)
  }

  pub fn get_tick_frequency(&self) -> Seconds {
    self.tick_frequency
  }
}

//Implementation block for timers
impl ServerTime {
  ///Adds a new timer to the [`ServerTime`]'s list of timers and returns an [`Rc`] to the timer alongside its index.
  pub fn new_timer(&mut self, duration: Miliseconds) -> (Rc<RefCell<BasicTimer>>, usize) {
    let timer = Rc::new(RefCell::new(BasicTimer::new(duration)));
    if let Some((index, _)) = self.timermap.iter().enumerate().find(|(_index, free)| **free == true) {
      self.timers[index] = timer;
      let timer = self.timers[index].clone();
      (timer, index)
    } else {
      self.timers.push(timer);
      let index = self.timers.len() - 1;
      let timer = self.timers[index].clone();
      (timer, index)
    }
  }

  pub fn remove_timer(&mut self, index: usize) {
    self.timermap[index] = false;
  }

  ///Decrements all of the game's [`BasicTimer`]s by the duration of one logic tick.
  fn decrement_real_remaining(&self) {
    for timer in &self.timers {
      let mut borrowed_timer = timer.borrow_mut();
      borrowed_timer.decrement_real_remaining(self.tick_frequency);
      //Make sure the remaining time is not less than 0.0
      if borrowed_timer.real_remaining() <= 0.0 {
        borrowed_timer.zero_real()
      }
    }
  }

  ///Decrements all of the game's [`BasicTimer`]s by the duration of one render tick.
  fn decrement_display_remaining(&self) {
    for timer in &self.timers {
      let mut borrowed_timer = timer.borrow_mut();
      borrowed_timer.decrement_display_remaining(self.tick_frequency);
      if borrowed_timer.display_remaining() <= 0.0 {
        borrowed_timer.zero_display()
      }
    }
  }
}

//Implementation block for getting the CPU's count information
impl ServerTime {
  fn get_counts_per_second() -> PerSecond {
    //Only reliable on a single core
    let freq = unsafe {
      let mut freq = zeroed();
      QueryPerformanceFrequency(&mut freq);
      *freq.QuadPart() as PerSecond
    };
    freq
  }

  fn get_system_count_current() -> Count {
    let count = unsafe {
      let mut count = zeroed();
      QueryPerformanceCounter(&mut count);
      *count.QuadPart() as Count
    };
    count
  }
}

#[cfg(test)]
mod tests {
  use super::ServerTime;
  use std::mem::zeroed;
  use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};

  //adding the GameDuration struct broke this test
  #[test]
  fn updates_on_time() {
    let mut server_time = ServerTime::new();
    let mut current_time = 0.0;
    let ticks_per_second = 1.0 / 60.0;
    let mut number_of_ticks = 0;

    loop {
      server_time.tick();

      if server_time.should_update() {
        current_time = server_time.get_game_duration().get_seconds_since_start();
        number_of_ticks += 1;

        assert!(server_time.seconds_since_update >= ticks_per_second);
        dbg!(current_time);
        server_time.decrement_seconds_since_update();
      }

      if current_time >= 5.0 {
        assert!(number_of_ticks >= 300);
        dbg!(current_time);
        break;
      }
    }
  }

  #[test]
  fn time_does_ellapse() {
    let mut last_time = counter();
    let count_freq = freq();
    dbg!(count_freq);

    let mut total_seconds = 0.0;
    let mut last_second = 0.0;

    while total_seconds <= 10.0 {
      let current_time = counter();
      let time_elapsed = (current_time - last_time) / count_freq;
      total_seconds += time_elapsed;

      if total_seconds >= last_second + 1.0 {
        last_second += 1.0;
        dbg!(total_seconds);
      }
      last_time = current_time;
    }
  }

  #[test]
  fn counter_does_update() {
    let mut count;

    let mut tick: u64 = 0;

    loop {
      if tick < 20 {
        count = counter();
        tick += 1;
        println!("Tick:{}, Count:{}", tick, count);
      } else {
        break;
      }
    }
  }

  fn counter() -> f64 {
    let count = unsafe {
      let mut count = zeroed();
      QueryPerformanceCounter(&mut count);
      *count.QuadPart() as f64
    };
    count
  }

  fn freq() -> f64 {
    let freq = unsafe {
      let mut freq = zeroed();
      QueryPerformanceFrequency(&mut freq);
      *freq.QuadPart() as f64
    };
    freq
  }
}
