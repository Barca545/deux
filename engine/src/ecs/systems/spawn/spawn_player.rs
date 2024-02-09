use eyre::Result;
use gl::Gl;

use crate::{ecs::{World, component_lib::{AutoAttackMesh, AutoAttackScript, Controllable, Destination, Exp, Gold, Level, Player, Position, SelectionRadius, SkinnedMesh, Team, Velocity, KDA}, world_resources::DebugElements}, math::Vec3, filesystem::{load_object, load_champion_json}, view::AABB3DDebugMesh};
///spawns a player from a given name and player number
//possibly should be a system not a method on world
//might need to also take in the player number
pub fn spawn_player(world:&mut World, name:&str, number:u32) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap();
  
  //load player information JSON
  let champion_info = load_champion_json(name)?;

  //create the player entity 
  //basic info
  let player = Player(number);
  //the controllable flag information needs to get passed in from somewhere else
  let controllable = Controllable;
  let health = champion_info.health;
  //the team information needs to get passed in from somewhere else
  let team = Team::BLUE;
  let gold = Gold::default();
  let kda = KDA::default();
  let exp = Exp::default();
  let level = Level::default();

  //movement and collision info
  //the start position info needs to get passed in from somehwere else or depend on the player number
  let position_vec:Vec3 = Vec3::new(0.0, 0.0, 0.0);
  let position = Position::new(position_vec, position_vec);
  let destination = Destination::from(position_vec);
  let speed = champion_info.speed;
  let velocity = Velocity::default();
  let selection_radius = SelectionRadius::new(position_vec, champion_info.selection_radius.height, champion_info.selection_radius.radius);  
  let pathing_radius = champion_info.pathing_radius;

  //render info
  let (sprite_vertices, sprite_indices) = load_object(name)?;
  let player_mesh = SkinnedMesh::new(&gl,sprite_vertices,sprite_indices,"blank_texture", 1.0);
  //have some flag so this only spawns if in debug mode
  //also with_component can be used on world and affect the same entity until a new entity is spawned and incriments the index
  let player_hitbox_mesh = AABB3DDebugMesh::new(&gl, selection_radius.0, position_vec);

  //combat info 
  let (auto_attack_vertices, auto_attack_indices) = load_object("ball")?;
  let auto_attack_mesh = AutoAttackMesh::new(&gl, auto_attack_vertices, auto_attack_indices, "allied_attack", 0.5);
  let auto_attack_missle_speed = champion_info.auto_attack_missle_speed;
  let auto_attack_cooldown = champion_info.auto_attack_cooldown;
  let attack_damage = champion_info.attack_damage;
  //add armor to JSON
  
  world
    .create_entity()
    //general components
    .with_component(player)?
    .with_component(controllable)?
    .with_component(health)?
    .with_component(team)?
    .with_component(gold)?
    .with_component(kda)?
    .with_component(exp)?
    .with_component(level)?
    
    //movement and collision components
    .with_component(position)?
    .with_component(destination)?
    .with_component(speed)?
    .with_component(velocity)?
    .with_component(selection_radius)?
    .with_component(pathing_radius)?
    
    //combat components
    .with_component(auto_attack_mesh)?
    .with_component(auto_attack_missle_speed)?
    .with_component(auto_attack_cooldown)?
    .with_component(attack_damage)?
    
    //render components
    .with_component(player_mesh)?
    .with_component(player_hitbox_mesh)?

    //scripts
    .with_component(AutoAttackScript::new(r#"
    attack_damage = world:get_attack_damage(owner.id)
    world:remove_health(target.id,attack_damage)
  "#))?;
  
  let debug = world.immut_get_resource::<DebugElements>().unwrap();
  if debug.aabb {
    //update this to only include the debug stuff if debug elements is true
  }

  // if controllable {
  //   // add the controllable component only if controllable on the JSON is true
  // }
  
  Ok(())
}