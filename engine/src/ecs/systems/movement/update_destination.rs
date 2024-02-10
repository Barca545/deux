use eyre::Result;
use crate::{component_lib::{Controllable, Destination}, ecs::{
  world_resources::ScreenDimensions, World
}, math::{MouseRay, Transforms, Vec3}};

//Refactor
// -MouseRay should be a resource that systems can query instead of each system building its own

///Updates the `Destination` component for the entity marked with the `Controllable` component. 
/// Does not update velocities.
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

  let mut query = world.query();
  let entities = query
    .with_component::<Controllable>()?
    .with_component::<Destination>()?
    .run_entity();

  for entity in entities {
    //Update the destination to match the location the cursor has indicated
    let mut destination = entity.mut_get_component::<Destination>()?;
    *destination = Destination(intersection);
  }
  Ok(())
}