use crate::ecs::World;
use eyre::Result;

use super::{update_target::update_target, spawn_auto_attacks::spawn_auto_attacks, move_attacks::move_attacks, decriment_cooldowns::decriment_cooldowns, resolve_attacks::resolve_attacks};

pub fn combat(world:&mut World) -> Result<()>{
  //update the target to deselect the target when click to move instead of on hover
  update_target(world)?;
  spawn_auto_attacks(world)?;
  decriment_cooldowns(world)?;
  move_attacks(world)?;
  //need a step in resolve attacks that kills anyone 0>health.remaing and gives gold to the owner of the attack that killed them
  resolve_attacks(world)?;
  Ok(())
}