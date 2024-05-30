use crate::data_lib::{Position, SelectionRadius};
use nina::world::World;

//this isn't really the hitbox, it's the selection radius
pub fn update_hitbox(world:&World) {
  let mut query = world.query();

  let entities = query.with_component::<Position>().unwrap().with_component::<SelectionRadius>().unwrap().run();

  for entity in entities {
    let position = entity.get_component::<Position>().unwrap();
    let hitbox = entity.get_component_mut::<SelectionRadius>().unwrap();

    let new_hitbox = SelectionRadius::new(&position, hitbox.0.height, hitbox.0.radius);
    *hitbox = new_hitbox;
  }
}
