use crate::ecs::{World, component_lib::{Target, Position, Velocity, MissleSpeed, SkinnedMesh, AutoAttackCooldown, AutoAttack, AutoAttackMesh}};
use eyre::Result;

//maybe this could be a resource but might be unnessecary 
pub struct AutoAttackSpawner{
  indices:Vec<usize>,
  positions:Vec<Position>,
  missle_speeds:Vec<MissleSpeed>,
  velocities:Vec<Velocity>,
  // meshes:Vec<AutoAttackMeshCreator>
  meshes:Vec<AutoAttackMesh>
}

impl Default for AutoAttackSpawner{
  fn default() -> Self {
    Self { 
      indices: Default::default(),
      positions: Default::default(),
      missle_speeds: Default::default(),
      velocities: Default::default(),
      meshes: Default::default() 
    }
  }
}

impl AutoAttackSpawner{
  pub fn add(&mut self, position:Position,missle_speed:MissleSpeed,velocity:Velocity,mesh:AutoAttackMesh){
    let index = self.indices.len();
    self.indices.push(index);
    self.positions.push(position);
    self.missle_speeds.push(missle_speed);
    self.velocities.push(velocity);
    self.meshes.push(mesh);
  }
}

pub fn spawn_auto_attacks(world:&mut World) -> Result<()> {
  
  let mut spawner = AutoAttackSpawner::default();

  let mut query = world.query();
  
  let entities = query.with_component::<Target>()?.run_entity();

  //for every entity with a target spawn an auto attack
  for entity in entities{
    //reset the cooldown
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
        spawner.add(position.clone(), *missle_speed, velocity, auto_attack_mesh.clone());
        
      }
    }
  }
  
  //loop through the auto attacks and spawn the attack
  for index in spawner.indices{
    
    // create the mesh
    let auto_attack_mesh = spawner.meshes[index].clone();
    let mesh = SkinnedMesh::from(auto_attack_mesh);

    //create the entity
    world
      .create_entity()
      .with_component(spawner.positions[index])?
      .with_component(spawner.missle_speeds[index])?
      .with_component(spawner.velocities[index])?
      .with_component(mesh)?
      .with_component(spawner.positions[index])?
      .with_component(AutoAttack::default())?;
  }
  Ok(())
}

//I figured out why the balls kept spawing on the guy, I never un select the thing during movement