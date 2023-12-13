use std::mem::zeroed;
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
type Minutes = i32;
type Seconds = f64;
type PerSecond = f64;
type Count = f64;

//I kinda don't love this being it's own struct
#[derive(Debug)]
pub struct GameDuration {
	minutes: Minutes,
	seconds_in_current_minute: i32,
	seconds_since_start: Seconds,
}

impl GameDuration {
	//Should this actually be intialized to zero? I think it should for consistency. Worst case it just get's overwritten to the accurate state the next count
	pub fn new() -> Self {
		GameDuration {
			minutes: 0,
			seconds_in_current_minute: 0,
			seconds_since_start: 0.0,
		}
	}

	pub fn update(&mut self, seconds: Seconds) {
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

//if I later find out it is bad to use custom types this way just make the types a comment next to the line
//might eventually need a ClientTime and make the methods into a trait or something

pub struct ServerTime {
	start_count: Count,
	current_count: Count,
	previous_count: Count,
	game_duration: GameDuration,   //might need to be inside and Rc
	seconds_since_render: Seconds, //unprocessed time? This is the time left over if the time between frames was bigger than a tick
	seconds_since_update: Seconds,
	counts_per_second: PerSecond,
	tick_frequency: PerSecond, //this might need a different name since it is not the ticks per second but the duration of one tick (one tick is 1/60th of a second)
	render_frequency: PerSecond,
}

impl ServerTime {
	pub fn new() -> Self {
		let start = Self::get_system_count_current();

		ServerTime {
			start_count: start,
			current_count: start,
			previous_count: start,
			game_duration: GameDuration::new(),
			seconds_since_render: 0.0, //Should this actually be intialized to zero? I think it should for consistency. Worst case it just get's overwritten to the accurate state the next count
			seconds_since_update: 0.0,
			counts_per_second: Self::get_counts_per_second(),
			tick_frequency: 1.0 / 60.0,
			render_frequency: 1.0 / 240.0,
		}
	}

	pub fn tick(&mut self) {
		self.previous_count = self.current_count;
		self.current_count = Self::get_system_count_current();

		self.update_seconds_since_last_count();
		self.update_game_duration();
		// self.update
	}

	///Sets the game's `ticks_per_second`. Default value is 60.
	pub fn with_ticks_per_seconds(&mut self, ticks_per_second: Seconds) {
		self.tick_frequency = 1.0 / ticks_per_second
	}

	/// Updates the ServerTime's `seconds_since_last_update` and `unrendered_seconds` fields.  
	/// Returns the "time" passed since this `update_seconds_since_last_count()` was last called.
	/// Calculates time by subtracting the previously registered count from the newly queried count and dividing the result by the system's counts per second.
	/// Must execute first in a game loop.
	fn update_seconds_since_last_count(&mut self) {
		let seconds_since_last_count =
			(self.current_count - self.previous_count) / self.counts_per_second;
		self.seconds_since_update += seconds_since_last_count;
		self.seconds_since_render += seconds_since_last_count;
	}

	///Calculates the time since the game started by subtracting the start count from the current count and dividing by the system's frequency.
	fn update_game_duration(&mut self) {
		let seconds_since_start = (self.current_count - self.start_count) / self.counts_per_second;
		self.game_duration.update(seconds_since_start);
	}

	/**
	Compares the amount of unrendered time to engine's ticks per second and returns a boolean whose `true` value indicates the system should render/update.
	Use in a while loop with `Timer::decrimint_unrendered_time()` to render an amount of time from the unrendered time equal to the value of one game engine tick.

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

			server_time.decrimint_seconds_since_update();
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

	///Use at the end of a loop, decriments the unrendered time by the time 1 tick takes.
	pub fn decrimint_seconds_since_render(&mut self) {
		if self.seconds_since_render != 0.0 {
			self.seconds_since_render -= self.render_frequency;
		}
	}

	pub fn decrimint_seconds_since_update(&mut self) {
		//is it better to set this to zero or let it accumulate by just subtracting the frequency
		self.seconds_since_update -= self.tick_frequency;
	}

	//this docnote is wrong about what an interpolation factor is and also the calculation is wrong

	//I do not think this calculation for the interpolation factor is accurate
	///Returns the amount of time to render.
	pub fn get_interpolation_factor(&self) -> Seconds {
		let interpolation_factor = self.seconds_since_render / self.tick_frequency;
		interpolation_factor
	}

	// pub fn timer_in_minutes_seconds(&self,minutes:u32,seconds:u32)->Seconds{
	//   // let seconds = (minutes*60+seconds) as Seconds;
	//   todo!()
	// }

	// pub fn timer_in_seconds(&self,seconds:u32)->Seconds{
	//   todo!()
	// }

	// //is milliseconds a good interval?
	// pub fn timer_in_milliseconds(&self,miliseconds:u32)->Seconds{
	//   // let seconds = (miliseconds/1000) as Seconds;
	//   todo!()
	// }

	fn get_counts_per_second() -> PerSecond {
		//only reliable on a single core
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

	pub fn update_render_frequency(&mut self, hz: u32) {
		self.render_frequency = 1.0 / (hz as f64)
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
				server_time.decrimint_seconds_since_update();
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
