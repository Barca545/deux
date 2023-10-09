use gl::Gl;

//not sure I need Z since I am never letting ppl move the camera in or out
pub struct Viewport{
  pub x: i32,
  pub y: i32,
  //pub z: i32,
  pub h: i32,
  pub w: i32,
}

impl Viewport{
  pub fn for_window(h:i32,w:i32) -> Viewport {
    Viewport {
      x: 0,
      y: 0,
      //z: 0,
      h,
      w,
    }
  }

  pub fn update_size(&mut self, w:i32, h:i32){
    self.w = w;
    self.h = h;
  }

  pub fn set_used(&self, gl:&Gl){
    unsafe{
      gl.Viewport(self.x,self.y,self.w,self.h);
    }
  }
}