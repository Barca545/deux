use crate::{
  component_lib::{AbilityMap, Cooldowns, PlayerState},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::{eval_scripts_mouse, off_cooldown},
};

// Refactor:
// -Need to move the start logic from the auto_attack_start function into the spawn logic on world's lua implementation
// -This system should just start the channel/wind up.
//  Actually starting the attack should be a different system.
// -Add logic to check targets to lua World Implementation.
// -I can use the lua.eval thing to ask for a boolean indicating if the cast attempt was successful or not
// -Cooldown reset needs to be updated to take in the actual cooldown type not just "auto attack"
// -Playerstate update needs to be changed to Casting
// -Mouseray needs to be passed into lua so the scripts can use it if needed

///System for beginning auto attacks.
/// Queries the [`GameEventQueue`] for `AbilityStart` events.
/// For each `AbilityStart`, this system calls the any scripts associated with the [`Owner`]'s ability start up process.
pub fn ability_start(world: &mut World) {
  //Get the event
  let events = world.get_resource::<GameEventQueue>().unwrap();
  events.process_events(|event| {
    if let GameEvent::BufferAbility { owner, ability_type, mouseray } = event {
      //Get the Cooldown
    }
  });
}
