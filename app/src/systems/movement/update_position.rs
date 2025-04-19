use game_data::{PathingRadius, Position, PreviousPosition, Velocity};
use math::collisions::circle_circle_collision_test;
use nina::world::World;

///Updates the [`Position`] of all entities in the [`World`].
/// Moves entities forward by their [`Velocity`] component.
/// If they overshoot their [`Destination`] their `Position` is set to their
/// `Destination`. If moving forward would cause a collision with another
/// object, the entity does not move forward.
pub fn update_position(world: &World,) {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>()
    .unwrap()
    .with_component::<Velocity>()
    .unwrap()
    .run();

  'Entity: for entity in entities {
    let previous_position = entity.get_component_mut::<PreviousPosition>().unwrap();
    let position = entity.get_component_mut::<Position>().unwrap();
    let velocity = entity.get_component::<Velocity>().unwrap();
    let pathing_radius = entity.get_component::<PathingRadius>().unwrap();
    let id = entity.id;

    // Get an entity's hypothetical new position (hypothetical because it may be
    // blocked by a collision)
    let new_position = Position(position.0 + velocity.0,);

    // Handle Collisions

    // Ensure no collision is happening by testing for a collision between
    // the entity and all other endities
    // TODO: I dislike this becasue it kind of arbitrarily provides an
    // advantage to first move (I think?)
    let mut query2 = world.query();
    let test_entities = query2.with_component::<PathingRadius>().unwrap().run();

    for test_entity in test_entities {
      let test_entity_id = test_entity.id;

      if test_entity_id != id {
        let collision_check =
          collision_test(world, new_position, pathing_radius.0, test_entity_id,);
        // Stop doing calculations for an entity if a collision is detected
        // `continue` early before their position is updated
        // There is no more reason to continue
        if collision_check {
          continue 'Entity;
        }
      }
    }

    // If the function reaches this point it is because no collisions were detected
    // (which would have caused an early return)
    // Update the entity with its new position
    *previous_position = PreviousPosition::from(*position,);
    *position = new_position;
  }
}

// TODO: The collision test should be in the math crate or something
fn collision_test(
  world: &World,
  entity_position: Position,
  entity_radius: f32,
  test_id: usize,
) -> bool {
  let test_position = world.get_component::<Position>(test_id,).unwrap();
  let pathing_radius = world.get_component::<PathingRadius>(test_id,).unwrap();

  circle_circle_collision_test(
    entity_position.0,
    entity_radius,
    test_position.0,
    pathing_radius.0,
  )
}
