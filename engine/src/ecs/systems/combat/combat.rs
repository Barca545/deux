use crate::{component_lib::{Gold, Player, KDA}, ecs::{world_resources::DebugElements, World}};
use super::{decrement_cooldowns::decrement_cooldowns, move_attacks::move_attacks, resolve_attacks::resolve_attacks, scripts::run_scripts, spawn_auto_attacks::spawn_auto_attacks, update_target::update_target};

pub fn combat(world:&mut World){
  update_target(world);
  spawn_auto_attacks(world);
  decrement_cooldowns(world);
  move_attacks(world);
  run_scripts(world);
  resolve_attacks(world);
  //only run if debug attacks is enabled
  let debug = world.immut_get_resource::<DebugElements>().unwrap();
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