use gl::{Gl, TRIANGLES, UNSIGNED_INT, types::{GLsizei, GLvoid}};
use crate::view::mesh::Mesh;

//test by just doing the current render then start making it draw_elements
pub(crate) fn render_mesh(gl:&Gl, mesh:&Mesh){
  let texture = &mesh.texture;
  let vao = &mesh.vao;
  let indices = &mesh.indices;
  
  //do I bind texture before or after vao
  texture.bind(gl);
  vao.bind();
  
  //make a file just holding the draw mode functions

  // //bind the model transform
  // program.set_uniform_matrix4fv(gl, uniform_locations.model, &transforms.get_model_transform(&render_position, 1.0));
  unsafe {
    gl.DrawElements(
      TRIANGLES,
      indices.len() as GLsizei,
      UNSIGNED_INT,
      // null() as *const GLvoid
      indices.as_ptr() as *const GLvoid
      // 0 as *const GLvoid
    );
  }
  vao.unbind();
} 