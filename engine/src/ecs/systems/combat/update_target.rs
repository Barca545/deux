use crate::ecs::{World, world_resources::{Selected, Selected::{HOVERED,NONE}}, component_lib::{Target, Team, Player}};
use eyre::Result;

//this needs to be updated to used CLICKED instead of hovered
//this needs to add the target when clicked 
pub fn update_target(world:&mut World) -> Result<()>{
  //get the selection
  let selection = world.immut_get_resource::<Selected>().unwrap();
  if let HOVERED(selected_id) = selection {
    let selected_entity_team = world.immut_get_component_by_entity_id::<Team>(*selected_id)?;
    
    let mut query = world.query();

    let entities = query.with_component::<Player>()?.run_entity();
    for entity in entities{
      let team = entity.immut_get_component::<Team>()?;
      
      //check if the selection is *not* the same team as the entity, if so set it as the target
      if *team != *selected_entity_team {
        let target = Target(*selected_id);
      }
    }
  }
  else if let NONE = selection{
    let mut query = world.query();

    let entities = query.with_component::<Player>()?.run_entity();
    for entity in entities{
      //need to add a delete_component method to query entity
      // Remove the target component from the Player
      // entity
      world.add_component_to_entity_by_id(data, index)
    }
  }
  Ok(())
}