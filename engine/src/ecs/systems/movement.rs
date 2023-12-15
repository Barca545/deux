use crate::{
  ecs::{
    component_lib::{Controllable, Destination, Position, Speed, Velocity},
    world_resources::ScreenDimensions,
    World
  },
  math::{math::Vec3, MouseRay, Transforms}
};
use eyre::Result;

//the below should be incorporated into the system's description
//mouse ray should be a resource that is updated when the mouse moves
//arguably the mouse ray is information both the selection and this system
// needs selection needs to run first and do the AABB test
//this should only run if the selection test says nothing is selected
///Updates the destination and direction of all controllable entities.   
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
    .with_component::<Position>()?
    .with_component::<Destination>()?
    .with_component::<Speed>()?
    .with_component::<Velocity>()?
    .run_entity();

  for entity in entities {
    let mut destination = entity.mut_get_component::<Destination>()?;
    let position = entity.mut_get_component::<Position>()?;
    let speed = entity.immut_get_component::<Speed>()?;
    let mut velocity = entity.mut_get_component::<Velocity>()?;

    *destination = Destination(intersection);
    *velocity = Velocity::new(&position, &destination, &speed);
  }
  Ok(())
}

//maybe call this system like resolve movement
///Moves all units towards their destination by adding their velocity to their
/// position. Updates an entity's `Position` component.
pub fn resolve_movement(world:&World) -> Result<()> {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>()?
    .with_component::<Destination>()?
    .with_component::<Speed>()?
    .with_component::<Velocity>()?
    .run_entity();

  for entity in entities {
    let mut position = entity.mut_get_component::<Position>()?;
    let destination = entity.immut_get_component::<Destination>()?;

    if position.tick_end != destination.0 {
      let velocity = entity.immut_get_component::<Velocity>()?;

      let tick_start:Vec3 = position.tick_end;
      let tick_end:Vec3 = position.tick_end + velocity.0;

      let new_position = Position::new(tick_start, tick_end);

      let d1 = (new_position.tick_end.x - position.tick_end.x).powi(2) + (new_position.tick_end.z - position.tick_end.z).powi(2);
      let d2 = (destination.0.x - position.tick_end.x).powi(2) + (destination.0.z - position.tick_end.z).powi(2);

      if d1 < d2 {
        *position = new_position;
      } else {
        *position = Position::new(tick_start, destination.0.clone());
      }
    }
  }
  Ok(())
}

//I think I also need a system that updates the hitbox 
//just query all entities with Position & Hitbox then set the 



#[cfg(test)]
mod test {
  use super::resolve_movement;
  use crate::ecs::{
    component_lib::{Destination, Position, Speed, Velocity},
    World
  };
  use eyre::Result;
  use glm::vec3;

  #[test]
  fn get_direction() {
    let position = Position::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0));
    let destination = Destination::new(3.0, 4.0, 0.0);
    let speed = Speed(1.0);

    let velocity = Velocity::new(&position, &destination, &speed);
    dbg!(velocity.0);

    let new_destination = Destination::new(3.0, 3.0, 0.0);
    let velocity = Velocity::new(&position, &new_destination, &speed);

    dbg!(velocity.0);
  }

  #[test]
  fn update_position() -> Result<()> {
    let position = Position::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0));
    let destination = Destination::new(3.0, 0.0, 3.0);
    let speed = Speed(5.0);
    let velocity = Velocity::new(&position, &destination, &speed);
    dbg!(velocity);

    let mut world = World::new();

    world
      .register_component::<Position>()
      .register_component::<Destination>()
      .register_component::<Speed>()
      .register_component::<Velocity>();

    //entity the system should target
    world
      .create_entity()
      .with_component(position)?
      .with_component(destination)?
      .with_component(speed)?
      .with_component(velocity)?;

    //entity the system should ignore
    world
      .create_entity()
      .with_component(position)?
      .with_component(speed)?
      .with_component(velocity)?;

    resolve_movement(&world)?;

    //Confirm the update occured
    let mut query = world.query();

    let entities = query.with_component::<Position>()?.run_entity();

    for entity in entities {
      let updated_position = entity.immut_get_component::<Position>()?;
      dbg!(updated_position.tick_end);
    }
    Ok(())
  }
}
