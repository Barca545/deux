use crate::{
  component_lib::{BaseScript, Cooldown, Owner, PersistentScript, RunningScript},
  ecs::World,
  time::ServerTime,
};

// Refactor:
// -If let for the stop branch might be unnecessary because I am pretty sure all scripts will have stop logic

///Creates a [`PersistentScript`] entity containing a reference to the `running` script it executes and a [`Cooldown`] duration in seconds indicating how long it lasts.
pub fn create_persistent_script(world: &mut World, owner: usize, running: Option<BaseScript>, stop: Option<BaseScript>, duration: f64) {
  let cooldown;
  {
    let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
    cooldown = Cooldown::new(&mut server_time, duration);
  }

  world
    .create_entity()
    .with_component(PersistentScript)
    .unwrap()
    .with_component(Owner::new(owner))
    .unwrap()
    .with_component(RunningScript::new(running, stop))
    .unwrap()
    .with_component(cooldown)
    .unwrap();
}
