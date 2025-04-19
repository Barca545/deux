use game_data::{
  Controllable, Destination, PathingRadius, Position, PreviousPosition, Stalker, UnitSpeed,
  Velocity,
};
use math::collisions::circle_circle_collision_test;
use nina::world::{query::query_entity::QueryEntity, World};

/// Updates the [`Position`] of all [`Controllable`] entities in the [`World`].
/// Moves entities forward by their [`Velocity`] component.
pub fn update_position(world: &World,) {
  let mut query: nina::world::query::query::Query<'_,> = world.query();

  let entities = query
    .with_component::<Controllable>()
    .unwrap()
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

    // Get an entity's hypothetical new position (hypothetical because it may be
    // blocked by a collision)
    let new_position = Position(position.0 + velocity.0,);

    // Ensure no collision is happening by testing for a collision between
    // the entity and all other endities

    let mut query2 = world.query();
    let test_entities = query2.with_component::<PathingRadius>().unwrap().run();

    // If len is not 0 collisions were detected and this entity cannot move
    // TODO: I dislike this because it kind of arbitrarily provides an
    // advantage to first move (I think?)
    if test_collisions(entity.id, &new_position, pathing_radius, &test_entities,).len() > 0 {
      continue 'Entity;
    }

    // If the function reaches this point it is because no collisions were detected
    // (which would have caused an early return)
    // Update the entity with its new position
    *previous_position = PreviousPosition::from(*position,);
    *position = new_position;
  }
}

/// Update the [`Destination`] and [`Velocity`] of entities with the [`Stalker`]
/// component.
pub fn update_stalkers(world: &World,) {
  let mut query = world.query();
  let entities = query.with_component::<Stalker>().unwrap().run();

  // Get the target
  for entity in entities {
    if let Some(target,) = entity.get_component::<Stalker>().unwrap().target {
      // Update the stalker's destination
      entity.get_component_mut::<Destination>().unwrap().0 =
        world.get_component::<Position>(target,).unwrap().0;

      // TODO: Also have to update the velocity here
      // I want to move this somewhere better
      *entity.get_component_mut::<Velocity>().unwrap() = Velocity::new(
        entity.get_component::<Position>().unwrap(),
        entity.get_component_mut::<Destination>().unwrap(),
        &entity.get_component_mut::<UnitSpeed>().unwrap().max(),
      );
    }
  }
}

pub fn update_position_of_entities_with_destination(world: &World,) {
  update_stalkers(world,);
  let mut query = world.query();
  let entities = query
    .with_component::<Destination>()
    .unwrap()
    .without_component::<Controllable>()
    .unwrap()
    .run();

  for entity in entities {
    let previous_position = entity.get_component_mut::<PreviousPosition>().unwrap();
    let position = entity.get_component_mut::<Position>().unwrap();
    let velocity = entity.get_component::<Velocity>().unwrap();
    let pathing_radius = entity.get_component::<PathingRadius>().unwrap();

    let new_position = Position(position.0 + velocity.0,);

    // If len is not 0 collisions were detected and this entity cannot move
    // TODO: I dislike this because it kind of arbitrarily provides an
    // advantage to first move (I think?)
    let mut query2 = world.query();
    let test_entities = query2.with_component::<PathingRadius>().unwrap().run();
    if test_collisions(entity.id, &new_position, pathing_radius, &test_entities,).len() > 0 {
      continue;
    }
    *previous_position = PreviousPosition::from(*position,);
    *position = new_position;
  }
}

/// Test if any entities in `candidates` are colliding with the entity defined
/// by `subject_position` and `subject_radius`. Returns the entity ids with whom
/// collisions were detected.
// TODO: I am like 90% sure there is a better way to do collisions. Maybe with
// raycasting?
fn test_collisions(
  subject_id: usize,
  subject_position: &Position,
  subject_radius: &PathingRadius,
  candidates: &Vec<QueryEntity,>,
) -> Vec<usize,> {
  let mut collisions = Vec::new();

  for entity in candidates {
    // Can't collide with self
    if subject_id == entity.id {
      continue;
    }
    if circle_circle_collision_test(
      subject_position.0,
      subject_radius.0,
      entity.get_component::<Position>().unwrap().0,
      entity.get_component::<PathingRadius>().unwrap().0,
    ) {
      collisions.push(entity.id,);
    }
  }
  collisions
}
