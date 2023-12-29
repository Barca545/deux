use super::define_attributes::define_vertex_attrib_pointer;
use gl::Gl;
use std::mem::size_of;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct UntexturedVertex {
  pos:[f32;3],
  // color:[f32;3]
}

impl From<(f32, f32, f32)> for UntexturedVertex {
  fn from(value:(f32, f32, f32)) -> Self {
    let pos:[f32;3] = [value.0, value.1, value.2];
    Self::new(pos)
  }
}

impl UntexturedVertex {
  pub fn new(pos:[f32;3]) -> Self {
    UntexturedVertex { pos }
  }

  ///Defines attribute pointers for an untextured vertex.
  pub fn init_attrib_pointers(gl:&Gl) {
    let stride = size_of::<Self>();

    //shape
    let position = 0;
    let position_offset = 0;
    define_vertex_attrib_pointer(gl, stride, position, position_offset, 3);
  }
}
