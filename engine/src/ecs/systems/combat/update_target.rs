use crate::{component_lib::{Player, Target, Team}, ecs::{world_resources::Selected::{self, CLICKED, HOVERED, NONE}, World}};

//Refactor
// -This needs to be updated to used CLICKED instead of hovered
// -Update the target to deselect the target when click to move instead of on hover
// -Create/use a command buffer to clean up the code.

pub fn update_target(world:&mut World) {
  //Get the selection
  let selection = world.immut_get_resource::<Selected>().unwrap();
  
  let mut target;
  let player_id;

  match selection {
    HOVERED(selected_id) => {
      target = None;
      let selected_entity_team = world.immut_get_component_by_entity_id::<Team>(*selected_id).unwrap();
    
      let mut query = world.query();
      let entities = query.with_component::<Player>().unwrap().run_entity();
      let entity = &entities[0];
      
      player_id = entity.id;
      
      let team = entity.immut_get_component::<Team>().unwrap();
      
      //check if the selection is *not* the same team as the entity, if so set it as the target
      if *team != *selected_entity_team {
        target = Some(Target(*selected_id));
      }
    },
    CLICKED(_selected_id) => {
      target = None;

      let mut query = world.query();
      let entities = query.with_component::<Player>().unwrap().run_entity();
      let entity = &entities[0];
      
      player_id = entity.id;
    },
    NONE => {
      target = None;
      let mut query = world.query();
      let entities = query.with_component::<Player>().unwrap().run_entity();
      let entity = &entities[0];
      
      player_id = entity.id;
    }
  }

  if let Some(target) = target {
    world.add_component(player_id, target).unwrap();
  }
  else {
    world.remove_component::<Target>(player_id).unwrap();
  }
}