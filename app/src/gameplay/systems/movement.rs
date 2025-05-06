use std::collections::HashMap;

use game_data::{
  player_movement::PlayerMovement, Controllable, Destination, PathingRadius, Position,
  PreviousPosition, Stalker, UnitSpeed, Velocity,
};
use math::{
  // collisions::{circle_circle_collision_test, intersection_seg_seg, LnSeg},
  Vec2,
  Vec3,
};
use nina::world::{query::query_entity::QueryEntity, World};

pub fn movement(world: &mut World,) {
  update_velocity(world,);
  update_position(world,);
  update_position_of_entities_with_destination(world,);
}

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
    // if circle_circle_collision_test(
    //   subject_position.0,
    //   subject_radius.0,
    //   entity.get_component::<Position>().unwrap().0,
    //   entity.get_component::<PathingRadius>().unwrap().0,
    // ) {
    //   collisions.push(entity.id,);
    // }
  }
  collisions
}

// TODO: Eventually this will handle all entities with velocity and the function
// docs will need updating

/// Updates the [`Velocity`] component of the [`Controllable`] character.
pub fn update_velocity(world: &World,) {
  // Get the player
  let mut query = world.query();
  let player = &query.with_component::<Controllable>().unwrap().run()[0];

  // Get player movement stats
  let velocity = world.get_component_mut::<Velocity>(player.id,).unwrap();
  let speed = world.get_component::<UnitSpeed>(player.id,).unwrap();
  let player_input_state = world.get_resource_mut::<PlayerMovement>();

  // If there is a horizontal input add it to the player's velocity.
  match (player_input_state.left, player_input_state.right,) {
    // Both buttons pressed cancel each other out
    (true, true,) => velocity.0.x = 0.0,
    // Only left pressed means go left
    (true, false,) => velocity.0.x = 1.0,
    // Only right pressed means go right
    (false, true,) => velocity.0.x = -1.0,
    // Nothing to do if nothing is pressed :P
    (false, false,) => velocity.0.x = 0.0,
  }

  // If there is a vertical input add it to the player's velocity.
  match (player_input_state.up, player_input_state.down,) {
    // Both buttons pressed cancel each other out
    (true, true,) => velocity.0.z = 0.0,
    // Only up pressed means go up
    (true, false,) => velocity.0.z = 1.0,
    // Only down pressed means go down
    (false, true,) => velocity.0.z = -1.0,
    // Nothing to do if nothing is pressed :P
    (false, false,) => velocity.0.z = 0.0,
  }

  // Normalize + scale velocity to ensure it always has a |v| = speed
  if velocity.mag() > speed.max() {
    *velocity = Velocity(velocity.0.normalize().scale(speed.max(),),);
  }
}

fn check_collisions(world: &World,) -> Option<Vec3,> {
  // let mut collisions = HashMap::new();

  let mut query = world.query();
  // TODO: Will need more than a position, I believe
  let entities = query.with_component::<Position>().unwrap().run();

  // Check for a collision
  for entity in &entities {
    let start = entity.get_component::<PreviousPosition>().unwrap();
    let end = entity.get_component::<Position>().unwrap();
    for test in &entities {
      let test_start = entity.get_component::<PreviousPosition>().unwrap();
      let test_end = entity.get_component::<Position>().unwrap();
      // match intersection_seg_seg(
      //   // Entity's segment
      //   LnSeg::from((
      //     Vec2::new(start.0.x, start.0.y,),
      //     Vec2::new(end.0.x, end.0.y,),
      //   ),),
      //   // Test's segment
      //   LnSeg::from((
      //     Vec2::new(test_start.0.x, test_start.0.y,),
      //     Vec2::new(test_end.0.x, test_end.0.y,),
      //   ),),
      // ) {
      //   Some(collision,) => {
      //     collisions.insert((entity.id, test.id,), collision,);
      //   }
      //   None => {}
      // }
    }
  }

  // Each time an entity pair is tested store the result in the collisions
  // TODO:
  // hashmap collisions.insert((id_1, id_2,), Vec3::default(),);
  todo!()
}

// Each boid has
// - a center
// - Constant speed
// - A direction

// Need some way of grouping each cluster
// I think I need like a Boid type and cluster ID
// The parameters should be tunable by type and they should only try to group
// with others from their cluster

// I don't really think I want boid behavior
// Movement rules are:
// - Move towards player
// - Try to avoid other cluster members very slightly
// - Should try to maintain a clump

/// Parameter controlling the Boids' tendency to steer towards their flockmates.
const COHERENCE: u32 = 5;
/// Parameter controlling how strongly boids try to avoid their flockmates.
const SEPARATION: u32 = 5;
/// Parameter controlling the Boids' tendency to move in the same direction as
/// their flockmates.
// TODO: Alignment should be fairly strong since we want them to chase
const ALIGHTMENT: u32 = 5;

const VISUAL_RANGE: u32 = 5;
