use crate::{component_lib::{Position, SelectionRadius}, ecs::World};

//this isn't really the hitbox, it's the selection radius
pub fn update_hitbox(world:&World) {
  let mut query = world.query();

  let entities = query
    .with_component::<Position>().unwrap()
    .with_component::<SelectionRadius>().unwrap()
    .run
();

  for entity in entities {
    let position = entity.immut_get_component::<Position>().unwrap();
    let mut hitbox = entity.mut_get_component::<SelectionRadius>().unwrap();

    let new_hitbox = SelectionRadius::new(&position, hitbox.0.height, hitbox.0.radius);
    *hitbox = new_hitbox;
  }
}