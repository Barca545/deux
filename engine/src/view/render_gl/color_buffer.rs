use gl::Gl;
use crate::math::gl_data::F32Tuple4;

pub struct ColorBuffer{
  pub color: F32Tuple4
}

impl ColorBuffer {
  pub fn from_color(red:f32,green:f32,blue:f32,alpha:f32) -> ColorBuffer {
    ColorBuffer {
      color: F32Tuple4::from((red,green,blue,alpha))
    }
  }

  pub fn update_color(&mut self, red:f32,green:f32,blue:f32,alpha:f32){
    self.color = F32Tuple4::from((red,green,blue,alpha));
  }
  
  pub fn set_used(&self, gl:&Gl){
    unsafe{
      gl.ClearColor(
        self.color.d0,
        self.color.d1,
        self.color.d2,
        self.color.d3
      );
    }
  }

  pub fn clear(&self,gl:&Gl){
    unsafe {
      gl.Clear(gl::COLOR_BUFFER_BIT);
    }
  }
}