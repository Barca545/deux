use crate::{
  component_lib::{
    Armor, Destination, GameplayRadius, Gold, Health, IncomingDamage, PathingRadius, Position, PreviousPosition, SelectionRadius, SkinnedMesh, Team, Velocity,
    KDA,
  },
  ecs::World,
  filesystem::load_object,
  math::Vec3,
  view::AABB3DDebugMesh,
};
use eyre::Result;
use gl::Gl;

pub fn spawn_dummy(world: &mut World, position: Vec3) -> Result<()> {
  //Create the dummy entity

  let dummy_position = Position(position);
  let dummy_previous_position = PreviousPosition(position);
  let destination = Destination::from(position);
  let dummy_hitbox = SelectionRadius::new(&dummy_position, 2.0, 1.0);
  let (dummy_vertices, dummy_indices) = load_object("box").unwrap();
  let dummy_mesh;
  let dummy_hitbox_mesh;
  {
    let gl = world.get_resource::<Gl>().unwrap();
    dummy_mesh = SkinnedMesh::new(&gl, dummy_vertices, dummy_indices, "wall", 1.0);
    dummy_hitbox_mesh = AABB3DDebugMesh::new(&gl, dummy_hitbox.0, position);
  }
  //Combat info
  let dummy_team = Team::Red;
  let dummy_health = Health::new(500);
  let incoming_damage = IncomingDamage::new();
  // let dummy_target = Target(None);

  world
    .create_entity()
    // .with_component(Player)?
    .with_component(dummy_mesh)?
    .with_component(dummy_position)?
    .with_component(dummy_previous_position)?
    .with_component(Armor::new(100))?
    .with_component(destination)?
    // .with_component(UnitSpeed::new(0.05))?
    .with_component(Velocity::default())?
    .with_component(dummy_hitbox)?
    .with_component(dummy_hitbox_mesh)?
    .with_component(PathingRadius(0.2))?
    .with_component(GameplayRadius(0.1))?
    .with_component(dummy_team)?
    .with_component(dummy_health)?
    .with_component(Gold::default())?
    .with_component(KDA::default())?
    .with_component(incoming_damage)?;
  Ok(())
}
