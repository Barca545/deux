use crate::{ecs::{World, component_lib::{AutoAttack, Target, GameplayRadius, Position, Health, Owner, AttackDamage}}, physics::circle_point_collision_test};
use eyre::Result;

pub fn resolve_attacks(world:&mut World) -> Result<()>{
 let mut query = world.query();

 let entities = query
  .with_component::<AutoAttack>()?.run_entity();

  //Loop through the auto attacks to see if they are colliding with their target
  for entity in entities {
    let target = entity.immut_get_component::<Target>()?;
    let owner = entity.immut_get_component::<Owner>()?;  
    let attack_position = entity.immut_get_component::<Position>()?;
    let attack_damage = world.immut_get_component_by_entity_id::<AttackDamage>(owner.id)?;

    //Because target holds an option this has to check. 
    //Should always return true because an auto_attack only spawns if the owner's Target component has an id.
    if let Some(id) = target.0 {
      //get the target radius and position
      let target_position = world.immut_get_component_by_entity_id::<Position>(id)?;
      let target_radius = world.immut_get_component_by_entity_id::<GameplayRadius>(id)?;

      //Check if the attack is colliding with the target using a circle-point test
      let collision_check = circle_point_collision_test(attack_position.tick_end, target_position.tick_end, target_radius.0);
      
      //If the attack and the target are colliding, apply damage
      if collision_check {
        let mut target_health = world.mut_get_component_by_entity_id::<Health>(id)?;
        target_health.remaining -= attack_damage.0;
      }
    }
  }
  Ok(())
}