use std::any::TypeId;

use crate::component_lib::Owner;

//Refactor:
// -Events might need to hold timestamps
// -Consider adding a separate queue for buffered events that tracks how many frames they should be retried for since some events I might not want to discard each frame.
// -Move the ability IDs into a separate folder?
// -AutoAttackHit should become ability hit

//Game Ability types used to query an entity's AbilityMap
pub struct AbilityOne;
pub struct AbilityTwo;
pub struct AbilityThree;
pub struct AbilityFour;
pub struct AutoAttack;

#[derive(Debug, Clone)]
pub enum GameEvent {
  //Combat events
  AbilityStart { ability_type: TypeId, owner: Owner },
  AbilityHit { ability_type: TypeId, ability_id: usize, owner: Owner },
  EntityKilled { entity: usize, killer: usize },

  //Movement Events
  UpdateDestination { owner: Owner },

  //Camera Events
  MoveCameraUp,
  MoveCameraDown,
  MoveCameraRight,
  MoveCameraLeft,
  ZoomInCamera,
  ZoomOutCamera,
  CenterCamera,
}

#[derive(Debug, Clone)]
///A stucture which tracks the game events. Does not track input or other changes.
pub struct GameEventQueue {
  events: Vec<GameEvent>,
}

impl GameEventQueue {
  ///Create a new [`GameEventQueue`].
  pub fn new() -> Self {
    GameEventQueue { events: Vec::default() }
  }

  ///Add a [`GameEvent`] to the [`GameEventQueue`].
  pub fn push(&mut self, event: GameEvent) {
    self.events.push(event);
  }

  ///Empties the [`GameEvent`].
  pub fn clear(&mut self) {
    self.events.clear()
  }

  ///Iterates over the [`GameEvent`]s stored in the [`GameEventQueue`] and applies a callback function.
  pub fn process_events<F>(&self, mut f: F)
  where
    F: FnMut(&GameEvent),
  {
    for event in &self.events {
      f(event)
    }
  }

  ///Iterates over the [`GameEvent`]s stored in the [`GameEventQueue`] and applies a callback function which mutates the `GameEvent`.
  pub fn process_events_mut<F>(&mut self, mut f: F)
  where
    F: FnMut(&mut GameEvent),
  {
    for event in &mut self.events {
      f(event)
    }
  }
}
