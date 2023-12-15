use crate::math::math::Vec3;

use super::define_attributes::define_vertex_attrib_pointer;
use gl::Gl;
use std::mem::size_of;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct DebugVertex {
  pos:Vec3,
}

impl From<(f32, f32, f32)> for DebugVertex {
  fn from(value:(f32, f32, f32)) -> Self {
    let pos:Vec3 = Vec3::new(value.0, value.1, value.2);
    Self::new(pos)
  }
}

impl DebugVertex {
  pub fn new(pos:Vec3) -> Self {
    DebugVertex { pos }
  }

  pub fn init_attrib_pointers(gl:&Gl) {
    let stride = size_of::<Self>();
    
    //shape
    let position = 0;
    let position_offset = 0;
    define_vertex_attrib_pointer(gl, stride, position, position_offset, 3); 
  }
}

