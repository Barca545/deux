use crate::{
  component_lib::{AbilityMap, Armor, Dead, Health, Target},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::{calc_post_mitigation_damage, eval_scripts},
};

use std::cmp::max;

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
      if let GameEvent::AbilityHit {
        ability_type,
        ability_id,
        owner,
      } = event
      {
        //Get the ability script
        let map = world.get_component::<AbilityMap>(owner.0).unwrap();
        let ability = map.get(*ability_type);
        if let Some(onhit) = ability.onhit() {
          //Buffer the scripts to be evaluated
          buffered_scripts.push((owner.0, *ability_type, *ability_id, onhit));
        }
      }
    });
  }
  //Evaluate the stop script to get the damage of the ability.
  for (owner, ability_type, entity, script) in &buffered_scripts {
    let entity = entity;
    if let Some(damage) = eval_scripts::<i32>(world, ability_type, entity, owner, script) {
      //Get the target's information
      let target = world.get_component::<Target>(*entity).unwrap().0.unwrap();
      let remaining_health;
      {
        let resist = world.get_component::<Armor>(target).unwrap();
        //Pass the damage and resist into the damage calculation
        let damage = calc_post_mitigation_damage(damage, resist.0);
        //Deal damage
        let mut target_health = world.get_component_mut::<Health>(target).unwrap();
        target_health.remaining = max(0, target_health.remaining - damage);
        remaining_health = target_health.remaining;
      }

      //If the ability kills the target create a killed event
      if remaining_health == 0 {
        //Give the target the Dead marker
        world.add_component(target, Dead).unwrap();
        //Create a target killed event
        let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
        events.push(GameEvent::EntityKilled {
          entity: target,
          killer: *entity,
        })
      }
    }
    //Delete the ability
    world.delete_entity(*entity).unwrap();
  }
}
