use gl::{Gl, types::{GLuint, GLint, GLvoid}};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct F32Tuple3{
  pub d0:  f32,
  pub d1:  f32,
  pub d2:  f32,
}

impl F32Tuple3{
  pub fn new(d0:f32,d1:f32,d2:f32)->Self{
    F32Tuple3{d0,d1,d2}
  }

  pub unsafe fn vertex_attrib_pointer(gl:&Gl, stride:usize,location:usize, offset:usize){
    gl.EnableVertexAttribArray(location as GLuint);
    gl.VertexAttribPointer(
      location as GLuint,
      3, 
      gl::FLOAT,
      gl::FALSE, 
      stride as GLint,
      offset as *const GLvoid
    );
  }
}

impl From<(f32,f32,f32)> for F32Tuple3{
  fn from(value: (f32,f32,f32)) -> Self {
    F32Tuple3::new(value.0, value.1, value.2)
  }
}