use std::any::TypeId;

use crate::{
  component_lib::{Cooldown, Owner},
  math::MouseRay,
  time::{ServerTime, Timer},
};

//Refactor:
// -Events might need to hold timestamps
// -Consider adding a separate queue for buffered events that tracks how many frames they should be retried for since some events I might not want to discard each frame.
// -Move the ability IDs into a separate folder?
// -Change how abilitites are indexed to not use type ids?
//  Just use u32s instead and keep an index somewhere I can reference
// -AutoAttackHit should become ability hit
// -Could the start event just get passed a ground intersection and target from the intersection instead of holding the mouseray and calculating it?
// -Maybe pending and casting should be two different queues.
//  Pending holds incoming events that can't be processed.
//  Casting holds the ability currently being channeled.

//Game Ability types used to query an entity's AbilityMap
pub struct AbilityOne;
pub struct AbilityTwo;
pub struct AbilityThree;
pub struct AbilityFour;
pub struct AutoAttack;

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
  //Combat events
  BufferAbility { owner: Owner, ability_type: TypeId, mouseray: MouseRay },
  AbilityStart { owner: Owner, ability_type: TypeId, mouseray: MouseRay },
  //Ability id is the entity id of the ability
  AbilityHit { owner: Owner, ability_type: TypeId, ability_id: usize },
  EntityKilled { target: usize, killer: usize },

  //Movement Events
  UpdateDestination { owner: Owner, mouseray: MouseRay },

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
pub struct DelayedEvent {
  timer: Cooldown,
  event: GameEvent,
}

#[derive(Debug, Clone)]
///A stucture which tracks the game events. Does not track input or other changes.
pub struct GameEventQueue {
  events: Vec<GameEvent>,
  pending: Vec<DelayedEvent>,
}

impl GameEventQueue {
  ///Create a new [`GameEventQueue`].
  pub fn new() -> Self {
    GameEventQueue {
      events: Vec::default(),
      pending: Vec::default(),
    }
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

  ///Iterates over the [`GameEvent`]s stored in the [`GameEventQueue`] and applies a callback function which can mutate the `GameEvent` or `GameEventQueue itself`.
  pub fn process_events_mut<F>(&mut self, mut f: F)
  where
    F: FnMut(&mut GameEvent),
  {
    for event in &mut self.events {
      f(event)
    }
  }

  pub fn len(&self) -> usize {
    self.events.len()
  }

  ///Add a [`GameEvent`] to the [`GameEventQueue`]'s `events` field.
  pub fn push(&mut self, event: GameEvent) {
    self.events.push(event);
  }

  ///Add a [`DelayedEvent`] to the [`GameEventQueue`]'s `pending` field.
  pub fn push_pending(&mut self, timer: f64, server_time: &mut ServerTime, event: GameEvent) {
    //this needs to create a new timer with the cd duration instead
    //the move pending needs to make sure to delete the timer
    let timer = Cooldown::new(server_time, timer);
    let event = DelayedEvent { timer, event };
    self.pending.push(event);
  }

  ///Checks whether any [`DelayedEvent`]s' timers are completed. Moves completed events into the [`GameEventQueue`].
  pub fn move_pending(&mut self) {
    //Collect the finished events into a new vector
    let completed = self
      .pending
      .iter()
      .filter_map(|event| if event.timer.is_zero() { Some(event.event) } else { None })
      .collect::<Vec<GameEvent>>();

    //Add the completed events to the current events
    self.events.extend(completed);

    //Remove the finished event from the pending queue
    self.pending.retain(|event| !event.timer.is_zero());
  }
}
