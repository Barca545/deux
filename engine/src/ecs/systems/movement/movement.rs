use crate::ecs::World;
use eyre::Result;
use super::{update_velocity::update_velocity, update_position, update_hitbox};

//Refactor
//the below should be incorporated into the system's description
//mouse ray should be a resource that is updated when the mouse moves
//arguably the mouse ray is information both the selection and this system
// needs selection needs to run first and do the AABB test
//this should only run if the selection test says nothing is selected

pub fn movement(world:&World) -> Result<()> {
  update_velocity(world)?;
  update_position(world)?;
  update_hitbox(world)?;
  Ok(())
}

#[cfg(test)]
mod test {
  use super::update_position;
  use crate::{
    component_lib::{Destination, Position, UnitSpeed, Velocity}, 
    ecs::World, 
    math::Vec3
  };
  use eyre::Result;

  #[test]
  fn get_direction() {
    let position = Position(Vec3::new(0.0, 0.0, 0.0));
    let destination = Destination(Vec3::new(3.0, 4.0, 0.0));
    let speed = UnitSpeed(1.0);

    let velocity = Velocity::new(&position, &destination, &speed.0);
    dbg!(velocity.0);

    let velocity = Velocity::new(&position, &destination, &speed.0);

    dbg!(velocity.0);
  }

  #[test]
  fn test_update_position() -> Result<()> {
    let position = Position(Vec3::new(0.0, 0.0, 0.0));
    let destination = Destination(Vec3::new(3.0, 0.0, 3.0));
    let speed = UnitSpeed(5.0);
    let velocity = Velocity::new(&position, &destination, &speed.0);
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

    update_position(&world)?;

    //Confirm the update occured
    let mut query = world.query();

    let entities = query.with_component::<Position>()?.run_entity();

    for entity in entities {
      let updated_position = entity.immut_get_component::<Position>()?;
      dbg!(updated_position.0);
    }
    Ok(())
  }
}
