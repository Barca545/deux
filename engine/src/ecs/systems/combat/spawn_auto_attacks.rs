use crate::{component_lib::{AutoAttack, AutoAttackCooldown, AutoAttackMesh, AutoAttackScript, Destination, MissleSpeed, Owner, Player, Position, PreviousPosition, SkinnedMesh, Target, Timer, Velocity}, ecs::World};

//Refactor: 
// -Load all the meshes into a resource and get a ref to them to speed up creation
// -Raise the height of the auto attacks might also require updating the move subsystem to ignore the y component

///Loops through all entities with `Player` and `Target` components. 
/// For entities with no remaining auto attack `Cooldown`s, spawn an auto attack which moves towards the `Target` each game logic tick.
pub fn spawn_auto_attacks(world:&mut World) { 
  let mut spawner = vec![];

  let mut query = world.query();
  
  let entities = query
    .with_component::<Player>().unwrap()
    .with_component::<Target>().unwrap()
    .run();

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
      spawner.push((AutoAttack::default(), attack_position, previous_attack_position,*missle_speed, velocity, SkinnedMesh::from(auto_attack_mesh.clone()), owner, *target, script_ref));
    }
  }
  
  //Loop through the buffered auto attacks and spawn
  for attack in spawner {
    world.create_entity().with_components(attack).unwrap();
  }
}