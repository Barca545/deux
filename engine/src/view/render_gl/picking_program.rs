use gl::{
	types::{GLint, GLuint},
	Gl,
};

use crate::{
	ecs::World,
	math::{math::Vec3, Transforms},
};

use super::Program;

pub struct PickingProgram {
	gl: Gl,
	shader_program: Program,
	model_uniform_loc: GLint,
	view_uniform_loc: GLint,
	projection_uniform_loc: GLint,
	object_index_location: GLint,
	draw_index_location: GLint,
}

impl PickingProgram {
	pub fn new(gl: &Gl, world: &World, name: &str) -> Self {
		let gl = gl.clone();
		let shader_program = Program::from_shader_files(&gl, name);

		//need to update the shader since the tutorial multiplies them in this code instead of the shader
		let model_uniform_loc = shader_program.get_uniform_location("model");
		let view_uniform_loc = shader_program.get_uniform_location("view");
		let projection_uniform_loc = shader_program.get_uniform_location("projection");
		let object_index_location = shader_program.get_uniform_location("gObjectIndex");
		let draw_index_location = shader_program.get_uniform_location("gDrawIndex");

		PickingProgram {
			gl,
			shader_program,
			model_uniform_loc,
			view_uniform_loc,
			projection_uniform_loc,
			object_index_location,
			draw_index_location,
		}
	}

	//this could maybe be a function on the main program struct
	pub fn set_transformations(&self, position: &Vec3, transforms: &Transforms) {
		//bind the model transform
		self.shader_program.set_uniform_matrix4fv(
			self.model_uniform_loc,
			&transforms.get_model_transform(position),
		);

		//bind the view transform
		self
			.shader_program
			.set_uniform_matrix4fv(self.view_uniform_loc, &transforms.get_view_transform());

		//bind the projection transform
		self.shader_program.set_uniform_matrix4fv(
			self.projection_uniform_loc,
			transforms.get_projection_transform().as_matrix(),
		);
	}

	//no idea what this name means
	pub fn draw_start_cb(&self, draw_index: GLint) {
		//this could maybe be a function on the main program struct
		unsafe {
			self
				.gl
				.Uniform1ui(self.draw_index_location, draw_index as GLuint)
		}
	}

	pub fn set_object_index(&self, object_index: GLint) {
		unsafe {
			self
				.gl
				.Uniform1ui(self.object_index_location, object_index as GLuint)
		}
	}
}
