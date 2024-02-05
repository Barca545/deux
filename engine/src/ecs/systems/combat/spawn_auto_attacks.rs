use crate::ecs::{World, component_lib::{AutoAttack, AutoAttackCooldown, AutoAttackMesh, MissleSpeed, Owner, Player, Position, Scripts, SkinnedMesh, Target, Velocity}};
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
  targets:Vec<Target>
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
    target:Target
  ){
    let index = self.indices.len();
    self.indices.push(index);
    self.positions.push(position);
    self.missle_speeds.push(missle_speed);
    self.velocities.push(velocity);
    self.meshes.push(mesh);
    self.owners.push(owner);
    self.targets.push(target);
  }
}

pub fn spawn_auto_attacks(world:&mut World) -> Result<()> { 
  let mut spawner = AutoAttackSpawner::default();

  let mut query = world.query();
  
  let entities = query
    // .with_component::<Target>()?
    .with_component::<Player>()?
    .run_entity();

 
  //for every entity with a target spawn an auto attack
  for entity in entities{
    //get the cooldown
    let mut cooldown = entity.mut_get_component::<AutoAttackCooldown>()?;
   
    //check if there is a target
    let target = entity.immut_get_component::<Target>()?;

    if let Some(id) = target.0 {
      //confirm the attack cooldown has expired
      if cooldown.remaining==0.0 {
        //reset the cooldown after starting the attack spawning
        cooldown.remaining = cooldown.duration;

        //get the start position
        let position = entity.immut_get_component::<Position>()?;

        //get the missle speed
        let missle_speed = entity.immut_get_component::<MissleSpeed>()?;
        
        //get the target's position
        let destination = world.immut_get_component_by_entity_id::<Position>(id)?;
        
        //calculate velocity
        let velocity = Velocity::new(&position.tick_end, &destination.tick_end, &missle_speed.0);
        
        //get the mesh info
        let auto_attack_mesh = entity.immut_get_component::<AutoAttackMesh>()?;

        //get the owner
        let owner = Owner{id: entity.id.clone()};
        
        //add all the values to the spawner
        spawner.add(*position, *missle_speed, velocity, auto_attack_mesh.clone(), owner, Target(Some(id)));
      }
    }
  }
  
  //loop through the auto attacks and spawn the attack
  for index in spawner.indices{
    // create the mesh
    let auto_attack_mesh = spawner.meshes[index].clone();
    let mesh = SkinnedMesh::from(auto_attack_mesh);

    //in a final set up the scripts will need to be stored on the entity
    let script = r#"
      world:remove_health(target_id,100000)
    "#;
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
      .with_component(Scripts::new(vec![script]))?;
  }
  Ok(())
}