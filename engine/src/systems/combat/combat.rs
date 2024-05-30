use nina::world::World;

use super::{ability_hit_resolve::ability_hit_resolve, process_damage::process_damage};
use crate::data_lib::{DebugElements, Gold, Player, KDA};

// Refactor:
// -Need functionality to add/replace abilties in the map
// -Need the ability to control the logic of abilties from scripts

// Refactor - Casting:
// -Need check that confirms they're not casting another spell
// -The checks should maybe be script side and not exe side
// -Need cast timers that take x amount of time before casting
// -Need player state check where a player can only be doing one cast/movement
// at a time -Buffer for events so If you try to cast one action while another
// is going it will queue the next action -Figure out how to ensure the first
// ability processed is the first one pressed. I might need a DequeVec or
// something? -Cooldowns should have a unique id they're identified by instead
// of a string  stuff that wants to access them just holds that id.
//  need to make a script strict that holds a bundle of all the data a script
// might need to run  could do it as a lazier version of the resources struct?
// that way the types it holds are not hard coded? -Add handling or if the tests
// are failed

pub fn combat(world:&mut World) {
  ability_hit_resolve(world);
  process_damage(world);
  //Only run if debug attacks is enabled
  let debug = world.get_resource::<DebugElements>();
  if debug.attacks {
    debug_combat(world);
  }
}

fn debug_combat(world:&World) {
  let mut query = world.query();

  let entites = query.with_component::<KDA>().unwrap().run();
  for entity in entites {
    //debugs the info of the dummy a player is hitting
    if let Err(_player) = entity.get_component::<Player>() {
      let kda = entity.get_component::<KDA>().unwrap();
      let gold = entity.get_component::<Gold>().unwrap();
      dbg!(entity.id);
      dbg!(kda);
      dbg!(gold);
    }
  }
}
