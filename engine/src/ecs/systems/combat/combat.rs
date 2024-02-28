use crate::{component_lib::{Gold, Player, KDA}, ecs::{world_resources::DebugElements, World}};
use super::{move_attacks::move_attacks, resolve_attacks::resolve_attacks, scripts::run_scripts, spawn_auto_attacks::spawn_auto_attacks};
// Refactor:
// -Scripts shouldn't run each frame. 
// -Scripts need some tag or something that indicate when they should run.
// -Update Target should possibly be part of selection


pub fn combat(world:&mut World){
  //Check for a combat event
  //if StartAuto/abilitynumber spawn attacks and update cooldowns
  //Spawning the attacks should be handled by scripts 
  //Wherever the events are created needs to check the target is valid and the auto cooldown is 0

  // spawn_auto_attacks(world);
  //move the attacks -> Should this go into the movement system?
  run_scripts(world);
  move_attacks(world);
  //Look for an attackhit event and resolve them and any onhit scripts
  //Maybe resolve events is a separate system and this goes there
  resolve_attacks(world);
  //only run if debug attacks is enabled
  let debug = world.get_resource::<DebugElements>().unwrap();
  if debug.attacks {
    debug_combat(world);
  }
}

fn debug_combat(world:&World) {
  let mut query = world.query();

  let entites = query.with_component::<KDA>().unwrap().run();
  for entity in entites {
    //debugs the info of the dummy a player is hitting
    if let Err(_player) = entity.immut_get_component::<Player>() {
      let kda = entity.immut_get_component::<KDA>().unwrap();
      let gold = entity.immut_get_component::<Gold>().unwrap();
      dbg!(entity.id);
      dbg!(kda);
      dbg!(gold);
    }
  }
}