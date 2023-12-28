use crate::ecs::{World, component_lib::{Target, Position, Velocity, MissleSpeed}};
use eyre::Result;

pub fn spawn_auto_attacks(world:&mut World) -> Result<()> {
  
  let mut query = world.query();
  
  let entities = query.with_component::<Target>()?.run_entity();

  //for every entity with a target spawn an auto attack
  for entity in entities{
    //check if there is a target
    let target = entity.immut_get_component::<Target>()?;
    if let Some(id) = target.0{
      //get the start position
      let position = entity.immut_get_component::<Position>()?;
      
      //get the missle speed
      let missle_speed = entity.immut_get_component::<MissleSpeed>()?;

      //get the target's position
      let destination = world.immut_get_component_by_entity_id::<Position>(id)?;
      
      //calculate velocity
      let velocity = Velocity::new(&position.tick_end, &destination.tick_end, &missle_speed.0);
    }
  }
  
  //get the mesh
  
  
  
  //spawn the attack
  Ok(())
}