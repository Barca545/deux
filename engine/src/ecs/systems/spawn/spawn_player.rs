use crate::{
  component_lib::{
    AbilityInfo, AbilityMap, CastQueue, Controllable, Destination, Exp, Gold, Health, IncomingDamage, Level, MissleSpeed, Path, PhysicalDamage, Player,
    PlayerModel, PlayerState, Position, PreviousPosition, Script, SelectionRadius, SpellResource, Target, Team, UnitSpeed, Velocity, KDA,
  },
  ecs::{world_resources::DebugElements, World},
  filesystem::load_champion_json,
  math::Vec3,
  time::ServerTime,
  view::Renderer,
};

// Refactor
// -Missing some component the combat system needs
// -Add the scripts as something that gets loaded in
// -Make spawn location based on player number
// -Add command buffer so I can set up the situational components
// -Add armor to JSON
// -Controllable flag information needs to get passed in from somewhere else
// -Team information needs to get passed in from somewhere else

///Spawns a player from a given champion name and player number.
pub fn spawn_player(world: &mut World, name: &str, number: u32, renderer: &mut Renderer) {
  //Load player information JSON
  let champion_info = load_champion_json(name).unwrap();

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
  let gameplay_radius = champion_info.gameplay_radius;
  let pathing_radius = champion_info.pathing_radius;
  let path = Path::new();

  //Render info
  let player_model = PlayerModel(renderer.add_model(name));

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
    .with_component(player)
    .unwrap()
    .with_component(controllable)
    .unwrap()
    .with_component(health)
    .unwrap()
    .with_component(team)
    .unwrap()
    .with_component(target)
    .unwrap()
    .with_component(gold)
    .unwrap()
    .with_component(kda)
    .unwrap()
    .with_component(exp)
    .unwrap()
    .with_component(level)
    .unwrap()
    .with_component(player_state)
    .unwrap()
    //Movement and collision components
    .with_component(position)
    .unwrap()
    .with_component(previous_position)
    .unwrap()
    .with_component(destination)
    .unwrap()
    .with_component(speed)
    .unwrap()
    .with_component(velocity)
    .unwrap()
    .with_component(selection_radius)
    .unwrap()
    .with_component(gameplay_radius)
    .unwrap()
    .with_component(pathing_radius)
    .unwrap()
    .with_component(path)
    .unwrap()
    //Casting components
    .with_component(spell_resource)
    .unwrap()
    .with_component(cast_queue)
    .unwrap()
    //Combat components
    .with_component(auto_attack_missle_speed)
    .unwrap()
    .with_component(attack_damage)
    .unwrap()
    .with_component(ability_map)
    .unwrap()
    .with_component(incoming_damage)
    .unwrap()
    //Render components
    .with_component(player_model)
    .unwrap();

  let debug = world.get_resource::<DebugElements>().unwrap();
  if debug.aabb {
    //if true add the dbg mesh
  }

  // if controllable {
  //   // add the controllable component only if controllable on the JSON is true
  // }
}
