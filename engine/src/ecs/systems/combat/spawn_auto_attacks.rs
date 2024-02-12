use crate::{component_lib::{AutoAttack, AutoAttackCooldown, AutoAttackMesh, AutoAttackScript, Destination, MissleSpeed, Owner, Player, Position, PreviousPosition, SkinnedMesh, Target, Timer, Velocity}, ecs::{command_buffer::AutoAttackSpawner, World}};

//Refactor: 
// -Load all the meshes into a resource and get a ref to them to speed up creation
// -Raise the height of the auto attacks might also require updating the move subsystem

///Loops through all entities with `Player` and `Target` components. 
/// For entities with no remaining auto attack `Cooldown`s, spawn an auto attack which moves towards the `Target` each game logic tick.
pub fn spawn_auto_attacks(world:&mut World) { 
  let mut spawner = AutoAttackSpawner::default();

  let mut query = world.query();
  
  let entities = query
    .with_component::<Player>().unwrap()
    .with_component::<Target>().unwrap()
    .run_entity();

  //Spawn an auto attack for every player entity with a target.
  for entity in entities{
    //Get the cooldown
    let mut cooldown = entity.mut_get_component::<AutoAttackCooldown>().unwrap();

    if cooldown.remaining()==0.0 {
      //Get the target
      let target = entity.immut_get_component::<Target>().unwrap();
      
      //Reset the cooldown after starting the attack spawning
      cooldown.reset();

      //Get the position
      let entity_position = entity.immut_get_component::<Position>().unwrap();
      
      //Build the attack's position and previous position
      let attack_position = Position(entity_position.0);
      let previous_attack_position = PreviousPosition(entity_position.0);

      //Get the missle speed
      let missle_speed = entity.immut_get_component::<MissleSpeed>().unwrap();
      
      //Get the target's position
      let destination = Destination::from(*world.immut_get_component_by_entity_id::<Position>(target.0).unwrap());
      
      //Calculate velocity
      let velocity = Velocity::new(&attack_position, &destination, &missle_speed.0);
      
      //Get the mesh info
      let auto_attack_mesh = entity.immut_get_component::<AutoAttackMesh>().unwrap();

      //Get the owner
      let owner = Owner(entity.id.clone());

      //Get the script's reference
      let script_ref = entity.get_commonent_ref::<AutoAttackScript>().unwrap();

      //Add all the values to the spawner
      spawner.add(attack_position, previous_attack_position,*missle_speed, velocity, auto_attack_mesh.clone(), owner, *target, script_ref);
    }
  }
  
  //Loop through the SpawnBuffer and spawn the attack
  for index in spawner.indices{
    //Create the auto attack's mesh
    let auto_attack_mesh = spawner.meshes[index].clone();
    let mesh = SkinnedMesh::from(auto_attack_mesh);

    //Create the auto attack entity
    world
    .create_entity()
    .with_component(AutoAttack::default()).unwrap()
    .with_component(spawner.positions[index]).unwrap()
    .with_component(spawner.previous_positions[index]).unwrap()
    .with_component(spawner.missle_speeds[index]).unwrap()
    .with_component(spawner.velocities[index]).unwrap()
    .with_component(mesh).unwrap()
    .with_component(spawner.owners[index]).unwrap()
    .with_component(spawner.targets[index]).unwrap()
    .with_component(spawner.scripts[index].clone()).unwrap();
  }
}