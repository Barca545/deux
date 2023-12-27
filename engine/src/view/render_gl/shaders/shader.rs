use std::{ffi::CString, ptr::null_mut};

use crate::filesystem::{create_whitespace_cstring, load_shader};
use eyre::Result;
use gl::{
  types::{GLchar, GLenum, GLint, GLuint},
  Gl, COMPILE_STATUS, FRAGMENT_SHADER, VERTEX_SHADER
};

pub struct Shader {
  pub id:GLuint
}

impl Shader {
  //Find ways to convert the result into a custom error
  pub fn new(gl:&Gl, name:&str, kind:GLenum) -> Result<Self, String> {
    let source;
    let id;

    if kind == VERTEX_SHADER {
      source = load_shader(name, "vert").unwrap();
      id = compile_shader(gl, kind, source)?;
    } else if kind == FRAGMENT_SHADER {
      source = load_shader(name, "frag").unwrap();
      id = compile_shader(gl, kind, source)?;
    } else {
      source = load_shader(name, "geom").unwrap();
      id = compile_shader(gl, kind, source)?;
    }

    Ok(Shader { id })
  }
}

fn compile_shader(gl:&Gl, kind:GLenum, source:CString) -> Result<GLuint, String> {
  let id:GLuint;
  let mut compile_status:GLint = 1;

  unsafe {
    id = gl.CreateShader(kind);
    gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
    gl.CompileShader(id);
    gl.GetShaderiv(id, COMPILE_STATUS, &mut compile_status)
  }

  if compile_status == 0 {
    let mut len:GLint = 0;
    unsafe { gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) }

    let error:CString = create_whitespace_cstring(len as usize);

    unsafe { gl.GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar) }

    return Err(error.to_string_lossy().into_owned());
  }

  Ok(id)
}
