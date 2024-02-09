use crate::ecs::{World, component_lib::{Gold, Player, KDA}, world_resources::DebugElements};
use eyre::Result;

use super::{decriment_cooldowns::decriment_cooldowns, move_attacks::move_attacks, resolve_attacks::resolve_attacks, scripts::run_scripts, spawn_auto_attacks::spawn_auto_attacks, update_target::update_target};

pub fn combat(world:&mut World) -> Result<()>{
  update_target(world)?;
  spawn_auto_attacks(world)?;
  decriment_cooldowns(world)?;
  move_attacks(world)?;
  run_scripts(world)?;
  resolve_attacks(world)?;
  //only run if debug attacks is enabled
  let debug = world.immut_get_resource::<DebugElements>().unwrap();
  if debug.attacks {
    debug_combat(world)?;
  }
  Ok(())
}

fn debug_combat(world:&World) -> Result<()> {
  let mut query = world.query();

  let entites = query.with_component::<KDA>()?.run_entity();
  for entity in entites {
    //debugs the info of the dummy a player is hitting
    if let Err(_player) = entity.immut_get_component::<Player>() {
      let kda = entity.immut_get_component::<KDA>()?;
      let gold = entity.immut_get_component::<Gold>()?;
      dbg!(entity.id);
      dbg!(kda);
      dbg!(gold);
    }
  }
  Ok(())
}