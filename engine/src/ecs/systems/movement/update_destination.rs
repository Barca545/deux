use crate::{ecs::{World, world_resources::ScreenDimensions, component_lib::{Controllable, Destination}}, math::{Transforms, MouseRay, Vec3}};
use eyre::Result;

//this shold only update the destination not set velocity

pub fn update_destination(world:&mut World, x:f64, y:f64) -> Result<()> {
  let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  //mouse ray should be a resource that is updated when the mouse moves
  //arguably the mouse ray is information both the selection and this system
  // needs selection needs to run first and do the AABB test
  //this should only run if the selection test says nothing is selected
  // let mouse_ray = world.immut_get_resource::<MouseRay>().unwrap();
  let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);
  let intersection:Vec3 = mouse_ray.0.ray_ground_intersection();

  //this gets the index of the component and updates its destination
  //this is necesary for adding a commponent

  let mut query = world.query();
  let entities = query
    .with_component::<Controllable>()?
    .with_component::<Destination>()?
    // .with_component::<Position>()?
    // .with_component::<Speed>()?
    // .with_component::<Velocity>()?
    .run_entity();

  for entity in entities {
    let mut destination = entity.mut_get_component::<Destination>()?;
    // let position = entity.mut_get_component::<Position>()?;
    // let speed = entity.immut_get_component::<Speed>()?;
    // let mut velocity = entity.mut_get_component::<Velocity>()?;

    *destination = Destination(intersection);
    // *velocity = Velocity::new(&position, &destination, &speed);
  }
  Ok(())
}