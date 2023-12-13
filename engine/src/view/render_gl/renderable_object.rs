extern crate nalgebra_glm as glm;

use crate::{
	ecs::{
		component_lib::{
			ModelUniformLocation, Position, ProjectionUniformLocation, ViewUniformLocation,
		},
		World,
	},
	math::{math::Vec3, Renderable, Transforms},
	view::render::Mesh,
};

use super::{
	buffer::{ArrayBuffer, VertexArray},
	Program, Texture, Vertex,
};

use eyre::Result;
use gl::{types::GLint, Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, TRIANGLES};

//right now this only renders the square
pub struct RenderableObject {
	vertices: Vec<Vertex>, //really this should be something that impls the vertex trait
	shader_program: Program,
	model_uniform_loc: GLint,
	view_uniform_loc: GLint,
	projection_uniform_loc: GLint,
	// _texture: Texture,
	// _vbo: ArrayBuffer,
	// vao:VertexArray,
	// index_count: GLsizei,
}
//add error wrapping
impl RenderableObject {
	/**
	Builds a `RenderableObject` by taking in data and drawing it into the VBO,
	EBO and constructing the VAO.
	*/
	pub fn new(
		gl: &Gl, world: &World, name: &str, vertices: Vec<Vertex>, texture_path: &str,
	) -> Result<RenderableObject> {
		let shader_program = Program::from_shader_files(&gl, name);

		//let transform_uniform_loc = shader_program.get_uniform_location("transform");
		let model_uniform_loc = shader_program.get_uniform_location("model");
		let view_uniform_loc = shader_program.get_uniform_location("view");
		let projection_uniform_loc = shader_program.get_uniform_location("projection");

		// dbg!(model_uniform_loc);
		// dbg!(view_uniform_loc);
		// dbg!(projection_uniform_loc);
		//this actually loads the texture beforehand which feels like it might be faster I'm not sure I love it doing all the texture binding logic before I actually tell it to render
		//texture is being bound but not unbound (I think, so I'm only able to use one at a time)
		//issue seems to be that the from path function binds the texture
		//I think I just want to load the texture into memory
		//generate the texture object and store those then when rendering use each as appropriate

		// let texture = Texture::rgb_from_path(path.as_str()).with_mipmaps().load(gl)?;

		// let indices = [
		//   0, 1, 3,  // first Triangle
		//   1, 2, 3   // second Triangle
		// ];

		Ok(RenderableObject {
			vertices,
			shader_program,
			// _texture:texture,
			model_uniform_loc,
			view_uniform_loc,
			projection_uniform_loc,
			// _vbo: vbo,
			// vao,
			// index_count: indices.len() as i32,
		})
	}

	pub fn render(&self, world: &World, vao: &VertexArray) -> Result<()> {
		let gl = world.immut_get_resource::<Gl>().unwrap();
		let transforms = world.immut_get_resource::<Transforms>().unwrap();
		let program = world.immut_get_resource::<Program>().unwrap();
		let model_uniform_loc = world.immut_get_resource::<ModelUniformLocation>().unwrap();
		let view_uniform_loc = world.immut_get_resource::<ViewUniformLocation>().unwrap();
		let projection_uniform_loc = world
			.immut_get_resource::<ProjectionUniformLocation>()
			.unwrap();

		let mut query = world.query();

		let entities = query
			.with_component::<Mesh>()?
			.with_component::<Position>()?
			.run_entity();

		unsafe { gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT) };

		// let vbo = ArrayBuffer::new(&gl);
		// vbo.bind();
		// vbo.static_draw_data(&self.vertices);
		// vbo.unbind();

		// vao.bind();
		// vbo.bind();

		// Vertex::init_attrib_pointers(&gl);
		// vao.unbind();
		// vbo.unbind();

		for entity in entities {
			let position = entity.immut_get_component::<Position>()?;
			//should all the shader stuff be wrapped into a method on the struct?
			program.use_program();

			//bind the model transform
			program.set_uniform_matrix4fv(
				model_uniform_loc.0,
				&transforms.get_model_transform(&position.0),
			);

			//bind the view transform
			program.set_uniform_matrix4fv(view_uniform_loc.0, &transforms.get_view_transform());

			//bind the projection transform
			self.shader_program.set_uniform_matrix4fv(
				projection_uniform_loc.0,
				transforms.get_projection_transform().as_matrix(),
			);

			vao.bind();

			unsafe {
				gl.DrawArrays(TRIANGLES, 0, 36);
			}
		}
		Ok(())
	}
}
