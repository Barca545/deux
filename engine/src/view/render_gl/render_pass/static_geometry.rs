use crate::{
  component_lib::{Position, StaticMesh}, ecs::{world_resources::ShaderPrograms, World}, math::{calculate_model_transform, math::Mat4, Vec3}
};
use gl::Gl;
use super::render_mesh::render_mesh;

pub fn static_geometry(world:&World) {
  let gl = world.get_resource::<Gl>().unwrap();
  let program = world.get_resource::<ShaderPrograms>().unwrap().normal;

  let mut query = world.query();

  let entities = query.with_component::<StaticMesh>().unwrap().with_component::<Position>().unwrap().run();

  for entity in entities {
    //Get the entity's position
    let position = entity.immut_get_component::<Position>().unwrap();
    let render_position:Vec3 = position.0;

    //Get the mesh
    let mesh = entity.immut_get_component::<StaticMesh>().unwrap();

    //Bind the model transform
    let model_transform:Mat4 = calculate_model_transform(&render_position, 1.1);
    
    //Set the model transform's value
    program.set_model_matrix(&gl, &model_transform);
    
    render_mesh(&gl, &mesh.0);
  }
}
