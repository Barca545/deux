use crate::{
  data_lib::{Destination, PathingRadius, Position, PreviousPosition, Velocity},
  physics::circle_circle_collision_test,
};
use nina::world::World;

// Refactor
// -Figure out why circle to circle collision panics for the collision check
// -Confirm new previous position does not take ownership of the position

///Updates the [`Position`] of all entities in the [`World`].
/// Moves entities forward by their [`Velocity`] component.
/// If they overshoot their [`Destination`] their `Position` is set to their
/// `Destination`. If moving forward would cause a collision with another
/// object, the entity does not move forward.
pub fn update_position(world:&World,) {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>()
    .unwrap()
    .with_component::<Velocity>()
    .unwrap()
    // .with_component::<Destination>()
    // .unwrap()
    .run();

  for entity in entities {
    let previous_position = entity.get_component_mut::<PreviousPosition>().unwrap();
    let position = entity.get_component_mut::<Position>().unwrap();
    let velocity = entity.get_component::<Velocity>().unwrap();
    let destination = entity.get_component::<Destination>().unwrap();
    let pathing_radius = entity.get_component::<PathingRadius>().unwrap();
    let id = entity.id;

    let new_position = Position(position.0 + velocity.0,);
    *previous_position = PreviousPosition::from(*position,);
    *position = new_position;
    dbg!(previous_position);

    // if position.0 != destination.0 {
    //   let new_position = Position(position.0 + velocity.0,);
    //   let mut collision_check = false;

    //   // Ensure no collision is happening by testing for a collision between
    // the   // entity and all other endities
    //   // TODO: I dislike this becasue it kind of arbitrarily provides an
    // advantage to   // first move (I think?)
    //   let mut query2 = world.query();
    //   let test_entities =
    // query2.with_component::<PathingRadius>().unwrap().run();

    //   for test_entity in test_entities {
    //     let test_entity_id = test_entity.id;

    //     if test_entity_id != id {
    //       collision_check = collision_test(world, new_position,
    // pathing_radius.0, test_entity_id,);     }
    //     // Return early if a collision is detected, there is no more reason
    // to continue     if collision_check {
    //       return;
    //     }
    //   }

    //   if !collision_check {
    //     *previous_position = PreviousPosition::from(*position,);
    //     *position = new_position;
    //   }
    // }
  }
}

fn calculate_new_position(
  position:Position,
  velocity:Velocity,
  destination:Destination,
) -> Position {
  // let new_position = Position(position.0 + velocity.0,);

  // Test if the entity's new position is past its destination.
  // If the entity has overshot its destination, return the destination,
  // otherwise return the new position.
  // let d1 = (new_position.0.x - position.0.x).powi(2,) + (new_position.0.z -
  // position.0.z).powi(2,);
  // let d2 = (destination.0.x - position.0.x).powi(2,) + (destination.0.z -
  // position.0.z).powi(2,);
  // if d1 < d2 {
  //   new_position
  // }
  // else {
  //   Position::from(destination,)
  // }

  Position(position.0 + velocity.0,)
}

fn collision_test(
  world:&World,
  entity_position:Position,
  entity_radius:f32,
  test_id:usize,
) -> bool {
  let test_position = world.get_component::<Position>(test_id,).unwrap();
  let pathing_radius = world.get_component::<PathingRadius>(test_id,).unwrap();

  let collision_test = circle_circle_collision_test(
    entity_position.0,
    entity_radius,
    test_position.0,
    pathing_radius.0,
  );

  collision_test
}
