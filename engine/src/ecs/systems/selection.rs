// use crate::{
//   component_lib::SelectionRadius, ecs::{
//     world_resources::{
//       ScreenDimensions, Selected::{self, CLICKED, HOVERED, NONE}
//     },
//     World
//   }, input::user_inputs::{FrameInputs, UserInput}, math::{MouseRay, Transforms}, physics::ray_aabb3d_collision_test
// };

// // Refactor: 
// // -The hovered thing needs to updated during a render but the clicked does not. It can take a callback

// fn update_hovered(world:&mut World, x:f64, y:f64) {
//   let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
//   let transforms = world.immut_get_resource::<Transforms>().unwrap();

//   //should mouse ray be a resource? Probably not unless I find another way to get
//   // the transforms to it.
//   let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);

//   let mut selection_state = NONE;

//   let mut query = world.query();
//   let entities = query.with_component::<SelectionRadius>().unwrap().run();
  
//   for entity in entities {
//     let hitbox = entity.immut_get_component::<SelectionRadius>().unwrap();
//     let hit_check = ray_aabb3d_collision_test(hitbox.0, mouse_ray.0);
//     if hit_check == true {
//       selection_state = HOVERED(entity.id)
//     }
//   }

//   let selection = world.mut_get_resource::<Selected>().unwrap();
//   *selection = selection_state;
// }

// pub fn update_clicked(world:&mut World){
//   let frame_inputs = world.mut_get_resource::<FrameInputs>().unwrap();
//     if let Some(UserInput::MouseClick(mouse_ray)) = frame_inputs.get_input(){
//       let mut query = world.query();
//       let entities = query.with_component::<SelectionRadius>().unwrap().run();
    
//     let mut selection_state = NONE;

//     for entity in entities {
//       let hitbox = entity.immut_get_component::<SelectionRadius>().unwrap();
//       let hit_check = ray_aabb3d_collision_test(hitbox.0, mouse_ray.0);
//       if hit_check == true {
//         selection_state = CLICKED(entity.id)
//       }
//     }

//     let selection = world.mut_get_resource::<Selected>().unwrap();
//     *selection = selection_state;
//   }
// }

// pub fn update_selection(world:&mut World) {
//   // update_hovered(world, x, y);
//   update_clicked(world);
// }
