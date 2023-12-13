use eyre::Result;
use gl::Gl;

use crate::ecs::{
	component_lib::{Model, Position},
	World,
};

use super::mesh::Mesh;

pub fn render_units(world: &World) -> Result<()> {
	let mut query = world.query();

	let entities = query
		.with_component::<Mesh>()?
		.with_component::<Position>()?
		.run_entity();

	for entity in entities {
		let renderable = entity.immut_get_component::<Mesh>()?;
	}

	//the original method made a new vbo but I don't think I need to generate a new one each time. I think I can just store it on the Renderable struct?
	//does every object need a new vbo or can I just use the same one?
	//when do I unbind the vbo and vao?

	Ok(())
}
