use super::shader::Shader;
use crate::{errors::ShaderErrors, filesystem::create_whitespace_cstring, math::math::Mat4};
use eyre::Result;
use gl::{
  types::{GLchar, GLenum, GLint, GLuint},
  Gl, VERTEX_SHADER,
};
use std::{ffi::CString, ptr::null_mut};

// Refactor:
// -Make a program builder

//For both the Program and the shader, find a way to print the errors
#[derive(Debug, Clone, Copy)]
pub struct Program {
  pub id: GLuint,
  pub model_uniform_location: Option<i32>,
  pub view_uniform_location: Option<i32>,
  pub projection_uniform_location: Option<i32>,
}

impl Program {
  pub fn new(gl: &Gl, vert_name: &str, shader_2_name: &str, shader_2_kind: GLenum) -> Result<ProgramBuilder, String> {
    let vert_shader = Shader::new(gl, vert_name, VERTEX_SHADER).unwrap();
    let shader_2 = Shader::new(gl, shader_2_name, shader_2_kind).unwrap();
    let id;
    let mut compile_status: GLint = 1;

    unsafe {
      id = gl.CreateProgram();
      gl.AttachShader(id, vert_shader.id);
      gl.AttachShader(id, shader_2.id);
      gl.LinkProgram(id);
      gl.GetProgramiv(id, gl::LINK_STATUS, &mut compile_status);
    }

    if compile_status == 0 {
      let mut len: GLint = 0;
      unsafe {
        gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring(len as usize);

      unsafe {
        gl.GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
      }
      return Err(error.to_string_lossy().into_owned());
    }

    unsafe {
      gl.DeleteShader(vert_shader.id);
      gl.DeleteShader(shader_2.id);
    }

    Ok(ProgramBuilder {
      id,
      model_uniform_location: None,
      view_uniform_location: None,
      projection_uniform_location: None,
    })
  }

  //not working because use program does not actually use program, this is getting the location
  pub fn use_program(&self, gl: &Gl) {
    unsafe { gl.UseProgram(self.id) }
  }

  pub fn set_model_matrix(&self, gl: &Gl, uniform_value: &Mat4) {
    Self::set_uniform_matrix4fv(gl, self.model_uniform_location.unwrap(), uniform_value);
  }

  pub fn set_view_matrix(&self, gl: &Gl, uniform_value: &Mat4) {
    Self::set_uniform_matrix4fv(gl, self.view_uniform_location.unwrap(), uniform_value);
  }

  pub fn set_projection_matrix(&self, gl: &Gl, uniform_value: &Mat4) {
    Self::set_uniform_matrix4fv(gl, self.projection_uniform_location.unwrap(), uniform_value);
  }

  fn set_uniform_matrix4fv(gl: &Gl, uniform_location: i32, uniform_value: &Mat4) {
    unsafe {
      gl.UniformMatrix4fv(uniform_location, 1, gl::FALSE, uniform_value.as_ptr() as *const f32);
    }
  }
}

pub struct ProgramBuilder {
  id: GLuint,
  model_uniform_location: Option<i32>,
  view_uniform_location: Option<i32>,
  projection_uniform_location: Option<i32>,
}

impl ProgramBuilder {
  pub fn build(&self) -> Result<Program, String> {
    Ok(Program {
      id: self.id,
      model_uniform_location: self.model_uniform_location,
      view_uniform_location: self.view_uniform_location,
      projection_uniform_location: self.projection_uniform_location,
    })
  }
  ///Register a `model` uniform for the [`Program`].
  pub fn with_model(&mut self, gl: &Gl) -> Result<&mut Self> {
    self.model_uniform_location = Some(self.get_uniform_location(gl, "model")?);
    Ok(self)
  }

  ///Register a `view` uniform for the [`Program`].
  pub fn with_view(&mut self, gl: &Gl) -> Result<&mut Self> {
    self.view_uniform_location = Some(self.get_uniform_location(gl, "view")?);
    Ok(self)
  }

  ///Register a `projection` uniform for the [`Program`].
  pub fn with_projection(&mut self, gl: &Gl) -> Result<&mut Self> {
    self.projection_uniform_location = Some(self.get_uniform_location(gl, "projection")?);
    Ok(self)
  }

  fn get_uniform_location(&self, gl: &Gl, name: &str) -> Result<i32> {
    let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

    let location = unsafe { gl.GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8) };

    if location == -1 {
      return Err(ShaderErrors::UniformLocationNotFound.into());
    }

    Ok(location)
  }
}
