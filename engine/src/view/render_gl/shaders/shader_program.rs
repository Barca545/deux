use super::shader::Shader;
use crate::{errors::ShaderErrors, filesystem::create_whitespace_cstring, math::math::Mat4};
use eyre::Result;
use gl::{
  types::{GLchar, GLint, GLuint},
  Gl, INFO_LOG_LENGTH, LINK_STATUS,
};
use std::{collections::HashMap, ffi::CString, ptr::null_mut};

// Refactor:
// -Add actual error handling if the program doesn't have the expected fields.

//For both the ShaderProgram and the shader, find a way to print the errors
#[derive(Debug, Default, Clone)]
pub struct ShaderProgram {
  pub id: GLuint,
  pub uniforms: HashMap<String, GLint>,
}

impl ShaderProgram {
  ///Create a new [`ShaderProgram`].
  pub fn new(gl: &Gl, shaders: Vec<Shader>) -> Result<ShaderProgramBuilder> {
    let id;
    let mut compile_status: GLint = 1;

    unsafe {
      //Generate the ShaderProgram's id
      id = gl.CreateProgram();

      //Attach the shaders
      for shader in &shaders {
        gl.AttachShader(id, shader.id);
      }

      //Compile the ShaderProgram
      gl.LinkProgram(id);

      //Check whether the shader program compiled
      gl.GetProgramiv(id, LINK_STATUS, &mut compile_status);
    }

    //Return an error if the program did not compile
    if compile_status == 0 {
      let mut len: GLint = 0;
      unsafe {
        gl.GetProgramiv(id, INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring(len as usize);

      unsafe {
        gl.GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
      }
      return Err(
        ShaderErrors::ShaderDidNotCompile {
          error: error.to_string_lossy().into_owned(),
        }
        .into(),
      );
    }

    for shader in &shaders {
      unsafe {
        gl.DeleteShader(shader.id);
      }
    }

    Ok(ShaderProgramBuilder::new(gl, id))
  }

  //not working because use program does not actually use program, this is getting the location
  pub fn use_program(&self, gl: &Gl) {
    unsafe { gl.UseProgram(self.id) }
  }

  pub fn set_model_matrix(&self, gl: &Gl, uniform_value: &Mat4) {
    let location = self.uniforms.get("model").unwrap();
    Self::set_uniform_matrix4fv(gl, *location, uniform_value);
  }

  pub fn set_view_matrix(&self, gl: &Gl, uniform_value: &Mat4) {
    let location = self.uniforms.get("view").unwrap();
    Self::set_uniform_matrix4fv(gl, *location, uniform_value);
  }

  pub fn set_projection_matrix(&self, gl: &Gl, uniform_value: &Mat4) {
    let location = self.uniforms.get("projection").unwrap();
    Self::set_uniform_matrix4fv(gl, *location, uniform_value);
  }

  pub fn set_uniform_matrix4fv(gl: &Gl, location: i32, uniform_value: &Mat4) {
    unsafe {
      gl.UniformMatrix4fv(location, 1, gl::FALSE, uniform_value.as_ptr() as *const f32);
    }
  }
}

impl PartialEq for ShaderProgram {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

pub struct ShaderProgramBuilder<'b> {
  gl: &'b Gl,
  id: GLuint,
  uniforms: HashMap<String, GLint>,
}

impl<'b> ShaderProgramBuilder<'b> {
  pub fn new(gl: &'b Gl, id: GLuint) -> Self {
    let mut builder = Self {
      gl,
      id,
      uniforms: HashMap::new(),
    };

    //Attempt to register common uniforms
    builder.with_uniform("model");
    builder.with_uniform("view");
    builder.with_uniform("projection");
    builder
  }

  ///Build a new [`ShaderProgram`] with the provided parameters.
  pub fn build(&self) -> Result<ShaderProgram> {
    Ok(ShaderProgram {
      id: self.id,
      uniforms: self.uniforms.clone(),
    })
  }

  ///Register a uniform location with the [`ShaderProgram`].
  /// If the uniform does not exist, no uniform is added and an error is printed.
  pub fn with_uniform(&mut self, uniform: &str) -> &mut Self {
    let uniform_location = self.get_uniform_location(uniform);
    match uniform_location {
      Ok(location) => {
        let uniform_name = uniform.to_owned();
        self.uniforms.insert(uniform_name, location);
        self
      }
      Err(err) => {
        print!("{}", err);
        self
      }
    }
  }

  fn get_uniform_location(&self, name: &str) -> Result<i32> {
    let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

    let location = unsafe { self.gl.GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8) };

    if location == -1 {
      return Err(ShaderErrors::UniformLocationNotFound.into());
    }

    Ok(location)
  }
}
