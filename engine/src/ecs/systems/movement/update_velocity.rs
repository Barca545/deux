use crate::ecs::{World, component_lib::{Position, Destination, Speed, Velocity}};
use eyre::Result;

pub fn update_velocity(world:&World) -> Result<()> {
  let mut query = world.query();
  let entities = query
    .with_component::<Position>()?
    .with_component::<Destination>()?
    .with_component::<Speed>()?
    .with_component::<Velocity>()?
    .run_entity();

    for entity in entities {
      let destination = entity.mut_get_component::<Destination>()?;
      let position = entity.mut_get_component::<Position>()?;
      let speed = entity.immut_get_component::<Speed>()?;
      let mut velocity = entity.mut_get_component::<Velocity>()?;
  
      *velocity = Velocity::new(&position, &destination, &speed);
    }  
  Ok(())
}