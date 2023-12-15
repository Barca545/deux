use std::{ffi::CString, ptr::null_mut};
use gl::{Gl, VERTEX_SHADER, FRAGMENT_SHADER, types::{GLuint, GLint, GLchar}};
use eyre::Result;
use crate::{math::math::Mat4, errors::ShaderErrors, filesystem::create_whitespace_cstring};
use super::shader::Shader;

//For both the Program and the shader, find a way to print the errors
pub struct Program{
  pub id:GLuint,
}

impl Program{
  pub fn new(gl:&Gl, vert_name:&str, frag_name:&str) -> Result<Self,String> {
    let vert_shader = Shader::new(gl, vert_name, VERTEX_SHADER).unwrap();
    let frag_shader = Shader::new(gl, frag_name, FRAGMENT_SHADER).unwrap();

    let id;
    let mut compile_status:GLint = 1;

    unsafe{
      id = gl.CreateProgram();
      gl.AttachShader(id, vert_shader.id);
      gl.AttachShader(id, frag_shader.id);
      gl.LinkProgram(id);
      gl.GetProgramiv(id, gl::LINK_STATUS, &mut compile_status);
    }

    if compile_status == 0 {
      let mut len:GLint = 0;
      unsafe {
        gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring(len as usize);

      unsafe {
        gl.GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
      }
      return Err(error.to_string_lossy().into_owned());
    }

    unsafe{
      gl.DeleteShader(vert_shader.id);
      gl.DeleteShader(frag_shader.id);
    }

    Ok(Program{id})
  }

  pub fn use_program(&self, gl:&Gl){
    unsafe{gl.UseProgram(self.id)}
  }

  pub fn get_uniform_location(&self, gl:&Gl, name:&str) -> Result<i32> {
    let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

    let location = unsafe {
      gl.GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
    };

    if location == -1 {
      return Err(ShaderErrors::UniformLocationNotFound.into());
    }

    Ok(location)
  }

  //figure out why I can't use f64
  pub fn set_uniform_matrix4fv(&self, gl:&Gl, uniform_location:i32, uniform_value:&Mat4) {
    unsafe {
      gl.UniformMatrix4fv(
        uniform_location,
        1,
        gl::FALSE,
        uniform_value.as_ptr() as *const f32
      );
    }
  }
}
