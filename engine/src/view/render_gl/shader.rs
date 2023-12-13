extern crate nalgebra_glm as glm;
use crate::{ecs::World, filesystem::load_cstring, math::math::Mat4};

use gl::{
	types::{GLchar, GLenum, GLint, GLuint},
	Gl, FRAGMENT_SHADER, VERTEX_SHADER,
};
use std::{
	ffi::{CStr, CString},
	ptr::null_mut,
};

pub struct Program {
	gl: Gl,
	pub id: GLuint,
	pub name: String,
}

impl Program {
	pub fn from_shader_files(gl: &Gl, name: &str) -> Program {
		const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

		let names = POSSIBLE_EXT
			.iter()
			.map(|file_extension| format!("{}{}", name, file_extension))
			.collect::<Vec<String>>();

		let shaders: Vec<Shader> = names
			.iter()
			.map(|name| Shader::from_shader_files(gl, name))
			.collect();

		Program::from_shaders(name, gl, &shaders[..]).unwrap()
	}

	pub fn from_shaders(name: &str, gl: &Gl, shaders: &[Shader]) -> Result<Program, String> {
		let program_id = unsafe { gl.CreateProgram() };

		for shader in shaders {
			unsafe {
				gl.AttachShader(program_id, shader.id_ref());
			}
		}

		unsafe {
			gl.LinkProgram(program_id);
		}

		//error handling
		let mut compile_status: GLint = 1;
		unsafe {
			gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut compile_status);
		}

		if compile_status == 0 {
			let mut len: GLint = 0;
			unsafe {
				gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
			}

			let error = create_whitespace_cstring_with_len(len as usize);

			unsafe {
				gl.GetProgramInfoLog(program_id, len, null_mut(), error.as_ptr() as *mut GLchar);
			}
			return Err(error.to_string_lossy().into_owned());
		}

		for shader in shaders {
			unsafe {
				gl.DetachShader(program_id, shader.id_ref());
			}
		}

		Ok(Program {
			name: name.into(),
			gl: gl.clone(),
			id: program_id,
		})
	}

	pub fn use_program(&self) {
		unsafe { self.gl.UseProgram(self.id) }
	}

	//needs to return and error
	pub fn get_uniform_location(&self, name: &str) -> i32 {
		let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

		let location = unsafe {
			self
				.gl
				.GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
		};

		//-1 means location not found
		location
	}

	pub fn set_uniform_matrix4fv(&self, uniform_location: i32, uniform_value: &Mat4) {
		// value is incorrect if the matricies use higher precision
		// value when f64 0x00000099c4dfe6e0
		// value when f32 0x000000f76aafead0

		unsafe {
			self.gl.UniformMatrix4fv(
				uniform_location,
				1,
				gl::FALSE,
				uniform_value.as_ptr() as *const f32,
			);
		}
	}

	pub fn id_ref(&self) -> GLuint {
		self.id
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe { self.gl.DeleteProgram(self.id) }
	}
}

pub struct Shader {
	gl: gl::Gl,
	id: GLuint,
}

impl Shader {
	pub fn from_shader_files(gl: &Gl, name: &str) -> Shader {
		const POSSIBLE_EXT: [(&str, GLenum); 2] =
			[(".vert", VERTEX_SHADER), (".frag", FRAGMENT_SHADER)];

		let shader_kind = POSSIBLE_EXT
			.iter()
			.find(|&&(file_extension, _)| name.ends_with(file_extension))
			.map(|&(_, kind)| kind)
			.unwrap();

		let source = load_cstring(name).unwrap();

		Shader::from_source(gl, &source, shader_kind).unwrap()
	}

	pub fn from_source(gl: &Gl, source: &CStr, kind: GLenum) -> Result<Shader, String> {
		let id = shader_from_source(gl, source, kind)?;
		Ok(Shader { gl: gl.clone(), id })
	}

	pub fn from_vertex_source(gl: &Gl, source: &CStr) -> Result<Shader, String> {
		Shader::from_source(gl, source, VERTEX_SHADER)
	}

	pub fn from_fragment_source(gl: &Gl, source: &CStr) -> Result<Shader, String> {
		Shader::from_source(gl, source, FRAGMENT_SHADER)
	}

	///Retrieves the parent shader's id.
	pub fn id_ref(&self) -> GLuint {
		self.id
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			self.gl.DeleteShader(self.id);
		}
	}
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
	let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
	buffer.extend([b' '].iter().cycle().take(len as usize));
	unsafe { CString::from_vec_unchecked(buffer) }
}

//is leaving this outside somehow faster?
fn shader_from_source(gl: &gl::Gl, source: &CStr, kind: GLenum) -> Result<GLuint, String> {
	let id = unsafe { gl.CreateShader(kind) };
	unsafe {
		gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
		gl.CompileShader(id);
	}

	let mut compile_status: GLint = 1;

	unsafe { gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_status) }

	if compile_status == 0 {
		let mut len: GLint = 0;
		unsafe { gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) }

		let error: CString = create_whitespace_cstring_with_len(len as usize);

		unsafe { gl.GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar) }
		return Err(error.to_string_lossy().into_owned());
	}
	Ok(id)
}
