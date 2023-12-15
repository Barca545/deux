use gl::{
  types::{GLenum, GLuint},
  Gl, DRAW_FRAMEBUFFER, FRAMEBUFFER, FRAMEBUFFER_COMPLETE, FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
  FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER, FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
  FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT, FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
  FRAMEBUFFER_INCOMPLETE_READ_BUFFER, FRAMEBUFFER_UNDEFINED, INVALID_ENUM, INVALID_OPERATION, TEXTURE_2D
};

use eyre::Result;

use crate::errors::FramebufferErrors;

pub struct FrameBuffer {
  gl:Gl,
  buffer_obj:GLuint // _marker: PhantomData<B>
}

impl FrameBuffer {
  ///Generates a new framebuffer object.
  pub fn new(gl:&Gl) -> Self {
    let gl = gl.clone();
    //should this be 0? 0 is the default framebuffer. Maybe I want it to be another
    // number?
    let mut buffer_obj:GLuint = 0;
    unsafe { gl.GenFramebuffers(1, &mut buffer_obj) }
    dbg!(buffer_obj);
    FrameBuffer { gl, buffer_obj }
  }

  ///Returns the Buffer object's `GLuint` value.
  pub fn get_buffer_obj(&self) -> GLuint {
    self.buffer_obj
  }

  ///Binds the framebuffer. Useful for off-screen rendering.
  pub fn bind(&self) {
    unsafe {
      self.gl.BindFramebuffer(FRAMEBUFFER, self.buffer_obj);
    }
  }

  ///Binds the default framebuffer object and unbinds any textures.
  pub fn bind_default(&self) {
    unsafe {
      self.gl.BindTexture(TEXTURE_2D, 0);
      self.gl.BindFramebuffer(FRAMEBUFFER, 0)
    }
  }

  ///Deletes the framebuffer object.
  pub fn delete(&self) {
    unsafe { self.gl.DeleteFramebuffers(1, self.buffer_obj as *const GLuint) }
  }

  ///Sets the framebuffer render target to the current framebuffer object.
  pub fn enable_writing(&self) {
    unsafe { self.gl.BindFramebuffer(DRAW_FRAMEBUFFER, self.buffer_obj) }
  }

  ///Sets the framebuffer render target back to default.
  pub fn disable_writing(&self) {
    unsafe { self.gl.BindFramebuffer(DRAW_FRAMEBUFFER, 0) }
  }

  pub fn attach_2d_texture(&self, attachment:GLenum, texture_obj:GLuint) {
    unsafe {
      self
        .gl
        .FramebufferTexture2D(FRAMEBUFFER, attachment, TEXTURE_2D, texture_obj, 0);
    }
  }

  ///Checks whether the current framebuffer has successfully completed, returns
  /// an error otherwise.
  pub fn check_framebuffer_status(&self) -> Result<()> {
    unsafe {
      let status:GLenum = self.gl.CheckFramebufferStatus(FRAMEBUFFER);
      match status {
        FRAMEBUFFER_COMPLETE => return Ok(()),
        FRAMEBUFFER_UNDEFINED => return Err(FramebufferErrors::GL_FRAMEBUFFER_UNDEFINED.into()),
        FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
          return Err(FramebufferErrors::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT.into())
        }
        FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
          return Err(FramebufferErrors::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT.into())
        }
        FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
          return Err(FramebufferErrors::GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER.into())
        }
        FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
          return Err(FramebufferErrors::GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS.into())
        }
        FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
          return Err(FramebufferErrors::GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER.into())
        }
        FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
          return Err(FramebufferErrors::GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE.into())
        }
        INVALID_ENUM => return Err(FramebufferErrors::GL_INVALID_ENUM.into()),
        INVALID_OPERATION => return Err(FramebufferErrors::GL_INVALID_OPERATION.into()),
        _ => unreachable!()
      }
    }
  }
}
