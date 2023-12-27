use crate::{ecs::{World, component_lib::{Position, Velocity, Destination,PathingRadius}}, math::Vec3, physics::circle_circle_collision_test};
use eyre::Result;

pub fn update_position(world:&World) -> Result<()> {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>()?
    .with_component::<Velocity>()?
    .with_component::<Destination>()?
    .run_entity();


  for entity in entities {
    let mut position = entity.mut_get_component::<Position>()?;
    let velocity = entity.immut_get_component::<Velocity>()?;
    let destination = entity.immut_get_component::<Destination>()?;
    let pathing_radius = entity.immut_get_component::<PathingRadius>()?;
    let id = entity.id;

    if position.tick_end != destination.0 {
      let new_position = calculate_new_position(*position, *velocity, *destination);
      let mut collision_check = false;

      //practically speaking it might make sense to have a resource that holds copies of all the entities and their collision logic
      //I think that might be what an acceleratin structure is
      
      let mut query2 = world.query();

      let test_entities = query2
        .with_component::<Position>()?
        .with_component::<Velocity>()?
        .with_component::<Destination>()?
        .run_entity();
  
      for test_entity in test_entities{
        let test_entity_id = test_entity.id;

        if test_entity_id != id {
          collision_check = collision_test(world, new_position, pathing_radius.0, test_entity_id)?;
        }
      }

      if !collision_check {
        *position = new_position;
      }
    }
  }

  Ok(())
}


fn calculate_new_position(position:Position, velocity:Velocity, destination:Destination)->Position{
  let tick_start:Vec3 = position.tick_end;
  let tick_end:Vec3 = position.tick_end + velocity.0;

  let new_position = Position::new(tick_start, tick_end);

  //this tests if the new position is past the destination, if it is, it returns the destination
  let d1 = (new_position.tick_end.x - position.tick_end.x).powi(2) + (new_position.tick_end.z - position.tick_end.z).powi(2);
  let d2 = (destination.0.x - position.tick_end.x).powi(2) + (destination.0.z - position.tick_end.z).powi(2);

  if d1 < d2 {
    new_position
  } else {
    Position::new(tick_start, destination.0.clone())
  }
}
//why does this work but just using the circle-circle test panics
fn collision_test(world:&World, entity_position:Position, entity_radius:f32, test_id:usize,)->Result<bool>{
  let test_position = world.immut_get_component_by_entity_id::<Position>(test_id)?;
  let pathing_radius = world.immut_get_component_by_entity_id::<PathingRadius>(test_id)?;

  let collision_test = circle_circle_collision_test(entity_position.tick_end, entity_radius, test_position.tick_end, pathing_radius.0);
  

  Ok(collision_test)
}