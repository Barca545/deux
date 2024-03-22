use super::{Cooldown, Owner, Script};
use crate::{
  math::MouseRay,
  time::{ServerTime, Timer},
};
use std::collections::{HashMap, VecDeque};

// Refactor:
// -The CastQueue should track how long something is buffered and delete older buffers
// -Add the can cast checks into the create_ability_cast method. They should be utility functions.

///Structure holding information needed to create an [`AbilityCast`] -- the information needed to cast an ability.
#[derive(Debug)]
pub struct AbilityInfo {
  //Struct that tracks the passage of time, decrement each game logic tick
  cooldown: Cooldown,
  //Number of seconds an ability takes to finish channeling
  cast_time: f64,
  //Scripts governing an Ability's logic
  pub scripts: Script,
}

impl AbilityInfo {
  ///Creates a new [`AbilityInfo`].
  pub fn new(cooldown_duration: f64, server_time: &mut ServerTime, cast_time: f64, scripts: Script) -> Self {
    let cooldown = Cooldown::new(server_time, cooldown_duration);
    AbilityInfo { cooldown, cast_time, scripts }
  }
}

///Component which holds a list of an entity's [`AbilityInfo`].
#[derive(Debug, Default)]
pub struct AbilityMap {
  map: HashMap<u32, AbilityInfo>,
}

impl AbilityMap {
  ///Add a new [`AbilityInfo`] to the [`AbilityMap`].
  pub fn insert(&mut self, ability_slot: u32, ability_info: AbilityInfo) {
    self.map.insert(ability_slot, ability_info);
  }

  pub fn get(&self, ability_slot: u32) -> &AbilityInfo {
    self.map.get(&ability_slot).unwrap()
  }

  ///Creates an [`BufferedAbilityCast`] from the [`AbilityInfo`] in the [`AbilityMap`]'s ability slot.
  pub fn create_ability_cast(&self, slot: &u32, owner: &Owner, mouse: MouseRay) -> Option<BufferedAbilityCast> {
    let info = self.map.get(slot).unwrap();
    //Check whether the cooldown and cost to determine if the ability can be cast.
    //Return the buffered ability if it can be cast.
    if true {
      Some(BufferedAbilityCast::new(info, owner, mouse))
    } else {
      None
    }
  }

  pub fn get_cooldown(&self, ability_slot: u32) -> Cooldown {
    let info = self.map.get(&ability_slot).unwrap();
    info.cooldown.clone()
  }
}

///Component which holds the information needed to create an [`AbilityCast`].
#[derive(Debug, Clone)]
pub struct AbilityCast {
  //Entity casting the ability
  pub owner: Owner,
  //The ability's cooldown
  pub cooldown: Cooldown,
  //Scripts controlling what the cast does
  pub scripts: Script,
  //Data describing where the mouse is hovering in the game's world
  pub mouse: MouseRay,
}

impl From<BufferedAbilityCast> for AbilityCast {
  fn from(value: BufferedAbilityCast) -> Self {
    value.ability
  }
}

///Component which holds an [`AbilityCast`] and its channel time.
#[derive(Debug, Clone)]
pub struct BufferedAbilityCast {
  //Number of seconds an ability takes to cast
  cast_time: f64,
  ability: AbilityCast,
}

impl BufferedAbilityCast {
  ///Creates an [`Option`] which holds an [`BufferedAbilityCast`] created from [`AbilityInfo`].
  pub fn new(info: &AbilityInfo, owner: &Owner, mouse: MouseRay) -> Self {
    let owner = *owner;
    let scripts = info.scripts.clone();
    let cast_time = info.cast_time;
    let cooldown = info.cooldown.clone();
    let ability = AbilityCast {
      owner,
      cooldown,
      scripts,
      mouse,
    };
    BufferedAbilityCast { cast_time, ability }
  }
}

///Component that holds the [`AbilityCast`] currently being channeled by a player and its `channel` [`Cooldown`].
#[derive(Debug, Clone)]
pub struct Casting {
  //Tracks the amount of time left to channel until the ability casts
  channel: Cooldown,
  pub ability: AbilityCast,
}

impl Casting {
  ///Creates a [`Casting`] component from a [`BufferedAbilityCast`].
  pub fn new(buffered_cast: BufferedAbilityCast, server_time: &mut ServerTime) -> Self {
    let channel = Cooldown::new(server_time, buffered_cast.cast_time);
    let ability = AbilityCast::from(buffered_cast);
    Casting { channel, ability }
  }

  pub fn is_done(&self) -> bool {
    self.channel.is_zero()
  }
}

///Component holding the list of abilities
#[derive(Debug, Default, Clone)]
pub struct CastQueue {
  queue: VecDeque<BufferedAbilityCast>,
}

impl CastQueue {
  ///Returns an [`Option`] which holds the next [`AbilityCast`] to cast.
  pub fn next(&mut self) -> Option<BufferedAbilityCast> {
    let buffered_cast = self.queue.pop_front();
    buffered_cast
  }

  ///Add a new [`BufferedAbilityCast`] to the [`CastQueue`].
  pub fn add(&mut self, buffered_cast: BufferedAbilityCast) {
    self.queue.push_back(buffered_cast);
  }

  ///Returns the number of elements in the [`CastQueue`].
  pub fn len(&self) -> usize {
    self.queue.len()
  }
}
