use crate::{
  component_lib::{Armor, GameplayRadius, Gold, Health, PathingRadius, Position, PreviousPosition, SelectionRadius, SkinnedMesh, Team, KDA},
  ecs::World,
  filesystem::load_object,
  math::Vec3,
  view::AABB3DDebugMesh,
};
use eyre::Result;
use gl::Gl;

pub fn spawn_dummy(world: &mut World, gl: Gl, position: Vec3) -> Result<()> {
  //Create the dummy entity
  let dummy_position_vec: Vec3 = position;
  let dummy_position = Position(dummy_position_vec);
  let dummy_previous_position = PreviousPosition(dummy_position_vec);
  let dummy_hitbox = SelectionRadius::new(&dummy_position, 2.0, 0.1);
  let dummy_hitbox_mesh = AABB3DDebugMesh::new(&gl, dummy_hitbox.0, dummy_position_vec);

  let (dummy_vertices, dummy_indices) = load_object("box").unwrap();
  let dummy_mesh = SkinnedMesh::new(&gl, dummy_vertices, dummy_indices, "wall", 1.0);

  //combat info
  let dummy_team = Team::Red;
  let dummy_health = Health::new(500);
  // let dummy_target = Target(None);

  world
    .create_entity()
    // .with_component(Player)?
    .with_component(dummy_mesh)?
    .with_component(dummy_position)?
    .with_component(dummy_previous_position)?
    .with_component(Armor(100))?
    // .with_component(Destination::new(0.0, 0.0, 0.0))?
    // .with_component(Speed(0.05))?
    // .with_component(Velocity::default())?
    .with_component(dummy_hitbox)?
    .with_component(dummy_hitbox_mesh)?
    .with_component(PathingRadius(0.2))?
    .with_component(GameplayRadius(0.1))?
    .with_component(dummy_team)?
    .with_component(dummy_health)?
    .with_component(Gold::default())?
    .with_component(KDA::default())?;
  Ok(())
}
