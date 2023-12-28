use crate::ecs::{World, world_resources::{Selected, Selected::HOVERED}, component_lib::{Target, Team}};
use eyre::Result;

//this needs to be updated to used CLICKED instead of hovered
pub fn update_target(world:&World) -> Result<()>{
  //get the selection
  let selection = world.immut_get_resource::<Selected>().unwrap();
  if let HOVERED(selected_id) = selection {
    let selected_entity_team = world.immut_get_component_by_entity_id::<Team>(*selected_id)?;
    
    let mut query = world.query();

    let entities = query.with_component::<Target>()?.run_entity();
    for entity in entities{
      let team = entity.immut_get_component::<Team>()?;
      let mut target = entity.mut_get_component::<Target>()?;
      
      //check if the selection is *not* the same team as the entity, if so set it as the target
      if *team != *selected_entity_team {
        *target = Target(Some(*selected_id));
      }
    }
  }
  Ok(())
}