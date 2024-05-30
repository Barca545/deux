use nina::world::World;

use crate::{
  data_lib::{AbilityMap, BecsId},
  event::{GameEvent, GameEventQueue},
  utility::run_scripts
};

//Refactor
// -Add death system. Death system should listen for a "killed" event and update
// KDA, Gold and EXP -Gold should vary based on external factors not be hard
// coded. -EXP should vary depending on entity killed
// -Get rid of Colliding component?
// -This should consume an event like incoming damage or something that holds
// the damage amoun target and owner  If an attack kills should be handled by
// another system. -Need to measure which attack hit first
// -Can't run the end script, what if it hits and then applies an ability?
//  Also can't delete the entity for the same reason.
// -I am not sure a hit should automatically trigger the entity's deletion.

///Processes all `AbilityHit` [`GameEvent`]s. If an entity is killed, creates
/// an `EntityKilled` `GameEvent`.
pub fn ability_hit_resolve(world:&mut World) {
  let mut buffered_scripts = Vec::new();
  {
    //Process AbilityHit events.
    let events = world.get_resource::<GameEventQueue>();
    events.process_events(|event| {
      if let GameEvent::AbilityHit {
        owner,
        ability_slot,
        ability_id
      } = event
      {
        //Get the ability script
        let map = world.get_component::<AbilityMap>(owner.id()).unwrap();
        let ability = map.get(*ability_slot);
        let scripts = ability.scripts.clone();
        if let Some(onhit) = scripts.onhit() {
          //Buffer the scripts to be evaluated
          buffered_scripts.push((owner.id(), *ability_id, onhit.0));
        }
      }
    });
  }
  //Run the onhit field of the script and delete the ability
  for (owner, ability_id, script) in &buffered_scripts {
    run_scripts(world, owner, ability_id, script);
    world.delete_entity(*ability_id).unwrap();
  }
}
