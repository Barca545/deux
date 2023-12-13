use crate::math::{
  gl_data::{F32Tuple2, F32Tuple3},
  math::{Vec2, Vec3},
};
use gl::{
  types::{GLint, GLuint, GLvoid},
  Gl, FALSE, FLOAT,
};
use std::mem::size_of;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
  pos: Vec3,
  txt: Vec2,
}

impl From<(f32, f32, f32, f32, f32)> for Vertex {
  fn from(value: (f32, f32, f32, f32, f32)) -> Self {
    let pos = Vec3::new(value.0, value.1, value.2);
    let txt = Vec2::new(value.3, value.4);
    Self::new(pos, txt)
  }
}
impl Vertex {
  pub fn new(pos: Vec3, txt: Vec2) -> Self {
    Vertex { pos, txt }
  }

  pub fn init_attrib_pointers(gl: &Gl) {
    let stride = size_of::<Self>();
    let size = size_of::<Vec3>();
    //shape
    let position = 0;
    let position_offset = 0;
    unsafe {
      define_vertex_attrib_pointer(gl, stride, position, position_offset, 3);
    }

    //texture
    let texture = 2;
    let texture_offset = position_offset + size;
    unsafe {
      define_vertex_attrib_pointer(gl, stride, texture, texture_offset, 2);
    }

    unsafe fn define_vertex_attrib_pointer(
      gl: &Gl, stride: usize, location: usize, offset: usize, tuple_size: GLint,
    ) {
      //why does GITGD (https://github.com/amengede/OpenGL-for-Beginners/blob/main/week%2006%20design%20patterns/4%20entity%20component%20system/src/controller/app.cpp#L12)
      //have EnableVertexAttribArray after VertexAttribPointer?
      gl.EnableVertexAttribArray(location as GLuint);
      gl.VertexAttribPointer(
        location as GLuint,
        tuple_size,
        FLOAT,
        FALSE,
        stride as GLint,
        offset as *const GLvoid,
      );
    }
  }
}
