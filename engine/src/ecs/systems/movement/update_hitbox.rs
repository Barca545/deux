use eyre::Result;
use crate::{component_lib::{Position, SelectionRadius}, ecs::World};

//this isn't really the hitbox, it's the selection radius
pub fn update_hitbox(world:&World) -> Result<()> {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>()?
    .with_component::<SelectionRadius>()?
    .run_entity();

  for entity in entities {
    let position = entity.immut_get_component::<Position>()?;
    let mut hitbox = entity.mut_get_component::<SelectionRadius>()?;

    let new_hitbox = SelectionRadius::new(&position, hitbox.0.height, hitbox.0.radius);
    *hitbox = new_hitbox;
  }
  Ok(())
}