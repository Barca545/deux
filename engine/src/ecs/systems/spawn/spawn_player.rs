use crate::{
  component_lib::{
    AbilityMap, AutoAttackMesh, Controllable, Cooldowns, Destination, Exp, GameplayRadius, Gold, Level, Path, Player, PlayerState, Position, PreviousPosition,
    Script, SelectionRadius, SkinnedMesh, SpellResource, Target, Team, Velocity, KDA,
  },
  ecs::{world_resources::DebugElements, World},
  filesystem::{load_champion_json, load_object},
  math::Vec3,
  time::ServerTime,
  view::AABB3DDebugMesh,
};
use eyre::Result;
use gl::Gl;

// Refactor
// -Missing some component the combat system needs
// -Add the scripts as something that gets loaded in.
// -Add dynamic bundles like in Hecs
// -Make spawn location based on player number
// -Add command buffer so I can set up the situational components
// -Add armor to JSON
// -Controllable flag information needs to get passed in from somewhere else
// -Team information needs to get passed in from somewhere else
// -GamplayRadius needs to load in from JSON
// -Make mesh something stored as a resource not a component and just give entities access to wherever they're stored

///Spawns a player from a given champion name and player number.
pub fn spawn_player(world: &mut World, name: &str, number: u32) -> Result<()> {
  //Load player information JSON
  let champion_info = load_champion_json(name)?;

  //Create the player entity
  //Basic info
  let player = Player(number);
  let controllable = Controllable;
  let health = champion_info.health;
  let team = Team::Blue;
  let target = Target(None);
  let gold = Gold::default();
  let kda = KDA::default();
  let exp = Exp::default();
  let level = Level::default();

  //Movement and collision info
  let position_vec = Vec3::new(0.0, 0.0, 0.0);
  let position = Position(position_vec);
  let previous_position = PreviousPosition(position_vec);
  let destination = Destination(position_vec);
  let speed = champion_info.speed;
  let velocity = Velocity::default();
  let selection_radius = SelectionRadius::new(&position, champion_info.selection_radius.height, champion_info.selection_radius.radius);
  let gameplay_radius = GameplayRadius(1.0);
  let pathing_radius = champion_info.pathing_radius;
  let path = Path::new();
  let status = PlayerState::default();

  //Render info
  let (sprite_vertices, sprite_indices) = load_object(name)?;
  let (auto_attack_vertices, auto_attack_indices) = load_object("ball")?;

  let player_mesh;
  let auto_attack_mesh;
  let player_hitbox_mesh;
  //Spawn the meshes
  {
    let gl = world.get_resource::<Gl>().unwrap();
    player_hitbox_mesh = AABB3DDebugMesh::new(&gl, selection_radius.0, position_vec);
    player_mesh = SkinnedMesh::new(&gl, sprite_vertices, sprite_indices, "blank_texture", 1.0);
    auto_attack_mesh = AutoAttackMesh::new(&gl, auto_attack_vertices, auto_attack_indices, "allied_attack", 0.5);
  }

  //Casting info
  let spell_resource = SpellResource(1000);

  //Combat info
  let auto_attack_missle_speed = champion_info.auto_attack_missle_speed;
  let attack_damage = champion_info.attack_damage;
  let basic_cooldown_duration = champion_info.auto_attack_cooldown;
  let auto_windup = 0.0;
  let ability_1_cooldown_duration = 0.0;
  let ability_2_cooldown_duration = 0.0;
  let ability_3_cooldown_duration = 0.0;
  let ability_4_cooldown_duration = 0.0;
  //Script info
  let ability_1 = Script::new("start", "onhit", "running", "stop");
  let ability_2 = Script::new("start", "onhit", "running", "stop");
  let ability_3 = Script::new("start", "onhit", "running", "stop");
  let ability_4 = Script::new("start", "onhit", "running", "stop");
  let autoattack = Script::new(
    r#"
  local target = world:getTarget(owner.id);
  local cost = 50;
  if world:has_resource(owner.id, cost) and world:target_is_alive(target) and world:is_enemy(owner.id, target) then 
    world:remove_resource(owner.id, cost);
    world:spawnTargetedProjectile(owner.id, target);
    return true
  else
    return false
  end
"#,
    r#"
    local damage = world:get_attack_damage(owner.id);
    return damage
    "#,
    "running",
    "stop",
  );
  let ability_map = AbilityMap::new(ability_1, ability_2, ability_3, ability_4, autoattack);

  //Create timers
  let cooldowns;
  {
    let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
    cooldowns = Cooldowns::new(
      &mut server_time,
      basic_cooldown_duration,
      auto_windup,
      ability_1_cooldown_duration,
      ability_2_cooldown_duration,
      ability_3_cooldown_duration,
      ability_4_cooldown_duration,
    );
  }

  world
    .create_entity()
    //General components
    .with_component(player)?
    .with_component(controllable)?
    .with_component(health)?
    .with_component(team)?
    .with_component(target)?
    .with_component(gold)?
    .with_component(kda)?
    .with_component(exp)?
    .with_component(level)?
    .with_component(status)?
    //Movement and collision components
    .with_component(position)?
    .with_component(previous_position)?
    .with_component(destination)?
    .with_component(speed)?
    .with_component(velocity)?
    .with_component(selection_radius)?
    .with_component(gameplay_radius)?
    .with_component(pathing_radius)?
    .with_component(path)?
    //Casting components
    .with_component(spell_resource)?
    //Combat components
    .with_component(auto_attack_mesh)?
    .with_component(auto_attack_missle_speed)?
    .with_component(cooldowns)?
    .with_component(attack_damage)?
    .with_component(ability_map)?
    //Render components
    .with_component(player_mesh)?
    .with_component(player_hitbox_mesh)?;

  let debug = world.get_resource::<DebugElements>().unwrap();
  if debug.aabb {
    //if true add the dbg mesh
  }

  // if controllable {
  //   // add the controllable component only if controllable on the JSON is true
  // }

  Ok(())
}
