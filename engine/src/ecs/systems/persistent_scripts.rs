use crate::{
  component_lib::{BecsId, Cooldown, Owner, PersistentScript, RunningScript},
  ecs::World,
  time::Timer,
  utility::run_scripts,
};

// Refactor:
// -I will have to handle the damage in the scripts after all, there's just to much random logic I'd need them to implement

///Query all [`PersistentScript`] entities and run their scripts.
/// If an entity has timed out, delete it.
pub fn execute_scripts(world: &mut World) {
  let mut buffered_scripts = Vec::default();
  let mut timed_out_scripts = Vec::default();

  let mut query = world.query();
  let entities = query.with_component::<PersistentScript>().unwrap().run();
  for entity in entities {
    //If a script has timed out, buffer an entity for deletion.
    let timer = entity.get_component::<Cooldown>().unwrap();
    if timer.is_zero() {
      let owner = entity.get_component::<Owner>().unwrap();
      let script = entity.get_component::<RunningScript>().unwrap();
      if let Some(script) = script.stop() {
        timed_out_scripts.push((owner.id(), entity.id, script));
      }
    }
    //Otherwise buffer the script for execution
    else {
      let owner = entity.get_component::<Owner>().unwrap();
      let script = entity.get_component::<RunningScript>().unwrap();
      if let Some(script) = script.running() {
        buffered_scripts.push((owner.id(), entity.id, script));
      }
    }
  }

  //Delete the timed out scripts and run their stop logic
  for (owner, entity, stop) in timed_out_scripts {
    run_scripts(world, &owner, &stop.0);
    world.delete_entity(entity).unwrap();
  }

  //Execute scripts
  for (owner, entity, running) in buffered_scripts {
    run_scripts(world, &owner, &running.0)
  }
}
