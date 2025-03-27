use crate::{
  data_lib::{Controllable, Position},
  view::camera::Camera,
};
use nina::world::World;

// TODO: Does the camera a position need to be interpolated like the player
// positon

/// Move the [`Camera`] so it is centered on the player.
pub fn update_camera_position(world:&mut World,) {
  // Get the camera
  let camera = world.get_resource_mut::<Camera>();

  // Get the player position
  let mut query = world.query();
  let player = &query.with_component::<Controllable>().unwrap().run()[0];
  let position = player.get_component::<Position>().unwrap();

  // Use the player x/z coors as the new target of the camera

  // TODO: This messes up the camera's position placing it directly above the
  // player but otherwise works Might be an issue with the camera_up field
  camera.offset_camera_relative_to_position(position.0,);
}
