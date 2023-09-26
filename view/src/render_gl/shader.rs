use gl::{VERTEX_SHADER,FRAGMENT_SHADER,types::{GLint,GLchar,GLenum,GLuint}};
use std::{
  ffi::{CString, CStr},
  ptr::null_mut
};

pub struct Program{
  gl: gl::Gl,
  id:GLuint
}

impl Program {
  pub fn from_shaders(gl: &gl::Gl,shaders:&[Shader]) -> Result<Program,String>{
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
      gl: gl.clone(),
      id: program_id ,
    })
  }

  pub fn use_program(&self){
    unsafe{self.gl.UseProgram(self.id)}
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