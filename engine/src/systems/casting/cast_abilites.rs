use crate::{
  data_lib::{BecsId, Casting},
  time::Timer,
};
use nina::world::World;

// Refactor:
// - Should checking for the resources occur here or in the scripts

///Queries all entities with a [`Casting`] component.
/// If the ability is ready to cast, cast the ability.
pub fn cast_abilites(world:&mut World,) {
  let mut buffered_scripts = Vec::new();

  let mut query = world.query();
  let entities = query.with_component::<Casting>().unwrap().run();

  for entity in entities {
    //If the channel for a casting ability is finished, cast the ability
    let cast = entity.get_component::<Casting>().unwrap();

    let cooldown = cast.ability.cooldown.clone();

    if cast.is_done() {
      //Check if the ability is off cooldown
      if cooldown.is_zero() {
        //Buffer the start logic for the ability
        let owner = cast.ability.owner;
        let mouse = cast.ability.mouse;
        let target = cast.ability.target;
        let start_script = cast.ability.scripts.start().unwrap();
        buffered_scripts.push((mouse, owner.id(), start_script.0.clone(), cooldown, target,),);
      }
    }
  }

  // Cast the buffered abilities
  for (_mouse, _owner, _script, _cooldown, _target,) in &mut buffered_scripts {
    // Execute the scripts
    // TODO: Move over to new scripting engine.

    // let did_cast = eval_scripts_mouse::<bool,>(world, owner, target, mouse,
    // script,).unwrap();

    // if did_cast {
    //   //Reset the ability's cooldown
    //   cooldown.reset();

    //   //Emit an AbilityCast event
    //   let events = world.get_resource_mut::<GameEventQueue>();
    //   let event = GameEvent::AbilityCast;
    //   events.push(event,);
    // }
  }
}
