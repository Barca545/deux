use crate::{component_lib::AutoAttackCooldown, ecs::World, time::ServerTime};
use crate::component_lib::Timer;

///Decrements the cooldowns by the time passed since the last game logic tick.
/// Sets the cooldown == 0.0 if the value would be negative.
pub fn decrement_cooldowns(world:&World) {
  let server_time = world.immut_get_resource::<ServerTime>().unwrap();
  let time_passed = server_time.get_tick_frequency();
  
  let mut query = world.query();
  let entities = query.with_component::<AutoAttackCooldown>().unwrap().run_entity();
  
  for entity in entities{
    let mut auto_attack_cooldown = entity.mut_get_component::<AutoAttackCooldown>().unwrap();
    auto_attack_cooldown.decrement(time_passed);
    
    if auto_attack_cooldown.remaining() < 0.0 {
      auto_attack_cooldown.zero();
    } 
  }
}