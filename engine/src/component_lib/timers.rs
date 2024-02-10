use serde::{Deserialize, Serialize};

// Refactor
// -Bring in the seconds alias from the ServerTime mod 
// -Is there a way to make it so Timer hooks into the ServerTimer struck directly so all instances automatically decrement?
// -Should Timer be a trait and not a struct or maybe have timer be a trait and then have BasicTimer be the struct

pub trait Timer{
  ///Creates a new `Timer` instance from a max value.
  fn new(duration:f64) -> Self;

  ///Returns the duration of the `Timer`.
  fn duration(&self) -> f64;
  
  ///Returns the time remaining on the `Timer`.
  fn remaining(&self) -> f64;
  
  ///Decrements the `Timer` by the time passed.
  fn decrement(&mut self, passed:f64);
  
  ///Sets the remaining on the `Timer` to 0.0.
  fn zero(&mut self);

  ///Resets the duration of the `Timer`.
  fn reset(&mut self);
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component for tracking time. To use, decremint the `remaining` field eack tick by the amount of time passed.
pub struct BasicTimer{
  duration:f64,
  remaining:f64
}
impl Timer for BasicTimer{
  fn new(duration:f64) -> Self {
    BasicTimer { 
      duration, 
      remaining: duration
    }
  }

  fn duration(&self) -> f64 {
    self.duration
  }

  fn remaining(&self) -> f64 {
    self.remaining
  }
  
  fn decrement(&mut self, passed:f64){
    self.remaining -= passed;
  }

  fn zero(&mut self){
    self.remaining = 0.0;
  }

  fn reset(&mut self){
    self.remaining = self.duration;
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
///Component containg a `Timer`. 
/// Use to track the time remaining until an entity can auto attack again. 
pub struct AutoAttackCooldown(BasicTimer);

impl Timer for AutoAttackCooldown{
  ///Creates a new `AutoAttackCooldown` instance from the max value.
  fn new(max:f64) -> Self {
    AutoAttackCooldown(BasicTimer::new(max))
  }

  ///Returns the duration of the `AutoAttackCooldown`'s `Timer`.
  fn duration(&self) -> f64 {
    self.0.duration()
  }

  ///Returns the remaining time in the `AutoAttackCooldown`'s `Timer` duration.
  fn remaining(&self) -> f64 {
    self.0.remaining()
  }

  ///Decrements the `AutoAttackCooldown`'s `Timer` by the time passed.
  fn decrement(&mut self, passed:f64){
    self.0.decrement(passed)
  }

  ///Sets the remaining cooldown to 0.0.
  fn zero(&mut self){
    self.0.zero()
  }

  ///Resets the cooldown.
  fn reset(&mut self) {
    self.0.remaining = self.0.duration();
  }
}