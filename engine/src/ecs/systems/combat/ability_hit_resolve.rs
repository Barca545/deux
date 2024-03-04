use crate::{
  component_lib::{AbilityMap, Armor, Dead, Health, Target},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::{calc_post_mitigation_damage, eval_scripts},
};

//Refactor
// -Add death system. Death system should listen for a "killed" event and update KDA, Gold and EXP
// -Gold should vary based on external factors not be hard coded.
// -EXP should vary depending on entity killed
// -Get rid of Colliding component?
// -This should consume an event like incoming damage or something that holds the damage amoun target and owner
//  If an attack kills should be handled by another system.
// -Need to measure which attack hit first
// -Can't run the end script, what if it hits and then applies an ability?
//  Also can't delete the entity for the same reason.

///Processes all `AbilityHit` [`GameEvent`]s. If an entity is killed, creates an `EntityKilled` `GameEvent`.
pub fn ability_hit_resolve(world: &mut World) {
  let mut buffered_scripts = Vec::new();
  {
    //Process AbilityHit events.
    let events = world.get_resource::<GameEventQueue>().unwrap();
    events.process_events(|event| {
      if let GameEvent::AbilityHit { ability_type, ability_id, owner } = event {
        //Get the ability script
        let map = world.get_component::<AbilityMap>(owner.0).unwrap();
        let ability = map.get(*ability_type);
        if let Some(onhit) = ability.onhit() {
          //Buffer the scripts to be evaluated
          buffered_scripts.push((owner.0, *ability_id, onhit));
        }
      }
    });
  }
  //Evaluate the stop script to get the damage of the ability.
  for (owner, entity, script) in &buffered_scripts {
    let entity = entity;
    if let Some(damage) = eval_scripts::<u32>(world, entity, owner, script) {
      //Get the target's information
      let target = world.get_component::<Target>(*entity).unwrap().0.unwrap();
      let remaining_health;
      {
        let resist = world.get_component::<Armor>(target).unwrap();
        //Pass the damage and resist into the damage calculation
        let damage = calc_post_mitigation_damage(damage, resist.0);

        //Deal damage
        let mut target_health = world.get_component_mut::<Health>(target).unwrap();
        target_health.remaining -= damage;
        remaining_health = target_health.remaining;
      }

      //If the ability kills the target create a killed event
      if remaining_health == 0 {
        //Give the target the Dead marker
        world.add_component(target, Dead).unwrap();
        //Create a target killed event
        let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
        events.push(GameEvent::EntityKilled { entity: target, killer: *entity })
      }
    }
    //Delete the ability
    world.delete_entity(*entity).unwrap();
  }
}

// ///Queries all entities with `AutoAttack` and `Colliding` components.
// /// If the auto attack killed, award its owner kill gold and increment their KDA.
// /// Delete all collided auto attack entities.
// pub fn resolve_attacks(world: &mut World) {
//   //buffer holding the id of the attacks that hit and need to be removed at the end of this function
//   let mut attacks_to_delete = Vec::new();

//   //Fetch all entities with the AutoAttack and Colliding components
//   let mut query = world.query();
//   let entities = query.with_component::<AutoAttack>().unwrap().with_component::<Colliding>().unwrap().run();

//   //Check the result of attacks landing.
//   //Award gold and update KDAs
//   for entity in entities {
//     //Get a boolean balue indicating if the attack killed.
//     let owner = entity.get_component::<Owner>().unwrap();
//     let killed = world.get_component::<Killed>(owner.0).is_ok();

//     if killed {
//       //Get the target component
//       let target = entity.get_component::<Target>().unwrap();

//       //Update the owner's gold
//       let mut owner_gold = world.get_component_mut::<Gold>(owner.0).unwrap();
//       owner_gold.0 += 350;

//       //Update the KDAs
//       let mut owner_kda = world.get_component_mut::<KDA>(owner.0).unwrap();
//       let mut target_kda = world.get_component_mut::<KDA>(target.0.unwrap()).unwrap();
//       owner_kda.kill(1);
//       target_kda.death(1);

//       //Update the exp
//       let mut owner_exp = world.get_component_mut::<Exp>(owner.0).unwrap();
//       owner_exp.0 += 100;

//       //Set the target entity's state to dead and start the death timer
//     }
//     //Add the entity id to the buffer to be deleted
//     attacks_to_delete.push(entity.id);
//   }

//   //Delete all the attacks that hit
//   for id in attacks_to_delete {
//     world.delete_entity(id).unwrap();
//   }
// }
