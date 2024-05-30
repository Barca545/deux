use nina::world::World;

use crate::{
  data_lib::{
    Armor, Destination, GameplayRadius, Gold, Health, IncomingDamage, PathingRadius, Position, PreviousPosition, SelectionRadius, SkinnedRenderable, Team,
    Velocity, KDA
  },
  // ecs::World,
  view::Renderer
};

// Refactor:
// -Dummy should load in from a JSON too

pub fn spawn_dummy(world:&mut World, position:[f32; 3], renderer:&mut Renderer) {
  //Create the dummy entity

  let dummy_position = Position::from(position);
  let dummy_previous_position = PreviousPosition::from(position);
  let destination = Destination::from(position);
  let dummy_hitbox = SelectionRadius::new(&dummy_position, 2.0, 1.0);

  //Render info
  let player_model = SkinnedRenderable(renderer.add_model("cube"));

  //Combat info
  let dummy_team = Team::Red;
  let dummy_health = Health::new(50000000);
  let incoming_damage = IncomingDamage::new();
  // let dummy_target = Target(None);

  world
    .create_entity()
    // .with_component(Player).unwrap()
    .with_component(player_model)
    .unwrap()
    .with_component(dummy_position)
    .unwrap()
    .with_component(dummy_previous_position)
    .unwrap()
    .with_component(Armor::new(100))
    .unwrap()
    .with_component(destination)
    .unwrap()
    // .with_component(UnitSpeed::new(0.05)).unwrap()
    .with_component(Velocity::default())
    .unwrap()
    .with_component(dummy_hitbox)
    .unwrap()
    .with_component(PathingRadius(0.2))
    .unwrap()
    .with_component(GameplayRadius(0.1))
    .unwrap()
    .with_component(dummy_team)
    .unwrap()
    .with_component(dummy_health)
    .unwrap()
    .with_component(Gold::default())
    .unwrap()
    .with_component(KDA::default())
    .unwrap()
    .with_component(incoming_damage)
    .unwrap();
  // Ok(())
}
