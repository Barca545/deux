use crate::ecs::World;
use eyre::Result;

use super::{update_target::update_target, spawn_auto_attacks::spawn_auto_attacks, move_attacks::move_attacks, decriment_cooldowns::decriment_cooldowns, resolve_attacks::resolve_attacks};

pub fn combat(world:&mut World) -> Result<()>{
  //update the target to deselect the target when click to move instead of on hover
  update_target(world)?;
  spawn_auto_attacks(world)?;
  decriment_cooldowns(world)?;
  move_attacks(world)?;
  //need to add the feature to destroy the attack entity after the collision. 
  //currently the damage keeps applying (x3) as the attack passes through
  //destroying the entity will require me updating my index to be generational
  resolve_attacks(world)?;

  //if one hits, deal damage (for now more later) otherwise 
  //check circle-circle collision of the attack and target
  //if colliding check the id of the entity it is hitting
  //if the id matches the target, get the attack damage from the owner and apply the damage to the target hp
  //need an owner commponent, attack damage, and missle radius65
  
  Ok(())
}