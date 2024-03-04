use crate::{
  component_lib::{AbilityMap, AutoAttack, Colliding, Exp, Gold, Killed, Owner, Target, KDA},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::eval_scripts,
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

pub fn resolve_ability_hit(world: &mut World) {
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
          dbg!(onhit.clone());
          //Buffer the scripts to be evaluated
          buffered_scripts.push((*owner, *ability_id, onhit));
        }
      }
    });
  }
  //Evaluate the stop script to get the damage of the ability.
  for (owner, id, script) in &buffered_scripts {
    if let Some(damage) = eval_scripts::<u32>(world, owner, script) {
      dbg!(damage);
      //Pass the damage into the damage calculation
      //If the ability kills the target create a killed event
    }
    //Delete the ability
  }
}

///Queries all entities with `AutoAttack` and `Colliding` components.
/// If the auto attack killed, award its owner kill gold and increment their KDA.
/// Delete all collided auto attack entities.
pub fn resolve_attacks(world: &mut World) {
  //buffer holding the id of the attacks that hit and need to be removed at the end of this function
  let mut attacks_to_delete = Vec::new();

  //Fetch all entities with the AutoAttack and Colliding components
  let mut query = world.query();
  let entities = query.with_component::<AutoAttack>().unwrap().with_component::<Colliding>().unwrap().run();

  //Check the result of attacks landing.
  //Award gold and update KDAs
  for entity in entities {
    //Get a boolean balue indicating if the attack killed.
    let owner = entity.get_component::<Owner>().unwrap();
    let killed = world.get_component::<Killed>(owner.0).is_ok();

    if killed {
      //Get the target component
      let target = entity.get_component::<Target>().unwrap();

      //Update the owner's gold
      let mut owner_gold = world.get_component_mut::<Gold>(owner.0).unwrap();
      owner_gold.0 += 350;

      //Update the KDAs
      let mut owner_kda = world.get_component_mut::<KDA>(owner.0).unwrap();
      let mut target_kda = world.get_component_mut::<KDA>(target.0.unwrap()).unwrap();
      owner_kda.kill(1);
      target_kda.death(1);

      //Update the exp
      let mut owner_exp = world.get_component_mut::<Exp>(owner.0).unwrap();
      owner_exp.0 += 100;

      //Set the target entity's state to dead and start the death timer
    }
    //Add the entity id to the buffer to be deleted
    attacks_to_delete.push(entity.id);
  }

  //Delete all the attacks that hit
  for id in attacks_to_delete {
    world.delete_entity(id).unwrap();
  }
}

pub fn process_hits(world: &mut World) {
  let mut attacks_to_clear = Vec::default();
  {
    let events = world.get_resource::<GameEventQueue>().unwrap();
    events.process_events(|event| match event {
      GameEvent::AutoAttackHit { attack_id, owner } => {
        resolve_hit(world, attack_id, owner);
        attacks_to_clear.push(*attack_id);
      }
      _ => {}
    });
  }
  for attack in attacks_to_clear {
    world.delete_entity(attack).unwrap();
  }
}

fn resolve_hit(world: &World, attack_id: &usize, owner: &Owner) {
  // This basically just calls the script logic
  //unsure if I want to also kill the entity and award gold here or if there is some reason to handle it somewhere else.
  // I think handling it in another system that notifies every player who got a kill etc is useful
}
