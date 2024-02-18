use crate::{component_lib::{Controllable, Destination}, ecs::World, math::{MouseRay, Vec3}};

//Refactor
// -MouseRay should be a resource that systems can query instead of each system building its own
// -This should only run if the selection test says nothing is selected
//  needs selection needs to run first and do the AABB test
// -This should require a click

///Updates the [`Destination`] component for the entity marked with the [`Controllable`] component. 
/// Does not update entity's `Velocity` components.
pub fn update_destination(world:&mut World) { 
  let mouse_ray = world.immut_get_resource::<MouseRay>().unwrap();
  let intersection:Vec3 = mouse_ray.0.ray_ground_intersection();

  let mut query = world.query();
  let entities = query
    .with_component::<Controllable>().unwrap()
    .with_component::<Destination>().unwrap()
    .run();

  for entity in entities {
    //Update the destination to match the location the cursor has indicated
    let mut destination = entity.mut_get_component::<Destination>().unwrap();
    *destination = Destination(intersection);
  }
}