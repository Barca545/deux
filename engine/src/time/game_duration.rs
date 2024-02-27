use super::{Minutes, Seconds};

// Refactor:
// -Evaluate if I need all the stuff to fetch times. I think I might for display purposes.

#[derive(Debug, Clone, Copy)]
pub struct GameDuration {
  minutes:Minutes,
  seconds_in_current_minute:i32,
  seconds_since_start:Seconds
}

impl GameDuration {
  pub fn new() -> Self {
    GameDuration {
      minutes:0,
      seconds_in_current_minute:0,
      seconds_since_start:0.0
    }
  }

  pub fn update(&mut self, seconds:Seconds) {
    self.minutes = (seconds / 60.0) as i32;
    self.seconds_since_start = seconds;
    self.seconds_in_current_minute = (seconds % 60.0) as i32;
  }

  pub fn get_seconds_since_start(&self) -> Seconds {
    self.seconds_since_start
  }

  pub fn get_minutes(&self) -> Minutes {
    self.minutes
  }

  pub fn get_seconds_in_current_minute(&self) -> i32 {
    self.seconds_in_current_minute
  }
  // pub fn get_nanoseconds(&self)->Seconds{todo!()}

  // pub fn get_milliseconds(&self)->Seconds{todo!()}
}