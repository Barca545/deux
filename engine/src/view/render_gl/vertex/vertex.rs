use crate::math::math::{Vec2, Vec3};
use gl::Gl;
use std::mem::size_of;

use super::define_attributes::define_vertex_attrib_pointer;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
  pos:Vec3,
  txt:Vec2
}

//could do multiple from impls and make txt and option
impl From<(f32, f32, f32, f32, f32)> for Vertex {
  fn from(value:(f32, f32, f32, f32, f32)) -> Self {
    let pos:Vec3 = Vec3::new(value.0, value.1, value.2);
    let txt:Vec2 = Vec2::new(value.3, value.4);
    Self::new(pos, txt)
  }
}

impl Vertex {
  pub fn new(pos:Vec3, txt:Vec2) -> Self {
    Vertex { pos, txt }
  }

  //take this out of the impl block and make independent function
  //could also be on the buffer
  pub fn init_attrib_pointers(gl:&Gl) {
    let stride = size_of::<Self>();
    
    //shape
    let position = 0;
    let position_offset = 0;
    define_vertex_attrib_pointer(gl, stride, position, position_offset, 3);

    //texture
    let size = size_of::<Vec3>();
    let texture = 1;
    let texture_offset = position_offset + size;
    define_vertex_attrib_pointer(gl, stride, texture, texture_offset, 2);
  }
}
