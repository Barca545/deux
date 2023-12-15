use gl::Gl;

pub struct DepthBuffer;
impl DepthBuffer {
  pub fn clear(gl:&Gl) {
    unsafe {
      gl.Clear(gl::DEPTH_BUFFER_BIT);
    }
  }
}
