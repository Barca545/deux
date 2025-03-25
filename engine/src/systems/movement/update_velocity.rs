use nina::world::World;

use crate::{
  data_lib::{Controllable, UnitSpeed, Velocity},
  event::{GameEvent, GameEventQueue},
};
// TODO: Eventually this will handle all entities with velocity and the function
// docs will need updating

/// Updates the [`Velocity`] component of the [`Controllable`] character.
pub fn update_velocity(world:&World,) {
  // Get the player
  let mut query = world.query();
  let player = &query.with_component::<Controllable>().unwrap().run()[0];

  // Get player movement stats
  let velocity = world.get_component_mut::<Velocity>(player.id,).unwrap();
  let speed = world.get_component::<UnitSpeed>(player.id,).unwrap();

  // Process movement events
  let events = world.get_resource::<GameEventQueue>();
  events.process_events(|event| {
    // Update the players velocity based on the event
    match event {
      GameEvent::StartUp => velocity.0.z = (velocity.0.z + 1.0).clamp(-1.0, 1.0,),
      GameEvent::StartDown => velocity.0.z = (velocity.0.z - 1.0).clamp(-1.0, 1.0,),
      GameEvent::StartLeft => velocity.0.x = (velocity.0.x + 1.0).clamp(-1.0, 1.0,),
      GameEvent::StartRight => velocity.0.x = (velocity.0.x - 1.0).clamp(1.0, 1.0,),
      _ => {}
    }
  },);

  // Normalize + scale velocity to ensure it always has a |v| = speed
  // *velocity = Velocity(velocity.0.normalize().scale(speed.total(),),);
}

#[cfg(test)]
mod test {
  use crate::{data_lib::Velocity, math::Vec3};

  #[test]
  // Confirms the math occuring in update velocity is correct
  fn update_velocity_yields_correct_velocity() {
    // A player's velocity, starts at 0, 0, 0
    let mut velocity = Velocity::default();
    let speed = 5.0;

    // 2 Up commands are registered
    velocity.0.z = (velocity.0.z + 1.0).clamp(-1.0, 1.0,);
    velocity.0.z = (velocity.0.z + 1.0).clamp(-1.0, 1.0,);

    // 1 Right command is registered
    velocity.0.x = (velocity.0.x + 1.0).clamp(-1.0, 1.0,);

    // Confirm the intermediate value is as expected
    assert_eq!(velocity.0, Vec3::new(1.0, 0.0, 1.0));

    // Normalize the velocity then scale by speed to get one with the correct
    // magnitude
    velocity = Velocity(velocity.0.normalize().scale(speed,),);

    // Confirm the magnitude came out approximately correctly
    assert_eq!(5.0, (velocity.0.x.powi(2) + velocity.0.x.powi(2)).sqrt());
    // Confirm both components of motion are equal
    assert_eq!(velocity.0.x, velocity.0.z);
  }
}
