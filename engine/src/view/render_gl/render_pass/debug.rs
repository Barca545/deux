// use crate::{
//   component_lib::{Position, PreviousPosition},
//   ecs::{world_resources::DbgShaderProgram, World},
//   math::Vec3,
//   view::mesh::AABB3DDebugMesh,
// };
// use gl::{Gl, FILL, FRONT, LINE, LINES};
// use glm::lerp;

// pub fn debug(world: &World, interpolation_factor: f64) {
//   let gl = world.get_resource::<Gl>().unwrap();
//   let program = world.get_resource::<DbgShaderProgram>().unwrap().program;

//   let mut query = world.query();
//   let entities = query.with_component::<AABB3DDebugMesh>().unwrap().with_component::<Position>().unwrap().run();

//   for entity in entities {
//     //Get the render position by lerping between the position at the end of the previous game logic tick and the position at the end of the current game logic tick
//     let position = entity.get_component::<Position>().unwrap();
//     let previous_position = entity.get_component::<PreviousPosition>().unwrap();
//     let render_position: Vec3 = lerp(&previous_position.0, &position.0, interpolation_factor as f32);

//     //Get the mesh and vao
//     let mesh = entity.get_component::<AABB3DDebugMesh>().unwrap();
//     let vao = &mesh.vao;

//     vao.bind(&gl);

//     //bind the model transform
//     // let model_transform = calculate_model_transform(&render_position, 1.1);
//     // program.set_model_matrix(gl, &model_transform);

//     unsafe {
//       gl.PolygonMode(FRONT, LINE);
//       gl.DrawArrays(LINES, 0, 36);
//       gl.PolygonMode(FRONT, FILL);
//     }
//     vao.unbind(&gl);
//   }
// }
