use std::mem::size_of;

use super::data::F32Tuple3;
use gl::Gl;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
  pub pos: F32Tuple3,
  pub clr: F32Tuple3,
}

impl Vertex{
  pub fn vertex_attrib_pointers(gl:&Gl){
    let stride = size_of::<Self>();
    let location = 0;
    let offset = 0;
    //shape
    unsafe {
      F32Tuple3::vertex_attrib_pointer(gl, stride, location, offset)
    }

    let location = 1;
    let offset = offset + size_of::<F32Tuple3>();
    //color
    unsafe {
      F32Tuple3::vertex_attrib_pointer(gl, stride, location, offset)
    }
  }
}