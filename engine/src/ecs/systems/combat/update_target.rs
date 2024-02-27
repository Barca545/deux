use crate::{component_lib::{Player, Target, Team}, ecs::{world_resources::Selected::{self, CLICKED}, World}};

//Refactor
// -This needs to be updated to used CLICKED instead of hovered
// -Update the target to deselect the target when click to move instead of on hover
// -This only needs to check for clicked no need to address the other selection possibilites 
// -Target can old an option? Then everything can fit inside the if let statement
//  I'm making it over complicated because I want to remove the target if there is no enemy selected but I can just keep the target

pub fn update_target(world:&mut World) {
  if let CLICKED(selected_id) = world.immut_get_resource::<Selected>().unwrap() {
    let selected_entity_team = world.immut_get_component_by_entity_id::<Team>(*selected_id).unwrap();
    
    let mut query = world.query();
    let entities = query.with_component::<Player>().unwrap().run();
    let entity = &entities[0];
    
    let team = entity.immut_get_component::<Team>().unwrap();
    
    //Confirm the selection is *not* the same team as the entity, if so set it as the target
    if *team != *selected_entity_team {
      let mut target = entity.mut_get_component::<Target>().unwrap();
      *target = Target(Some(*selected_id));
    }
  }
  else {
    let mut query = world.query();
    let entities = query.with_component::<Player>().unwrap().run();
    let entity = &entities[0];
  
    let mut target = entity.mut_get_component::<Target>().unwrap();
    *target = Target(None)
  }
}