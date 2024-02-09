use crate::ecs::{component_lib::{AutoAttack, AutoAttackCooldown, AutoAttackMesh, MissleSpeed, Owner, Player, Position, AutoAttackScript, SkinnedMesh, Target, Velocity}, query::ComponentRef, World};
use eyre::Result;

//maybe this could be a resource but might be unnessecary 
#[derive(Debug, Clone)]
pub struct AutoAttackSpawner{
  indices:Vec<usize>,
  positions:Vec<Position>,
  missle_speeds:Vec<MissleSpeed>,
  velocities:Vec<Velocity>,
  meshes:Vec<AutoAttackMesh>,
  owners:Vec<Owner>,
  targets:Vec<Target>,
  scripts: Vec<ComponentRef<AutoAttackScript>>,
}

impl Default for AutoAttackSpawner{
  fn default() -> Self {
    Self { 
      indices: Default::default(),
      positions: Default::default(),
      missle_speeds: Default::default(),
      velocities: Default::default(),
      meshes: Default::default(),
      owners: Default::default(),
      targets: Default::default(),
      scripts: Default::default(),
    }
  }
}

impl AutoAttackSpawner{
  pub fn add(
    &mut self, 
    position:Position,
    missle_speed:MissleSpeed,
    velocity:Velocity,
    mesh:AutoAttackMesh,
    owner:Owner,
    target:Target,
    script:ComponentRef<AutoAttackScript>,
  ){
    let index = self.indices.len();
    self.indices.push(index);
    self.positions.push(position);
    self.missle_speeds.push(missle_speed);
    self.velocities.push(velocity);
    self.meshes.push(mesh);
    self.owners.push(owner);
    self.targets.push(target);
    self.scripts.push(script);
  }
}

pub fn spawn_auto_attacks(world:&mut World) -> Result<()> { 
  let mut spawner = AutoAttackSpawner::default();

  let mut query = world.query();
  
  let entities = query
    .with_component::<Player>()?
    .with_component::<Target>()?
    .run_entity();

 
  //for every entity with a target spawn an auto attack
  for entity in entities{
    //get the cooldown
    let mut cooldown = entity.mut_get_component::<AutoAttackCooldown>()?;
   
    //check if there is a target
    let target = entity.immut_get_component::<Target>()?;

    if cooldown.remaining==0.0 {
      //reset the cooldown after starting the attack spawning
      cooldown.remaining = cooldown.duration;

      //get the start position
      let position = entity.immut_get_component::<Position>()?;

      //get the missle speed
      let missle_speed = entity.immut_get_component::<MissleSpeed>()?;
      
      //get the target's position
      let destination = world.immut_get_component_by_entity_id::<Position>(target.0)?;
      
      //calculate velocity
      let velocity = Velocity::new(&position.tick_end, &destination.tick_end, &missle_speed.0);
      
      //get the mesh info
      let auto_attack_mesh = entity.immut_get_component::<AutoAttackMesh>()?;

      //get the owner
      let owner = Owner(entity.id.clone());

      //Get the script's reference
      let script_ref = entity.get_commonent_ref::<AutoAttackScript>()?;

      //add all the values to the spawner
      spawner.add(*position, *missle_speed, velocity, auto_attack_mesh.clone(), owner, *target, script_ref);
    }
  }
  
  //loop through the auto attacks and spawn the attack
  for index in spawner.indices{
    // create the mesh
    let auto_attack_mesh = spawner.meshes[index].clone();
    let mesh = SkinnedMesh::from(auto_attack_mesh);

    //shift all the logic of the attack to scripts
      //have them fetch the damage from the player
    //store the scripts on the owner entity
    //event system
    //when attack connects, 
      // run script world.get_by_id 
      // feed that into the lua VM

    //make a script generate the attack

    //create the entity
    world
      .create_entity()
      .with_component(AutoAttack::default())?
      .with_component(spawner.positions[index])?
      .with_component(spawner.missle_speeds[index])?
      .with_component(spawner.velocities[index])?
      .with_component(mesh)?
      .with_component(spawner.owners[index])?
      .with_component(spawner.targets[index])?
      //ideally get rid of clone here
      .with_component(spawner.scripts[index].clone())?;
  }
  Ok(())
}