use crate::{
  component_lib::AbilityMap,
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::run_scripts,
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
// -I am not sure a hit should automatically trigger the entity's deletion.

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
          buffered_scripts.push((owner.0, *ability_id, onhit.0));
        }
      }
    });
  }
  //Run the onhit field of the script and delete the ability
  for (owner, entity, script) in &buffered_scripts {
    run_scripts(world, owner, script);
    world.delete_entity(*entity).unwrap();
  }
}
