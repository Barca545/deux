use game_data::{player_movement::PlayerMovement, Controllable, UnitSpeed, Velocity};
use nina::world::World;
// TODO: Eventually this will handle all entities with velocity and the function
// docs will need updating

/// Updates the [`Velocity`] component of the [`Controllable`] character.
pub fn update_velocity(world: &World,) {
  // Get the player
  let mut query = world.query();
  let player = &query.with_component::<Controllable>().unwrap().run()[0];

  // Get player movement stats
  let velocity = world.get_component_mut::<Velocity>(player.id,).unwrap();
  let speed = world.get_component::<UnitSpeed>(player.id,).unwrap();
  let player_input_state = world.get_resource_mut::<PlayerMovement>();

  // If there is a horizontal input add it to the player's velocity.
  match (player_input_state.left, player_input_state.right,) {
    // Both buttons pressed cancel each other out
    (true, true,) => velocity.0.x = 0.0,
    // Only left pressed means go left
    (true, false,) => velocity.0.x = 1.0,
    // Only right pressed means go right
    (false, true,) => velocity.0.x = -1.0,
    // Nothing to do if nothing is pressed :P
    (false, false,) => velocity.0.x = 0.0,
  }

  // If there is a vertical input add it to the player's velocity.
  match (player_input_state.up, player_input_state.down,) {
    // Both buttons pressed cancel each other out
    (true, true,) => velocity.0.z = 0.0,
    // Only up pressed means go up
    (true, false,) => velocity.0.z = 1.0,
    // Only down pressed means go down
    (false, true,) => velocity.0.z = -1.0,
    // Nothing to do if nothing is pressed :P
    (false, false,) => velocity.0.z = 0.0,
  }

  // Normalize + scale velocity to ensure it always has a |v| = speed
  if velocity.mag() > speed.max() {
    *velocity = Velocity(velocity.0.normalize().scale(speed.max(),),);
  }
}

#[cfg(test)]
mod test {
  use game_data::Velocity;
  use math::Vec3;

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
