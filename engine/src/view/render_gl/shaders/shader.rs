use std::{ffi::CString, ptr::null_mut};

use crate::{
  errors::ShaderErrors,
  filesystem::{create_whitespace_cstring, load_shader},
};
use eyre::Result;
use gl::{
  types::{GLchar, GLenum, GLint, GLuint},
  Gl, COMPILE_STATUS,
};

#[derive(Debug, Clone, Copy)]
pub struct Shader {
  pub id: GLuint,
}

impl Shader {
  //Find ways to convert the result into a custom error
  pub fn new(gl: &Gl, path: &str, kind: GLenum) -> Result<Self> {
    let source;
    let id;

    source = load_shader(path).unwrap();
    id = Self::compile_shader(gl, kind, source)?;
    Ok(Shader { id })
  }

  ///Compiles a new [`Shader`] using [GetShaderInfoLog](https://registry.khronos.org/OpenGL-Refpages/es2.0/xhtml/glGetShaderInfoLog.xml).
  fn compile_shader(gl: &Gl, kind: GLenum, source: CString) -> Result<GLuint> {
    let id: GLuint;
    let mut compile_status: GLint = 1;

    unsafe {
      id = gl.CreateShader(kind);
      gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
      gl.CompileShader(id);
      gl.GetShaderiv(id, COMPILE_STATUS, &mut compile_status)
    }

    if compile_status == 0 {
      let mut len: GLint = 0;
      unsafe { gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) }

      let error: CString = create_whitespace_cstring(len as usize);

      unsafe { gl.GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar) }

      return Err(
        ShaderErrors::ShaderDidNotCompile {
          error: error.to_string_lossy().into_owned(),
        }
        .into(),
      );
    }

    Ok(id)
  }
}
