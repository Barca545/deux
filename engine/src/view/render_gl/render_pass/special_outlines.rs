// use crate::{
//   component_lib::{Position, PreviousPosition, SkinnedMesh},
//   ecs::{
//     world_resources::{
//       Selected::{self, HOVERED},
//       ShaderPrograms,
//     },
//     World,
//   },
//   math::{calculate_model_transform, Vec3},
// };
// use gl::{Gl, TRIANGLES};
// use glm::lerp;

// pub fn special_outlines(world: &World, interpolation_factor: f64) {
//   let gl = world.get_resource::<Gl>().unwrap();
//   let selection = world.get_resource::<Selected>().unwrap();

//   //probably need to make selected a vec and so still need a loop
//   match *selection {
//     HOVERED(id) => {
//       let mesh = world.get_component::<SkinnedMesh>(id).unwrap();
//       let program = world.get_resource::<ShaderPrograms>().unwrap().normal;

//       //Get the render position by lerping between the position at the end of the previous game logic tick and the position at the end of the current game logic tick
//       let position = world.get_component::<Position>(id).unwrap();
//       let previous_position = world.get_component::<PreviousPosition>(id).unwrap();
//       let render_position: Vec3 = lerp(&previous_position.0, &position.0, interpolation_factor as f32);

//       let texture = &mesh.mesh.texture;
//       let vao = &mesh.mesh.vao;

//       texture.bind(&gl);
//       vao.bind(&gl);

//       //Bind the model transform
//       let model_transform = calculate_model_transform(&render_position, 1.1);

//       //Set the model transform's value
//       program.set_model_matrix(&gl, &model_transform);

//       unsafe {
//         gl.DrawArrays(TRIANGLES, 0, 36);
//       }
//       vao.unbind(&gl);
//     }
//     _ => {}
//   }
// }
