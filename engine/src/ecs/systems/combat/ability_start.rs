use crate::{
  component_lib::AbilityMap,
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::{has_resource, off_cooldown, run_scripts},
};

pub fn ability_start(world: &mut World) {
  let mut buffered_scripts = Vec::new();
  {
    let events = world.get_resource::<GameEventQueue>().unwrap();
    events.process_events(|event| {
      if let GameEvent::AbilityStart { ability_type, owner } = event {
        //Check if oom
        if has_resource(world, owner.0, 0) {
          //Check if the ability is not on cooldown
          if off_cooldown(world, owner.0, "ability 1") {
            let map = world.get_component::<AbilityMap>(owner.0).unwrap();
            let ability = map.get(*ability_type);
            if let Some(start) = ability.start() {
              //Buffer the scripts to be executed
              buffered_scripts.push((*owner, start.clone()));
              //Reset the cooldowns
              //Set the player state to Casting
            }
          } else {
            dbg!("On CD");
          }
        } else {
          dbg!("OOM");
        }
      }
    });
  }
  for (owner, script) in &buffered_scripts {
    run_scripts(world, owner, script);
  }
}
