use crate::view::mesh::Mesh;
use gl::{
  types::{GLsizei, GLvoid},
  Gl, TRIANGLES, UNSIGNED_INT,
};

// Refactor:
// -Should texture be bound before or after the VAO
// -Need draw quad function for UI elements?

//I am debating making a renderable trait. 
// This will allow me to have different implementations for drawing without breaking 
// the game everytime I modify one

///Draw the provided [`Mesh`].
/// Uses [`DrawElements`](https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDrawElements.xhtml) in `GL_TRIANGLES` mode.
pub fn draw_indexed_primative(gl: &Gl, mesh: &Mesh) {
  let texture = &mesh.texture();
  let vao = &mesh.vao;
  let indices = &mesh.indices;

  texture.bind(gl);
  vao.bind(gl);

  unsafe {
    gl.DrawElements(TRIANGLES, indices.len() as GLsizei, UNSIGNED_INT, indices.as_ptr() as *const GLvoid);
  }
  vao.unbind(gl);
}
