use crate::ecs::World;
use eyre::Result;

use super::{update_target::update_target, spawn_auto_attacks::spawn_auto_attacks};

pub fn combat(world:&mut World) -> Result<()>{
  update_target(world)?;
  spawn_auto_attacks(world)?;
  Ok(())
}