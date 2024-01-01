use crate::ecs::{World, component_lib::{KDA, Gold}};
use eyre::Result;

use super::{update_target::update_target, spawn_auto_attacks::spawn_auto_attacks, move_attacks::move_attacks, decriment_cooldowns::decriment_cooldowns, resolve_attacks::resolve_attacks};

pub fn combat(world:&mut World) -> Result<()>{
  //update the target to deselect the target when click to move instead of on hover
  update_target(world)?;
  spawn_auto_attacks(world)?;
  decriment_cooldowns(world)?;
  move_attacks(world)?;
  //the actual gold value given on kill probably needs to vary based on external factors
  resolve_attacks(world)?;
  confirm_attack(world)?;
  Ok(())
}

fn confirm_attack(world:&World) -> Result<()> {
  let mut query = world.query();

  let entites = query.with_component::<KDA>()?.run_entity();
  for entity in entites {
    let kda = entity.immut_get_component::<KDA>()?;
    let gold = entity.immut_get_component::<Gold>()?;
    dbg!(kda);
    dbg!(gold);
  }
  Ok(())
}