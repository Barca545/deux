use crate::{
  component_lib::{
    AbilityInfo, AbilityMap, AutoAttackMesh, CastQueue, Controllable, Destination, Exp, GameplayRadius, Gold, Health, IncomingDamage, Level, MissleSpeed, Path,
    PhysicalDamage, Player, PlayerState, Position, PreviousPosition, Script, SelectionRadius, SkinnedMesh, SpellResource, Target, Team, UnitSpeed, Velocity,
    KDA,
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
  let health = Health::new(champion_info.health);
  let spell_resource = SpellResource::new(1000);
  let team = Team::Blue;
  let target = Target(None);
  let gold = Gold::default();
  let kda = KDA::default();
  let exp = Exp::default();
  let level = Level::default();
  let player_state = PlayerState::default();

  //Movement and collision info
  let position_vec = Vec3::new(0.0, 0.0, 0.0);
  let position = Position(position_vec);
  let previous_position = PreviousPosition(position_vec);
  let destination = Destination(position_vec);
  let speed = UnitSpeed::new(champion_info.unit_speed);
  let velocity = Velocity::default();
  let selection_radius = SelectionRadius::new(&position, champion_info.selection_radius.height, champion_info.selection_radius.radius);
  let gameplay_radius = GameplayRadius(1.0);
  let pathing_radius = champion_info.pathing_radius;
  let path = Path::new();

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

  //Combat info
  let incoming_damage = IncomingDamage::new();
  let auto_attack_missle_speed = MissleSpeed::new(champion_info.auto_attack_missle_speed);
  let attack_damage = PhysicalDamage::new(champion_info.attack_damage);
  let auto_attack_cooldown_duration = champion_info.auto_attack_cooldown;
  let ability_1_cooldown_duration = 0.0;
  let ability_2_cooldown_duration = 0.0;
  let ability_3_cooldown_duration = 0.0;
  let ability_4_cooldown_duration = 0.0;
  let ability_1_cast_time = 0.0;
  let ability_2_cast_time = 0.0;
  let ability_3_cast_time = 0.0;
  let ability_4_cast_time = 0.0;
  let auto_attack_cast_time = 0.10;

  //Script info
  let ability_1_scripts = Script::new(Some("start"), Some("onhit"), Some("running"), Some("stop"));
  let ability_2_scripts = Script::new(Some("start"), Some("onhit"), Some("running"), Some("stop"));
  let ability_3_scripts = Script::new(
    Some(
      r#"
      world:accelerate(owner.id,3.0)
      world:spawnPersistentScript(owner.id,5.0,3);
      local pos = mouse:ground_intersection()
      world:blink(owner.id,pos)
      "#,
    ),
    Some("onhit"),
    None,
    Some(
      r#"
    print("stoping");
    world:accelerate(owner.id,-3.0);
    "#,
    ),
  );
  let ability_4_scripts = Script::new(Some("start"), Some("onhit"), Some("running"), Some("stop"));
  let auto_attack_scripts = Script::new(
    Some(
      r#"
      local target = world:getTarget(entity.id);
      local cost = 50;
      if world:hasResource(owner.id, cost) and world:targetIsalive(target.id) and world:isEnemy(owner.id, target.id) then 
        world:removeResource(owner.id, cost);
        world:spawnTargetedProjectile(owner.id, target.id);
        return true
      else
        return false
      end
    "#,
    ),
    Some(
      r#"
      local target = world:getTarget(owner.id);
      world:knockback(owner.id,target.id,0.1,1.0);
      local damage = world:getPhysicalDamage(owner.id);
      world:dealTrueDamage(owner.id,target.id,damage*100);
    "#,
    ),
    Some("running"),
    Some("stop"),
  );
  let mut ability_map = AbilityMap::default();
  {
    //Create the abilityinfo for the basic abilities
    let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
    let ability_1 = AbilityInfo::new(ability_1_cooldown_duration, &mut server_time, ability_1_cast_time, ability_1_scripts);
    let ability_2 = AbilityInfo::new(ability_2_cooldown_duration, &mut server_time, ability_2_cast_time, ability_2_scripts);
    let ability_3 = AbilityInfo::new(ability_3_cooldown_duration, &mut server_time, ability_3_cast_time, ability_3_scripts);
    let ability_4 = AbilityInfo::new(ability_4_cooldown_duration, &mut server_time, ability_4_cast_time, ability_4_scripts);
    let auto_attack = AbilityInfo::new(auto_attack_cooldown_duration, &mut server_time, auto_attack_cast_time, auto_attack_scripts);

    //Insert the basic abilities into the ability map
    ability_map.insert(1, ability_1);
    ability_map.insert(2, ability_2);
    ability_map.insert(3, ability_3);
    ability_map.insert(4, ability_4);
    ability_map.insert(12, auto_attack);
  }

  //Casting Info
  let cast_queue = CastQueue::default();

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
    .with_component(player_state)?
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
    .with_component(cast_queue)?
    //Combat components
    .with_component(auto_attack_mesh)?
    .with_component(auto_attack_missle_speed)?
    // .with_component(cooldowns)?
    .with_component(attack_damage)?
    .with_component(ability_map)?
    .with_component(incoming_damage)?
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
