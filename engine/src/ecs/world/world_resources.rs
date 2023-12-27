use gl::Gl;

use crate::{ecs::World, math::Transforms, view::render_gl::Program};

#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
  pub height:i32,
  pub width:i32,
  pub aspect:f32
}

impl ScreenDimensions {
  pub fn new(height:i32, width:i32) -> Self {
    let aspect = width as f32 / height as f32;
    ScreenDimensions { height, width, aspect }
  }
}

//probably need to make selections a vec so multiple can be selected and so
// still need a loop
#[derive(Debug, Clone, Copy)]
pub enum Selected {
  NONE,
  HOVERED(usize),
  CLICKED(usize)
}

pub struct RenderUniformLocations {
  pub model:i32,
  pub view:i32,
  pub projection:i32
}

//can I store the uniforms on the program
impl RenderUniformLocations {
  pub fn new(model:i32, view:i32, projection:i32) -> Self {
    RenderUniformLocations { model, view, projection }
  }
}

pub struct ShaderPrograms {
  pub normal:Program,
  pub highlight:Program
}

impl ShaderPrograms {
  pub fn set_normal_uniforms(&self, world:&World) {
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();
    let gl = world.immut_get_resource::<Gl>().unwrap();

    self.normal.use_program(gl);

    //bind the view transform
    self
      .normal
      .set_uniform_matrix4fv(gl, uniform_locations.view, &transforms.get_view_transform());

    //bind the projection transform
    self
      .normal
      .set_uniform_matrix4fv(gl, uniform_locations.projection, transforms.get_projection_transform().as_matrix());
  }

  pub fn set_highlight_uniforms(&self, world:&World) {
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();
    let gl = world.immut_get_resource::<Gl>().unwrap();

    self.highlight.use_program(gl);

    //bind the view transform
    self
      .highlight
      .set_uniform_matrix4fv(gl, uniform_locations.view, &transforms.get_view_transform());

    //bind the projection transform
    self
      .highlight
      .set_uniform_matrix4fv(gl, uniform_locations.projection, transforms.get_projection_transform().as_matrix());
  }
}

pub struct DbgShaderProgram {
  pub program:Program
}

impl DbgShaderProgram {
  pub fn new(program:Program) -> Self {
    DbgShaderProgram { program }
  }

  pub fn set_normal_uniforms(&self, world:&World) {
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();
    let gl = world.immut_get_resource::<Gl>().unwrap();

    self.program.use_program(gl);

    //bind the view transform
    self
      .program
      .set_uniform_matrix4fv(gl, uniform_locations.view, &transforms.get_view_transform());

    //bind the projection transform
    self
      .program
      .set_uniform_matrix4fv(gl, uniform_locations.projection, transforms.get_projection_transform().as_matrix());
  }
}

pub struct DebugElements {
  pub aabb:bool
}

impl DebugElements {
  pub fn new(aabb:bool) -> Self {
    DebugElements { aabb }
  }
}
