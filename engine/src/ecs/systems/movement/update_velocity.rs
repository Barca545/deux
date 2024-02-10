use crate::{
  component_lib::{Destination, Position, UnitSpeed, Velocity}, 
  ecs::World
};
use eyre::Result;

///Updates the velocities of all entities in the world.
pub fn update_velocity(world:&World) -> Result<()> {
  let mut query = world.query();
  let entities = query
    .with_component::<Position>()?
    .with_component::<Destination>()?
    .with_component::<UnitSpeed>()?
    .with_component::<Velocity>()?
    .run_entity();

    for entity in entities {
      let destination = entity.mut_get_component::<Destination>()?;
      let position = entity.mut_get_component::<Position>()?;
      let speed = entity.immut_get_component::<UnitSpeed>()?;
      let mut velocity = entity.mut_get_component::<Velocity>()?;
      *velocity = Velocity::new(&position, &destination, &speed.0);
    }  
  Ok(())
}