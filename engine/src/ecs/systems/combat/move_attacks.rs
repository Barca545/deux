use crate::{component_lib::{AutoAttack, Colliding, GameplayRadius, Position, PreviousPosition, Target, Velocity}, ecs::World, physics::circle_point_collision_test};

///Moves all entities with the `AutoAttack` component forward each game logic tick. 
/// Marks auto attacks which reach their target with a `Colliding` component.
pub fn move_attacks(world:&mut World) {
  let mut query = world.query();
  let entities = query.with_component::<AutoAttack>().unwrap().run();

  let mut colliding = Vec::default();

  for entity in entities{
    //Get position and velocity
    let mut previous_position = entity.mut_get_component::<PreviousPosition>().unwrap();
    let mut position = entity.mut_get_component::<Position>().unwrap();
    let velocity = entity.immut_get_component::<Velocity>().unwrap();
    
    //Update the positions
    let new_previous_position = PreviousPosition(position.0);
    let new_position = Position(position.0 + velocity.0);
    *previous_position = new_previous_position;
    *position = new_position;

    //Check if entities have reached their target.
    
    //Get the target information
    let target = entity.immut_get_component::<Target>().unwrap();
    let target_position = world.immut_get_component_by_entity_id::<Position>(target.0.unwrap()).unwrap();
    let target_radius = world.immut_get_component_by_entity_id::<GameplayRadius>(target.0.unwrap()).unwrap();
    
    //Check if the attack is colliding with the target using a circle-point test
//I don't think I need to refetch the attack's position but double checks
    let collision_check = circle_point_collision_test(position.0, target_position.0, target_radius.0);

    //If the attack has hit its target, buffer the command to give it the Colliding component
    if collision_check {
      colliding.push(entity.id);
    }
  }

  //Loop through the buffered entity IDs and give them the Colliding component
  colliding.into_iter().for_each(|entity_id|world.add_component(entity_id, Colliding).unwrap());
}