use crate::utils::load::load_champion;
use game_data::{
  Controllable, DebugElements, Destination, Exp, Gold, Health, IncomingDamage, Level, MissleSpeed,
  Path, PhysicalDamage, Player, PlayerState, Position, PreviousPosition, SelectionRadius,
  SkinnedRenderable, SpellResource, Target, Team, UnitSpeed, Velocity, KDA,
};
use math::Vec3;
use nina::world::World;
use renderer::renderer::Renderer;

// Refactor
// -Missing some component the combat system needs
// -Add the scripts as something that gets loaded in
// -Make spawn location based on player number
// -Add command buffer so I can set up the situational components
// -Add armor to JSON
// -Controllable flag information needs to get passed in from somewhere else
// -Team information needs to get passed in from somewhere else

/// Spawns a player from a given champion name and player number.
pub fn spawn_player(world: &mut World, name: &str, number: u32, renderer: &mut Renderer,) {
  // Load player information JSON
  let champion_info = load_champion(name,).unwrap();

  // Basic info
  let player = Player(number,);
  let controllable = Controllable;
  let health = Health::new(champion_info.health,);
  let spell_resource = SpellResource::new(1000,);
  let team = Team::Blue;
  let target = Target(None,);
  let gold = Gold::default();
  let kda = KDA::default();
  let exp = Exp::default();
  let level = Level::default();
  let player_state = PlayerState::default();

  //Movement and collision info
  let position_vec = Vec3::new(0.0, 0.0, 0.0,);
  let position = Position(position_vec,);
  let previous_position = PreviousPosition(position_vec,);
  let destination = Destination(position_vec,);
  let speed = UnitSpeed::new(champion_info.unit_speed,);
  let velocity = Velocity::default();
  let selection_radius = SelectionRadius::new(
    &position,
    champion_info.selection_radius.height,
    champion_info.selection_radius.radius,
  );
  let gameplay_radius = champion_info.gameplay_radius;
  let pathing_radius = champion_info.pathing_radius;
  let path = Path::new();

  //Render info
  let player_model = SkinnedRenderable(renderer.add_model(name,),);

  //Combat info
  let incoming_damage = IncomingDamage::new();
  let auto_attack_missle_speed = MissleSpeed::new(champion_info.auto_attack_missle_speed,);
  let attack_damage = PhysicalDamage::new(champion_info.attack_damage,);

  world
    .create_entity()
    //General components
    .with_component(player,)
    .unwrap()
    .with_component(controllable,)
    .unwrap()
    .with_component(health,)
    .unwrap()
    .with_component(team,)
    .unwrap()
    .with_component(target,)
    .unwrap()
    .with_component(gold,)
    .unwrap()
    .with_component(kda,)
    .unwrap()
    .with_component(exp,)
    .unwrap()
    .with_component(level,)
    .unwrap()
    .with_component(player_state,)
    .unwrap()
    //Movement and collision components
    .with_component(position,)
    .unwrap()
    .with_component(previous_position,)
    .unwrap()
    .with_component(destination,)
    .unwrap()
    .with_component(speed,)
    .unwrap()
    .with_component(velocity,)
    .unwrap()
    .with_component(selection_radius,)
    .unwrap()
    .with_component(gameplay_radius,)
    .unwrap()
    .with_component(pathing_radius,)
    .unwrap()
    .with_component(path,)
    .unwrap()
    //Casting components
    .with_component(spell_resource,)
    .unwrap()
    .with_component(auto_attack_missle_speed,)
    .unwrap()
    .with_component(attack_damage,)
    .unwrap()
    .with_component(incoming_damage,)
    .unwrap()
    //Render components
    .with_component(player_model,)
    .unwrap();

  let debug = world.get_resource::<DebugElements>();
  if debug.aabb {
    //if true add the dbg mesh
  }

  // if controllable {
  //   // add the controllable component only if controllable on the JSON is
  // true }
}
