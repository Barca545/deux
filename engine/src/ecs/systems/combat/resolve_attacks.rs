use crate::{ecs::{World, component_lib::{AutoAttack, Target, GameplayRadius, Position, Health, Owner, AttackDamage, Gold, KDA, Exp}}, physics::circle_point_collision_test};
use eyre::Result;

pub fn resolve_attacks(world:&mut World) -> Result<()>{
 //vector holding the id of the attacks that hit and need to be removed at the end of this function
 let mut attacks_to_delete = Vec::new();
 
 let mut query = world.query();

 let entities = query
  .with_component::<AutoAttack>()?.run_entity();

  //Loop through the auto attacks to see if they are colliding with their target
  for entity in entities {
    let target = entity.immut_get_component::<Target>()?;
    let owner = entity.immut_get_component::<Owner>()?;  
    let attack_position = entity.immut_get_component::<Position>()?;
    let attack_damage = world.immut_get_component_by_entity_id::<AttackDamage>(owner.0)?;

    //Because target holds an option this has to check. 
    //Should always return true because an auto_attack only spawns if the owner's Target component has an id.
    //follow rustdev suggestion and just mark it as collided
    //why does target hold an option? remove that. untargeted abilities just won't have a target component
    //get the target radius and position
    let target_position = world.immut_get_component_by_entity_id::<Position>(target.0)?;
    let target_radius = world.immut_get_component_by_entity_id::<GameplayRadius>(target.0)?;

    //Check if the attack is colliding with the target using a circle-point test
    let collision_check = circle_point_collision_test(attack_position.tick_end, target_position.tick_end, target_radius.0);
      
    //If the attack and the target are colliding, apply damage
    if collision_check {
      //if the attack hit, add it for deletion at the end of the function
      //deletion will need to go in the part where the scripts are resolved
      attacks_to_delete.push(entity.id);
      
      let mut target_health = world.mut_get_component_by_entity_id::<Health>(target.0)?;
      target_health.remaining -= attack_damage.0;

      if target_health.remaining < 0 {
        //give gold to the attack owner
        let mut owner_gold = world.mut_get_component_by_entity_id::<Gold>(owner.0)?;
        owner_gold.0 += 350;
        
        //update the kdas
        let mut owner_kda = world.mut_get_component_by_entity_id::<KDA>(owner.0)?;
        let mut target_kda = world.mut_get_component_by_entity_id::<KDA>(target.0)?;
        
        owner_kda.kill(1);
        target_kda.death(1);

        //Update the exp
        //100 is not necesarily what I will settle on
        let mut owner_exp = world.mut_get_component_by_entity_id::<Exp>(owner.0)?;
        owner_exp.0 += 100;
        
        //set the entity state to dead and set the death timer
      }
    }
  }

  //delete all the attacks that hit
  for id in attacks_to_delete{
    world.delete_entity(id)?;
  }
  Ok(())
}