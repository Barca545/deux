use nina::world::World;

use crate::{
  data_lib::{Position, StaticRenderable},
  // ecs::World,
  view::Renderer
};

pub fn spawn_enviroment(world:&mut World, name:&str, renderer:&mut Renderer) {
  let position = Position::from([0.0, -0.5, 0.0]);
  let ground_model = StaticRenderable(renderer.add_model(name));
  world.create_entity().with_component(ground_model).unwrap().with_component(position).unwrap();
}
