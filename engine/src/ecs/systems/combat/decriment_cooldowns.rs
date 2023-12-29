use crate::{ecs::{World, component_lib::AutoAttackCooldown}, time::ServerTime};
use eyre::Result;

pub fn decriment_cooldowns(world:&World) -> Result<()>{
  let server_time = world.immut_get_resource::<ServerTime>().unwrap();
  let time_passed = server_time.get_tick_frequency();
  
  let mut query = world.query();

  let entities = query.with_component::<AutoAttackCooldown>()?.run_entity();
  for entity in entities{
    let mut auto_attack_cooldown = entity.mut_get_component::<AutoAttackCooldown>()?;
    auto_attack_cooldown.remaining -= time_passed;
    if auto_attack_cooldown.remaining < 0.0 {
      auto_attack_cooldown.remaining = 0.0
    }
    
  }
  Ok(())
}