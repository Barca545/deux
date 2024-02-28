use crate::{component_lib::{GameplayRadius, Gold, Health, PathingRadius, Position, PreviousPosition, SelectionRadius, SkinnedMesh, Team, KDA}, ecs::World, filesystem::load_object, math::Vec3, view::AABB3DDebugMesh};
use gl::Gl;

pub fn spawn_dummy(world:&mut World, gl:Gl, position:Vec3){
  //Create the dummy entity 
  let dummy_position_vec:Vec3 = position;
  let dummy_position = Position(dummy_position_vec);
  let dummy_previous_position = PreviousPosition(dummy_position_vec);
  let dummy_hitbox = SelectionRadius::new(&dummy_position, 0.2, 0.7);
  let dummy_hitbox_mesh = AABB3DDebugMesh::new(&gl, dummy_hitbox.0, dummy_position_vec);
  
  let (dummy_vertices, dummy_indices) = load_object("box").unwrap();
  let dummy_mesh = SkinnedMesh::new(&gl,dummy_vertices,dummy_indices,"wall", 1.0);

  //combat info
  let dummy_team = Team::RED;
  let dummy_health = Health::new(500);
  // let dummy_target = Target(None);

  world
    .create_entity()
    // .with_component(Player).unwrap()
    .with_component(dummy_mesh).unwrap()
    .with_component(dummy_position).unwrap()
    .with_component(dummy_previous_position).unwrap()
    // .with_component(Destination::new(0.0, 0.0, 0.0)).unwrap()
    // .with_component(Speed(0.05)).unwrap()
    // .with_component(Velocity::default()).unwrap()
    .with_component(dummy_hitbox).unwrap()
    .with_component(dummy_hitbox_mesh).unwrap()
    .with_component(PathingRadius(0.2)).unwrap()
    .with_component(GameplayRadius(0.1)).unwrap()
    .with_component(dummy_team).unwrap()
    .with_component(dummy_health).unwrap()
    .with_component(Gold::default()).unwrap()
    .with_component(KDA::default()).unwrap();
}