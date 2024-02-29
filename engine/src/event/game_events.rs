use crate::component_lib::Owner;

//Refactor:
// -Events might need to hold timestamps
// -Consider adding a separate queue for buffered events that tracks how many frames they should be retried for since some events I might not want to discard each frame.

pub enum GameEvent {
  //Combat events
  AutoAttackStart { owner: Owner },
  AutoAttackHit { attack_id: usize, owner: Owner },
  UpdateDestination { owner: Owner },
}

///A sturcture which tracks the game events. Does not track input or other changes.
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
  pub fn process_events_mut<F>(&mut self, f: F)
  where
    F: Fn(&mut GameEvent),
  {
    for event in &mut self.events {
      f(event)
    }
  }
}

//ok new plan each system just registers events and then I have a resolve ticks system at the end that consumes all of the events
