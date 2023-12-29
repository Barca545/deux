use crate::{ecs::{World, component_lib::{Position, Velocity, AutoAttack}}, math::Vec3};
use eyre::Result;

pub fn move_attacks(world:&World) ->Result<()>{
  let mut query = world.query();

  let entities = query.with_component::<AutoAttack>()?.run_entity();

  for entity in entities{
    //get position and velocity
    let mut position = entity.mut_get_component::<Position>()?;
    let velocity = entity.immut_get_component::<Velocity>()?;
    
    //update the position
    let tick_start:Vec3 = position.tick_end;
    let tick_end:Vec3 = position.tick_end + velocity.0;
  
    let new_position = Position::new(tick_start, tick_end);
    
    *position = new_position;
  }
  Ok(())
}