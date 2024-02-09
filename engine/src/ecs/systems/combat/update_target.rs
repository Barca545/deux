use crate::ecs::{World, world_resources::{Selected, Selected::{HOVERED,CLICKED,NONE}}, component_lib::{Target, Team, Player}};
use eyre::Result;

//this needs to be updated to used CLICKED instead of hovered
//this needs to add the target when clicked 
//consider creating a command buffer to clean this up
pub fn update_target(world:&mut World) -> Result<()>{
  //Get the selection
  let selection = world.immut_get_resource::<Selected>().unwrap();
  
  let mut target;
  let player_id;

  match selection {
    HOVERED(selected_id) => {
      target = None;
      let selected_entity_team = world.immut_get_component_by_entity_id::<Team>(*selected_id)?;
    
      let mut query = world.query();
      let entities = query.with_component::<Player>()?.run_entity();
      let entity = &entities[0];
      
      player_id = entity.id;
      
      let team = entity.immut_get_component::<Team>()?;
      
      //check if the selection is *not* the same team as the entity, if so set it as the target
      if *team != *selected_entity_team {
        target = Some(Target(*selected_id));
      }
    },
    CLICKED(_selected_id) => {
      target = None;

      let mut query = world.query();
      let entities = query.with_component::<Player>()?.run_entity();
      let entity = &entities[0];
      
      player_id = entity.id;
    },
    NONE => {
      target = None;
      let mut query = world.query();
      let entities = query.with_component::<Player>()?.run_entity();
      let entity = &entities[0];
      
      player_id = entity.id;
    }
  }

  if let Some(target) = target {
    world.add_component_to_entity_by_id(player_id, target)?;
  }
  else {
    world.delete_component_by_entity_id::<Target>(player_id)?;
  }

  Ok(())
}