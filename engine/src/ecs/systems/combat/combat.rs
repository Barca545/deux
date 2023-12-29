use crate::ecs::World;
use eyre::Result;

use super::{update_target::update_target, spawn_auto_attacks::spawn_auto_attacks, move_attacks::move_attacks, decriment_cooldowns::decriment_cooldowns};

pub fn combat(world:&mut World) -> Result<()>{
  //update the target to deselect the target when click to move
  update_target(world)?;
  spawn_auto_attacks(world)?;
  //reduce the cooldown timer
  decriment_cooldowns(world)?;
  move_attacks(world)?;
  Ok(())
}