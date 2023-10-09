use gl::{Gl, types::{GLuint, GLint, GLvoid}};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct F32Tuple2{
  pub d0:  f32,
  pub d1:  f32,
}

impl F32Tuple2{
  pub fn new(d0:f32,d1:f32)->Self{
    F32Tuple2{d0,d1}
  }
}

impl From<(f32,f32)> for F32Tuple2{
  fn from(value: (f32,f32)) -> Self {
    F32Tuple2::new(value.0, value.1)
  }
}

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
}

impl From<(f32,f32,f32)> for F32Tuple3{
  fn from(value: (f32,f32,f32)) -> Self {
    F32Tuple3::new(value.0, value.1, value.2)
  }
}

#[derive(Copy, Clone, Debug)]
pub struct F32Tuple4{
  pub d0:  f32,
  pub d1:  f32,
  pub d2:  f32,
  pub d3:  f32,
}

impl F32Tuple4{
  pub fn new(d0:f32,d1:f32,d2:f32,d3:f32)->Self{
    F32Tuple4{d0,d1,d2,d3}
  }
}

impl From<(f32,f32,f32,f32)> for F32Tuple4{
  fn from(value: (f32,f32,f32,f32)) -> Self {
    F32Tuple4::new(value.0, value.1, value.2,value.3)
  }
}