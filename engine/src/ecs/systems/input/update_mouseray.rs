use crate::{ecs::{world_resources::ScreenDimensions, World}, math::{MouseRay, Transforms}};

// Refactor: 
// -Should be using the input struct to get the mouse coordinates instead of taking them in as parameters
// -Update selection should feed into this

// pub fn update_mouseray(world:&mut World, x:f64,y:f64){
//   let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap().clone();
//   let transforms = world.immut_get_resource::<Transforms>().unwrap();
//   let new_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);
//   let mouse_ray = world.mut_get_resource::<MouseRay>().unwrap();
//   *mouse_ray = new_ray
// }

pub fn update_mouseray(world:&World, x:f64,y:f64)->MouseRay{
  let screen_dimensions = world.get_resource::<ScreenDimensions>().unwrap().clone();
  let transforms = world.get_resource::<Transforms>().unwrap();
  let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);
  mouse_ray
}