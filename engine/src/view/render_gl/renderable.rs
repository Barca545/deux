//programs register a program adds it to a hashmap
//Each Render pipeline stage might need its own program

use super::Program;
use crate::{ecs::World, math::Transforms};
use gl::Gl;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Programs {
  map: HashMap<usize, Program>,
}

impl Programs {
  ///Creates an empty [`Programs`].
  pub fn new() -> Self {
    Programs::default()
  }

  ///Registers a [`Program`] with the [`Programs`].
  pub fn register_program(&mut self, id: usize, program: Program) {
    self.map.insert(id, program);
  }

  ///Sets value of the view and projection uniforms.
  pub fn set_vp_uniforms(&self, id: usize, world: &World) {
    let transforms = world.get_resource::<Transforms>().unwrap();
    let gl = world.get_resource::<Gl>().unwrap();

    //Get the program
    let program = self.map.get(&id).unwrap();
    program.use_program(&gl);

    //Set the view uniform
    program.set_view_matrix(&gl, &transforms.view_transform);

    //Set the projection uniform
    program.set_projection_matrix(&gl, transforms.projection_transform.as_matrix());
  }

  ///Sets the value of the model uniform.
  pub fn set_model_uniform(&self, id: usize, world: &World) {}

  ///Bind the program for use.
  pub fn use_program(&self, id: usize, world: &World) {
    let gl = world.get_resource::<Gl>().unwrap();

    //Get the program
    let program = self.map.get(&id).unwrap();
    program.use_program(&gl);
  }
}

pub trait Renderable {
  fn draw();
}
