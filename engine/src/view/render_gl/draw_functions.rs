use crate::view::Mesh;
use gl::{
  types::{GLsizei, GLvoid},
  Gl, TRIANGLES, UNSIGNED_INT,
};

// Refactor:
// -Should texture be bound before or after the VAO
// -Need draw quad function for UI elements?

///Draw the provided [`Mesh`].
/// Uses `DrawElements` in `GL_TRIANGLES` mode.
pub fn draw_elements(gl: &Gl, mesh: &Mesh) {
  let texture = &mesh.texture;
  let vao = &mesh.vao;
  let indices = &mesh.indices;

  texture.bind(gl);
  vao.bind(gl);

  unsafe {
    gl.DrawElements(TRIANGLES, indices.len() as GLsizei, UNSIGNED_INT, indices.as_ptr() as *const GLvoid);
  }
  vao.unbind(gl);
}
