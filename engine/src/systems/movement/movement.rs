use super::{move_attacks::move_attacks, update_destination, update_hitbox, update_path, update_position, update_velocity};
use nina::world::World;

// Refactor:
// -Update path, update destination, and update velocity need to be integrated
// -Mouse ray should be a resource that is updated when the mouse moves
//  mouse ray is information both the selection and this system
//  needs selection needs to run first and do the AABB test
//  this should only run if the selection test says nothing is selected
// -Merge update path and update destination?
// -Have the system run a pathing system if the destination is past a certain
// distance -System should check for a path, if there is a path run the path
// system otherwise run the current move system -Clicking anything should clear
// the current Path and rerun the pathing calculation if applicable

pub fn movement(world:&mut World) {
  update_path(world);
  update_destination(world);
  update_velocity(world);
  update_position(world);
  update_hitbox(world);
  move_attacks(world);
}

#[cfg(test)]
mod test {
  use super::update_position;
  use crate::{
    data_lib::{Destination, Position, UnitSpeed, Velocity},
    math::Vec3
  };
  use eyre::Result;
  use nina::world::World;

  #[test]
  fn get_direction() {
    let position = Position(Vec3::new(0.0, 0.0, 0.0));
    let destination = Destination(Vec3::new(3.0, 4.0, 0.0));
    let speed = UnitSpeed::new(1.0);

    let velocity = Velocity::new(&position, &destination, &speed.total());
    dbg!(velocity.0);

    let velocity = Velocity::new(&position, &destination, &speed.total());

    dbg!(velocity.0);
  }

  #[test]
  fn test_update_position() -> Result<()> {
    let position = Position(Vec3::new(0.0, 0.0, 0.0));
    let destination = Destination(Vec3::new(3.0, 0.0, 3.0));
    let speed = UnitSpeed::new(5.0);
    let velocity = Velocity::new(&position, &destination, &speed.total());
    dbg!(velocity);

    let mut world = World::new();

    world
      .register_component::<Position>()
      .register_component::<Destination>()
      .register_component::<UnitSpeed>()
      .register_component::<Velocity>();

    //entity the system should target
    world
      .create_entity()
      .with_component(position)
      .unwrap()
      .with_component(destination)
      .unwrap()
      .with_component(speed)
      .unwrap()
      .with_component(velocity)
      .unwrap();

    //entity the system should ignore
    world
      .create_entity()
      .with_component(position)
      .unwrap()
      .with_component(speed)
      .unwrap()
      .with_component(velocity)
      .unwrap();

    update_position(&world);

    //Confirm the update occured
    let mut query = world.query();

    let entities = query.with_component::<Position>().unwrap().run();

    for entity in entities {
      let updated_position = entity.get_component::<Position>().unwrap();
      dbg!(updated_position.0);
    }
    Ok(())
  }
}