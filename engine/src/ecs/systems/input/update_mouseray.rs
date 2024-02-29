use crate::{
  ecs::{world_resources::ScreenDimensions, World},
  math::{MouseRay, Transforms},
};

// Refactor:
// -Should be using the input struct to get the mouse coordinates instead of taking them in as parameters

pub fn update_mouseray(world: &World, x: f64, y: f64) -> MouseRay {
  let screen_dimensions = world.get_resource::<ScreenDimensions>().unwrap().clone();
  let transforms = world.get_resource::<Transforms>().unwrap();
  let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);
  mouse_ray
}
