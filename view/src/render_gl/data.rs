use std::mem::size_of;
use gl::types::GLint;

//maybe merge this into a seprate math lib for the whole project
//these can also be generalized using generics
//making a personal math lib may be unnessecary, we'll see

//replace the individual funct methods here with calls to this
trait GlMath<T>{
  fn size()->usize{
    size_of::<T>()
  }

  fn glint_size()->GLint{
    size_of::<T>() as GLint
  }
}

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

  pub fn usize()->usize{
    size_of::<F32Tuple2>()
  }

  pub fn glint_size()->GLint{
    size_of::<F32Tuple2>() as GLint
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
  
  pub fn size()->usize{
    size_of::<F32Tuple3>()
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

  pub fn size()->usize{
    size_of::<F32Tuple4>()
  }
}

impl From<(f32,f32,f32,f32)> for F32Tuple4{
  fn from(value: (f32,f32,f32,f32)) -> Self {
    F32Tuple4::new(value.0, value.1, value.2,value.3)
  }
}
