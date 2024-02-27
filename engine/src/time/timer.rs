use serde::{Deserialize, Serialize};
use super::aliases::Miliseconds;

pub trait Timer{
  ///Returns the duration of the [`Timer`] in [`Miliseconds`].
  fn duration(&self) -> Miliseconds;
  
  ///Returns the [`Timer`]'s remaining time. This is updated each game logic tick.
  fn real_remaining(&self) -> Miliseconds;

  ///Returns the [`Timer`]'s remaining time. This is updated each render tick.
  fn display_remaining(&self) -> Miliseconds;
  
  ///Sets the [`Timer`]'s real remaining time to 0.0.
  fn zero_real(&mut self);

  ///Sets the [`Timer`]'s display remaining time to 0.0.
  fn zero_display(&mut self);

  ///Resets the [`Timer`]'s duration..
  fn reset(&mut self);

  ///Updates the [`Timer`]'s duration.
  fn update_duration(&mut self, new_duration:Miliseconds);
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component for tracking time. Base structure all other timers in the game must use. 
/// To use, decremint the `remaining` field eack tick by the amount of time passed.
pub struct BasicTimer{
  duration: Miliseconds,
  real_remaining: Miliseconds,
  display_remaining: Miliseconds
}

impl Timer for BasicTimer{
  fn duration(&self) -> Miliseconds {
    self.duration
  }

  fn real_remaining(&self) -> Miliseconds {
    self.real_remaining
  }

  fn display_remaining(&self) -> Miliseconds {
    self.display_remaining
  }

  fn zero_real(&mut self){
    self.real_remaining = 0.0;
  }

  fn zero_display(&mut self){
    self.display_remaining = 0.0;
  }

  fn reset(&mut self){
    self.real_remaining = self.duration;
    self.display_remaining = self.duration;
  }

  fn update_duration(&mut self, new_duration:Miliseconds){
    self.duration = new_duration;
  }
}

impl BasicTimer {
  ///Creates a new [`BasicTimer`].
  pub fn new(duration:Miliseconds) -> Self {
    BasicTimer { 
      duration, 
      real_remaining: duration,
      display_remaining: duration
    }
  }
  
  ///Decrements the [`Timer`]'s real remaining time by the time passed. 
  pub fn decrement_real_remaining(&mut self, passed:Miliseconds){
    self.real_remaining -= passed;
  }

  ///Decrements the [`Timer`]'s display remaining time by the time passed. 
  pub fn decrement_display_remaining(&mut self, passed:Miliseconds){
    self.display_remaining -= passed;
  }
}