use crate::{
  component_lib::{Destination, PathingRadius, Position, PreviousPosition, Velocity}, 
  ecs::World, 
  physics::circle_circle_collision_test
};

// Refactor
// -Figure out why circle to circle collision panics for the collision check

///Updates the positions of all entitys in the world.
/// Moves entities forward by their `Velocity` component. 
/// If they overshoot their destination their position is set to their destination.
/// If they collide with an object prevent them from moving forward.
/// Otherwise move the character forward.
pub fn update_position(world:&World) {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>().unwrap()
    .with_component::<Velocity>().unwrap()
    .with_component::<Destination>().unwrap()
    .run();

  for entity in entities {
    let mut previous_position = entity.mut_get_component::<PreviousPosition>().unwrap();
    let mut position = entity.mut_get_component::<Position>().unwrap();
    let velocity = entity.immut_get_component::<Velocity>().unwrap();
    let destination = entity.immut_get_component::<Destination>().unwrap();
    let pathing_radius = entity.immut_get_component::<PathingRadius>().unwrap();
    let id = entity.id;

    if position.0 != destination.0 {
      let new_previous_position = PreviousPosition(position.0);
      let new_position = calculate_new_position(*position, *velocity, *destination);
      let mut collision_check = false;
      
      let mut query2 = world.query();
      let test_entities = query2
        .with_component::<PathingRadius>().unwrap()
        .run();
  
      for test_entity in test_entities{
        let test_entity_id = test_entity.id;

        if test_entity_id != id {
          collision_check = collision_test(world, new_position, pathing_radius.0, test_entity_id);
        }
      }

      if !collision_check {
        *previous_position = new_previous_position;
        *position = new_position;
      }
    }
  }
}

///Test if the entity's new position is past its destination.
/// If the entity has overshot its destination, set its return the destination, otherwise return the new position.
fn calculate_new_position(position:Position, velocity:Velocity, destination:Destination)->Position{
  let new_position = Position(position.0 + velocity.0);

  let d1 = (new_position.0.x - position.0.x).powi(2) + (new_position.0.z - position.0.z).powi(2);
  let d2 = (destination.0.x - position.0.x).powi(2) + (destination.0.z - position.0.z).powi(2);

  if d1 < d2 {
    new_position
  } else {
    Position(destination.0)
  }
}

fn collision_test(world:&World, entity_position:Position, entity_radius:f32, test_id:usize,)->bool{
  let test_position = world.immut_get_component_by_entity_id::<Position>(test_id).unwrap();
  let pathing_radius = world.immut_get_component_by_entity_id::<PathingRadius>(test_id).unwrap();

  let collision_test = circle_circle_collision_test(entity_position.0, entity_radius, test_position.0, pathing_radius.0);
  
  collision_test
}