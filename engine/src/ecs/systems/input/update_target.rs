use crate::{component_lib::{Controllable, Player, SelectionRadius, Target}, ecs::World, input::user_inputs::{FrameInputs, UserInput}, physics::ray_aabb3d_collision_test};

// Refactor
// -Merge and remove the selection logic. Just update the target. 
//  Set to some if there is something under the MouseRay, otherwise set to none
// -Don't need to handle checking the target team here, handle it in lower level logic
// -Needs to select the entity that is in front, not just the first found
// -Needs to not overwrite a true
// -See if adding a states reduces the need for shennanigans with not moving if there is a target


pub fn update_target(world: &World){
  let frame_inputs = world.get_resource_mut::<FrameInputs>().unwrap();
  if let Some(UserInput::MouseClick(mouse_ray)) = frame_inputs.get_input(){
    //Get the target of the controllable Player
    let mut query = world.query();
    let entities = query
      .with_component::<Player>().unwrap()
      .with_component::<Controllable>().unwrap()
      .run();
    let entity = &entities[0];
    let mut target = entity.mut_get_component::<Target>().unwrap();
    let mut new_target = Target(None);

    let mut query_targetables = world.query();
    let targetable_entities = query_targetables.with_component::<SelectionRadius>().unwrap().run();
    //Query all targetable entities. 
    //If the MouseRay is hitting an entity, update the Controllable Player's Target.
    for targetable_entity in targetable_entities {
      let hitbox = targetable_entity.immut_get_component::<SelectionRadius>().unwrap();
      let hit_check = ray_aabb3d_collision_test(hitbox.0, mouse_ray.0);
      if hit_check{
        new_target = Target(Some(targetable_entity.id));
        *target = new_target;
      }
      else if new_target.0 == None {
        *target = Target(None);
      }
    }
  }
}

// pub fn update_target(world:&mut World) {
//   if let CLICKED(selected_id) = world.immut_get_resource::<Selected>().unwrap() {
//     let selected_entity_team = world.immut_get_component_by_entity_id::<Team>(*selected_id).unwrap();
    
//     let mut query = world.query();
//     let entities = query.with_component::<Player>().unwrap().run();
//     let entity = &entities[0];
    
//     let team = entity.immut_get_component::<Team>().unwrap();
    
//     //Confirm the selection is *not* the same team as the entity, if so set it as the target
//     if *team != *selected_entity_team {
//       let mut target = entity.mut_get_component::<Target>().unwrap();
//       *target = Target(Some(*selected_id));
//     }
//   }
//   else {
//     let mut query = world.query();
//     let entities = query.with_component::<Player>().unwrap().run();
//     let entity = &entities[0];
  
//     let mut target = entity.mut_get_component::<Target>().unwrap();
//     *target = Target(None)
//   }
// }