use crate::time::{BasicTimer, Miliseconds, ServerTime, Timer};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Refactor:
// -Add script plugin to create, check and remove cooldowns.
// -Remove the AutoAttackCooldown structure
// -Make the names an enum?

pub struct Cooldowns(HashMap<String, Cooldown>);

impl Cooldowns {
  pub fn new(
    server_time: &mut ServerTime, auto: Miliseconds, auto_windup: Miliseconds, ability_1: Miliseconds, ability_2: Miliseconds,
    ability_3: Miliseconds, ability_4: Miliseconds,
  ) -> Self {
    let mut cooldowns = HashMap::new();
    cooldowns.insert(String::from("auto attack"), Cooldown::new(server_time, auto));
    cooldowns.insert(String::from("auto windup"), Cooldown::new(server_time, auto_windup));
    cooldowns.insert(String::from("ability 1"), Cooldown::new(server_time, ability_1));
    cooldowns.insert(String::from("ability 2"), Cooldown::new(server_time, ability_2));
    cooldowns.insert(String::from("ability 3"), Cooldown::new(server_time, ability_3));
    cooldowns.insert(String::from("ability 4"), Cooldown::new(server_time, ability_4));

    Cooldowns(cooldowns)
  }

  pub fn get_duration(&self, cooldown_name: &str) -> Miliseconds {
    self.0.get(cooldown_name).unwrap().duration()
  }

  pub fn get_real_remaing(&self, cooldown_name: &str) -> Miliseconds {
    self.0.get(cooldown_name).unwrap().real_remaining()
  }

  pub fn get_display_remaing(&self, cooldown_name: &str) -> Miliseconds {
    self.0.get(cooldown_name).unwrap().display_remaining()
  }

  pub fn zero_real(&mut self, cooldown_name: &str) {
    self.0.get_mut(cooldown_name).unwrap().zero_real()
  }

  pub fn zero_display(&mut self, cooldown_name: &str) {
    self.0.get_mut(cooldown_name).unwrap().zero_display()
  }

  pub fn reset(&mut self, cooldown_name: &str) {
    self.0.get_mut(cooldown_name).unwrap().reset()
  }

  pub fn update_duration(&mut self, cooldown_name: &str, new_duration: Miliseconds) {
    self.0.get_mut(cooldown_name).unwrap().update_duration(new_duration);
  }

  pub fn remove(&mut self, server_time: &mut ServerTime, cooldown_name: &str) {
    let cooldown = self.0.remove(cooldown_name).unwrap();
    server_time.remove_timer(cooldown.index)
  }
}

#[derive(Debug, Clone)]
///Component containg a `Timer`.
/// Use to track the time remaining until an entity can auto attack again.
pub struct Cooldown {
  timer: Rc<RefCell<BasicTimer>>,
  index: usize,
}

impl Timer for Cooldown {
  ///Returns the [`Cooldown`]'s duration.
  fn duration(&self) -> f64 {
    self.timer.borrow().duration()
  }

  ///Returns the [`Cooldown`]'s real remaining time in [`Miliseconds`].
  fn real_remaining(&self) -> Miliseconds {
    self.timer.borrow().real_remaining()
  }

  ///Returns the [`Cooldown`]'s display remaining time in [`Miliseconds`].
  fn display_remaining(&self) -> Miliseconds {
    self.timer.borrow().display_remaining()
  }

  fn zero_real(&mut self) {
    self.timer.borrow_mut().zero_real()
  }

  fn zero_display(&mut self) {
    self.timer.borrow_mut().zero_display()
  }

  fn reset(&mut self) {
    self.timer.borrow_mut().reset();
  }

  fn update_duration(&mut self, new_duration: Miliseconds) {
    self.timer.borrow_mut().update_duration(new_duration);
  }
}

impl Cooldown {
  ///Creates a new `AutoAttackCooldown` instance from the max value.
  pub fn new(server_time: &mut ServerTime, max: f64) -> Self {
    let (timer, index) = server_time.new_timer(max);
    Cooldown { timer, index }
  }
}
