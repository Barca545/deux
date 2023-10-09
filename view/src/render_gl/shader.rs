

use ecs::World;
use cgmath::{Matrix4,Matrix};
use gl::{VERTEX_SHADER,FRAGMENT_SHADER,types::{GLint,GLchar,GLenum,GLuint}, Gl};
use std::{
  ffi::{CString, CStr},
  ptr::null_mut
};

pub struct Program{
  gl: Gl,
  id:GLuint,
  pub name: String
}

impl Program {
  pub fn from_shader_files(gl: &Gl, world: &World, name: &str) -> Program {
    const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

    let resource_names = POSSIBLE_EXT
      .iter()
      .map(|file_extension| format!("{}{}", name, file_extension))
      .collect::<Vec<String>>();

    let shaders:Vec<Shader> = resource_names
      .iter()
      .map(|resource_name| Shader::from_shader_files(gl, world, resource_name)).collect();

    Program::from_shaders(name, gl, &shaders[..]).unwrap()
}
  
  pub fn from_shaders(name:&str, gl: &Gl,shaders:&[Shader]) -> Result<Program,String>{
    let program_id = unsafe{gl.CreateProgram()};

    for shader in shaders{
      unsafe{gl.AttachShader(program_id,shader.id_ref());}
    }

    unsafe{gl.LinkProgram(program_id);}

    //error handling
    let mut compile_status:GLint = 1;
    unsafe{gl.GetProgramiv(program_id,gl::LINK_STATUS,&mut compile_status);}

    if compile_status == 0 {
      let mut len:GLint = 0;
      unsafe{gl.GetProgramiv(program_id,gl::INFO_LOG_LENGTH,&mut len);}

      let error = create_whitespace_cstring_with_len(len as usize);

      unsafe{
        gl.GetProgramInfoLog(
          program_id,
          len,
          null_mut(),
          error.as_ptr() as *mut GLchar
        );
      }
      return Err(error.to_string_lossy().into_owned());
    }

    for shader in shaders{
      unsafe{gl.DetachShader(program_id,shader.id_ref());}
    }
    
    Ok(Program { 
      name: name.into(),
      gl: gl.clone(),
      id: program_id,
    })
  }

  pub fn use_program(&self){
    unsafe{self.gl.UseProgram(self.id)}
  }

  //needs to return and error
  pub fn get_uniform_location(&self, name: &str) -> i32{
    let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

    let location = unsafe {
      self.gl.GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
    };

    //-1 means location not found
    location
  }

  pub fn set_uniform_matrix4fv(&self, uniform_location:i32,uniform_value:&Matrix4<f32>){
    unsafe{
      self.gl.UniformMatrix4fv(
        uniform_location, 
        1, 
        gl::FALSE, 
        uniform_value.as_ptr() as *const f32);
    }
  }

  pub fn id_ref(&self) -> GLuint{
    self.id
  }
  
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe{self.gl.DeleteProgram(self.id)}
  }
}

pub struct Shader{
  gl: gl::Gl,
  id:GLuint
}

impl Shader{
  pub fn from_shader_files(gl: &gl::Gl, world:&World, resource_name: &str) -> Shader {
    const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
      [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

    let shader_kind = POSSIBLE_EXT
      .iter()
      .find(|&&(file_extension, _)| resource_name.ends_with(file_extension))
      .map(|&(_, kind)| kind).unwrap();

    let source = world.load_resource_from_cstring(resource_name).unwrap();

    Shader::from_source(gl, &source, shader_kind).unwrap()
}
  
  pub fn from_source(gl:&gl::Gl, source:&CStr, kind:GLenum) -> Result<Shader,String> {
    let id = shader_from_source(gl,source,kind)?;
    Ok(Shader {
      gl: gl.clone(),
      id: id
    })
  }

  pub fn from_vertex_source(gl: &gl::Gl, source:&CStr) -> Result<Shader,String> {
    Shader::from_source(gl, source, VERTEX_SHADER)
  }
  
  pub fn from_fragment_source(gl: &gl::Gl, source:&CStr) -> Result<Shader,String> {
    Shader::from_source(gl, source,FRAGMENT_SHADER)
  }

  ///Retrieves the parent shader's id.
  pub fn id_ref(&self) -> GLuint{
    self.id
  }
}

impl Drop for Shader{
  fn drop(&mut self) {
    unsafe{
      self.gl.DeleteShader(self.id);
    }
  }
}

fn create_whitespace_cstring_with_len(len:usize) -> CString{
  let mut buffer: Vec<u8> = Vec::with_capacity(len as usize+1);
  buffer.extend([b' '].iter().cycle().take(len as usize));
  unsafe{CString::from_vec_unchecked(buffer)}
}

//is leaving this outside somehow faster?
fn shader_from_source(gl: &gl::Gl,source:&CStr,kind:GLenum) -> Result<GLuint,String>{
  let id = unsafe {gl.CreateShader(kind)};
  unsafe{    
    gl.ShaderSource(id,1,&source.as_ptr(),std::ptr::null());
    gl.CompileShader(id);
  }

  let mut compile_status: GLint = 1;
  
  unsafe{gl.GetShaderiv(id,gl::COMPILE_STATUS,&mut compile_status);}
  
  if compile_status == 0 {
    let mut len:GLint = 0;
    unsafe{
      gl.GetShaderiv(id,gl::INFO_LOG_LENGTH,&mut len)
    }

    let error: CString = create_whitespace_cstring_with_len(len as usize);

    unsafe {
      gl.GetShaderInfoLog(
        id,len,
        null_mut(),
        error.as_ptr() as *mut GLchar) 
    }
    return Err(error.to_string_lossy().into_owned());
  }
  Ok(id)
}