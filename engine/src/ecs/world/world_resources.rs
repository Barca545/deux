use gl::{Gl,FRAGMENT_SHADER};

use crate::{ecs::World, math::Transforms, view::render_gl::Program};

use super::world;


#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
  pub width:i32,
  pub height:i32,
  pub aspect:f32,
  // pub int_aspect: i32
}

impl ScreenDimensions {
  pub fn new(width:i32, height:i32, ) -> Self {
    let aspect = width as f32 / height as f32;
    // dbg!(width/height);
    // let aspect = (width/height) as f32;
    // let int_aspect = width/height;
    ScreenDimensions { 
      height, 
      width, 
      aspect,
      // int_aspect
    }
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
  pub highlight:Program,
  model_uniform_location:i32,
  view_uniform_location:i32,
  projection_uniform_location:i32,
}

impl ShaderPrograms {
  pub fn new(world:&World) -> Self {
    let gl = world.immut_get_resource::<Gl>().unwrap();

    let normal = Program::new(&gl, "textured", "textured", FRAGMENT_SHADER).unwrap();
    let highlight = Program::new(&gl, "textured", "highlight", FRAGMENT_SHADER).unwrap();

    let model_uniform_location = normal.get_uniform_location(gl, "model").unwrap();
    let view_uniform_location = normal.get_uniform_location(gl, "view").unwrap();
    let projection_uniform_location = normal.get_uniform_location(gl, "projection").unwrap();
    
    Self {
      normal,
      highlight,
      model_uniform_location,
      view_uniform_location,
      projection_uniform_location 
    }

  }
  pub fn set_normal_uniforms(&self, world:&World) {
    // let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    // dbg!(transforms.projection_transform);
    let gl = world.immut_get_resource::<Gl>().unwrap();

    self.normal.use_program(gl);

    //bind the view transform
    self
      .normal
      .set_uniform_matrix4fv(
        gl,
        self.view_uniform_location,
        &transforms.view_transform
      );

    //bind the projection transform
    self
      .normal
      .set_uniform_matrix4fv(
        gl, 
        self.projection_uniform_location, 
        transforms.projection_transform.as_matrix()
      );
  }

  pub fn set_highlight_uniforms(&self, world:&World) {
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    // let uniform_locations = world.immut_get_resource::<RenderUniformLocations>().unwrap();
    let gl = world.immut_get_resource::<Gl>().unwrap();

    self.highlight.use_program(gl);

    //bind the view transform
    self
      .highlight
      .set_uniform_matrix4fv(
        gl,
        self.view_uniform_location,
        &transforms.view_transform
      );

    //bind the projection transform
    self
      .highlight
      .set_uniform_matrix4fv(
        gl,
        self.projection_uniform_location,
        transforms.projection_transform.as_matrix()
      );
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
      .set_uniform_matrix4fv(
        gl,
        uniform_locations.view,
        &transforms.view_transform
      );

    //bind the projection transform
    self
      .program
      .set_uniform_matrix4fv(
        gl,
        uniform_locations.projection,
        transforms.projection_transform.as_matrix()
      );
  }
}

pub struct DebugElements {
  pub aabb:bool,
  pub attacks:bool
}

impl DebugElements {
  pub fn new(aabb:bool, attacks:bool) -> Self {
    DebugElements { 
      aabb,
      attacks 
    }
  }
}
