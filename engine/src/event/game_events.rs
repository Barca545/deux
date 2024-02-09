//combat needs to register the entity id of the attack that hit 
// and the id of the entity that was hit
//does it need a timestamp/tickstamp?

// use std::collections::HashMap;

// so maybe what I can do is instad of the event holding the timestamp I have a hashmap with timestamps as the index
//then it just listens for events in the current tick and discards the rest?  
pub enum GameEvent{
  //Combat events
  AutoAttackHit{
    timestamp: i64,
    attack_id: usize,
    target_id: usize
  },
  AutoAttackSpawn
}


///A sturcture which tracks the game events. Does not track input or other changes.
pub struct GameEventQueue{
  //takes a count as the index and a game event vec as the data
  //count will have to be an int generated by the timer
  events:Vec<GameEvent>
}

impl GameEventQueue {
  pub fn add_event(&mut self, event:GameEvent){
    self.events.push(event);
  }
  
  //this should take in an event type 
  //debating if it should take in the system as an fn mut as an argument or just spit out the events that match the requested type
  //for not let's just have it return all the events matching the timestamp and type
  pub fn process_event(&mut self){}
}


//ok new plan each system just registers events and then I have a resolve ticks system at the end that consumes all of the events


