use crate::ecs::World;
use eyre::Result;

use super::{update_target::update_target, spawn_auto_attacks::spawn_auto_attacks, move_attacks::move_attacks, decriment_cooldowns::decriment_cooldowns};

pub fn combat(world:&mut World) -> Result<()>{
  //update the target to deselect the target when click to move instead of on hover
  update_target(world)?;
  spawn_auto_attacks(world)?;
  decriment_cooldowns(world)?;
  move_attacks(world)?;
  //check collisions and handle the result
  //if one hits, deal damage (for now more later) otherwise 
  Ok(())
}