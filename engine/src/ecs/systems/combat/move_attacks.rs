use crate::{ecs::{component_lib::{AutoAttack, Colliding, GameplayRadius, Position, Target, Velocity}, World}, math::Vec3, physics::circle_point_collision_test};
use eyre::Result;

///Moves all entities with the `AutoAttack` component forward each game logic tick. 
/// Marks auto attacks which reach their target with a `Colliding` component.
pub fn move_attacks(world:&mut World) ->Result<()>{
  let mut query = world.query();
  let entities = query.with_component::<AutoAttack>()?.run_entity();

  let mut colliding = Vec::default();

  for entity in entities{
    //Get position and velocity
    let mut position = entity.mut_get_component::<Position>()?;
    let velocity = entity.immut_get_component::<Velocity>()?;
    
    //Update the position
    let tick_start:Vec3 = position.tick_end;
    let tick_end:Vec3 = position.tick_end + velocity.0;
    let new_position = Position::new(tick_start, tick_end);
    
    *position = new_position;

    //Check if entities have reached their target.
    
    //Get the target information
    let target = entity.immut_get_component::<Target>()?;
    let target_position = world.immut_get_component_by_entity_id::<Position>(target.0)?;
    let target_radius = world.immut_get_component_by_entity_id::<GameplayRadius>(target.0)?;
    
    //Check if the attack is colliding with the target using a circle-point test
//I don't think I need to refetch the attack's position but double checks
    let collision_check = circle_point_collision_test(position.tick_end, target_position.tick_end, target_radius.0);

    //If the attack has hit its target, buffer the command to give it the Colliding component
    if collision_check {
      colliding.push(entity.id);
    }
  }

  //Loop through the buffered entity IDs and give them the Colliding component
  colliding.into_iter().for_each(|entity_id|world.add_component(entity_id, Colliding).unwrap());

  Ok(())
}