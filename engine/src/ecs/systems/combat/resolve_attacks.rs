use crate::{component_lib::{Killed, AutoAttack, Colliding, Exp, Gold, Owner, Target, KDA}, ecs::World};

//Refactor
// -Add death system
// -Gold should vary based on external factors not be hard coded.
// -EXP should vary depending on entity killed

///Queries all entities with `AutoAttack` and `Colliding` components. 
/// If the auto attack killed, award its owner kill gold and increment their KDA.
/// Delete all collided auto attack entities.
pub fn resolve_attacks(world:&mut World) {
 //buffer holding the id of the attacks that hit and need to be removed at the end of this function
 let mut attacks_to_delete = Vec::new();
 
 //Fetch all entities with the AutoAttack and Colliding components
 let mut query = world.query();
 let entities = query
  .with_component::<AutoAttack>().unwrap()
  .with_component::<Colliding>().unwrap()
  .run
();

  //Check the result of attacks landing.
  //Award gold and update KDAs 
  for entity in entities {
    //Get a boolean balue indicating if the attack killed.
    let owner = entity.immut_get_component::<Owner>().unwrap();  
    let killed = world.immut_get_component_by_entity_id::<Killed>(owner.0).is_ok();
    
    if killed {
      //Get the target component
      let target = entity.immut_get_component::<Target>().unwrap();

      //Update the owner's gold 
      let mut owner_gold = world.mut_get_component_by_entity_id::<Gold>(owner.0).unwrap();
      owner_gold.0 += 350;

      //Update the KDAs
      let mut owner_kda = world.mut_get_component_by_entity_id::<KDA>(owner.0).unwrap();
      let mut target_kda = world.mut_get_component_by_entity_id::<KDA>(target.0.unwrap()).unwrap();
      owner_kda.kill(1);
      target_kda.death(1);

      //Update the exp
      let mut owner_exp = world.mut_get_component_by_entity_id::<Exp>(owner.0).unwrap();
      owner_exp.0 += 100;

      //Set the target entity's state to dead and start the death timer
    }
    //Add the entity id to the buffer to be deleted
    attacks_to_delete.push(entity.id); 
  }

  //Delete all the attacks that hit
  for id in attacks_to_delete{
    world.delete_entity(id).unwrap();
  }
}