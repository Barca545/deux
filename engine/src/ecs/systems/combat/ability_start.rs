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
  let mut buffered_scripts = Vec::new();
  {
    let events = world.get_resource::<GameEventQueue>().unwrap();
    events.process_events(|event| {
      if let GameEvent::AbilityStart { mouseray, ability_type, owner } = event {
        //Check cooldown, other casting checks are in the script but this is constant for all abilities
        if off_cooldown(world, owner.0, String::from("auto attack")) {
          let map = world.get_component::<AbilityMap>(owner.0).unwrap();
          let ability = map.get(*ability_type);
          if let Some(start) = ability.start() {
            //Buffer the scripts to be executed
            buffered_scripts.push((*mouseray, owner.0, *ability_type, start.clone()));
          }
        }
      }
    });
  }

  for (mouse, owner, ability_type, script) in &buffered_scripts {
    let owner = owner;
    let cast = eval_scripts_mouse::<bool>(world, owner, owner, mouse, script).unwrap();
    if cast {
      //Reset the cooldowns
      let mut cooldowns = world.get_component_mut::<Cooldowns>(*owner).unwrap();
      cooldowns.reset("auto attack");

      //Set the player state to Casting and
      let mut player_state = world.get_component_mut::<PlayerState>(*owner).unwrap();
      *player_state = PlayerState::Unoccupied;
    }
  }
}
