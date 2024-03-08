use crate::{
  component_lib::{Destination, Position, Velocity},
  ecs::World,
};

///Moves a target away from the the position of the collision.
/// Positive numbers push the target away from the owner.
/// Negative numbers pull the target towards the owner.
pub fn displacement(world: &World, owner: usize, target: usize, speed: f32, distance: f32) {
  //Get the displacement's direction
  let owner_pos = world.get_component::<Position>(owner).unwrap();
  let target_pos = world.get_component::<Position>(target).unwrap();
  let knockback_dir = (target_pos.0 - owner_pos.0).normalize();

  //Update the target's destination and velocity
  let mut target_des = world.get_component_mut::<Destination>(target).unwrap();
  let mut target_vel = world.get_component_mut::<Velocity>(target).unwrap();
  *target_des = Destination::from(target_pos.0 + knockback_dir * distance);
  *target_vel = Velocity::from(knockback_dir.scale(speed))
}
