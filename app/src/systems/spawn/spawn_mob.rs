use game_data::{
  Armor, Destination, GameplayRadius, Gold, Health, Mob, PathingRadius, Position, PreviousPosition,
  SelectionRadius, SkinnedRenderable, Stalker, Team, UnitSpeed, Velocity, KDA,
};
use nina::world::World;
use renderer::renderer::Renderer;

// Refactor:
// - mob should load in from a JSON too

pub fn spawn_mob(world: &mut World, position: [f32; 3], renderer: &mut Renderer,) {
  // Create the mob entity

  let dummy_position = Position::from(position,);
  let dummy_previous_position = PreviousPosition::from(position,);
  let destination = Destination::from(position,);
  let dummy_hitbox = SelectionRadius::new(&dummy_position, 2.0, 1.0,);

  // Render info
  let mob_model = SkinnedRenderable(renderer.add_model("cube",),);

  // Combat info
  let dummy_team = Team::Red;
  let dummy_health = Health::new(50000000,);
  // let incoming_damage = IncomingDamage::new();

  world
    .create_entity()
    .with_component(Mob,)
    .unwrap()
    .with_component(mob_model,)
    .unwrap()
    .with_component(dummy_position,)
    .unwrap()
    .with_component(dummy_previous_position,)
    .unwrap()
    .with_component(Armor::new(100,),)
    .unwrap()
    .with_component(destination,)
    .unwrap()
    .with_component(Velocity::default(),)
    .unwrap()
    .with_component(dummy_hitbox,)
    .unwrap()
    .with_component(PathingRadius(0.2,),)
    .unwrap()
    .with_component(GameplayRadius(0.1,),)
    .unwrap()
    .with_component(dummy_team,)
    .unwrap()
    .with_component(dummy_health,)
    .unwrap()
    .with_component(Gold::default(),)
    .unwrap()
    .with_component(KDA::default(),)
    .unwrap()
    .with_component(UnitSpeed::new(0.05,),)
    .unwrap()
    // Make the mobs stalk the player
    // TODO: Don't hardcode this
    .with_component(Stalker { target: Some(0,), },)
    .unwrap();
}
