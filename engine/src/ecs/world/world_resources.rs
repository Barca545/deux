use gl::{Gl,FRAGMENT_SHADER};

use crate::{ecs::World, math::Transforms, view::render_gl::Program};

use eyre::Result;

#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
  pub width:i32,
  pub height:i32,
  pub aspect:f32,
}

impl ScreenDimensions {
  pub fn new(width:i32, height:i32) -> Self {
    let aspect = width as f32 / height as f32;
    ScreenDimensions { 
      height, 
      width, 
      aspect,
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

pub struct ShaderPrograms {
  pub normal:Program,
  pub highlight:Program,
}

impl ShaderPrograms {
  pub fn new(world:&World) -> Result<Self> {
    let gl = world.get_resource::<Gl>().unwrap();

    let mut normal = Program::new(&gl, "textured", "textured", FRAGMENT_SHADER).unwrap();
    let mut highlight = Program::new(&gl, "textured", "highlight", FRAGMENT_SHADER).unwrap();
    
    normal
      .with_model(&gl)?
      .with_view(&gl)?
      .with_projection(&gl)?;

    highlight
      .with_model(&gl)?
      .with_view(&gl)?
      .with_projection(&gl)?;

    Ok(Self { normal, highlight })
  }
  
  pub fn set_normal_uniforms(&self, world:&World) {
    let transforms = world.get_resource::<Transforms>().unwrap();
    let gl = world.get_resource::<Gl>().unwrap();
    let program = self.normal;

    program.use_program(&gl);

    //Set the view transform's value
    program.set_view_matrix(&gl, &transforms.view_transform);
    
    //Set the projection transform's value
    program.set_projection_matrix(&gl, transforms.projection_transform.as_matrix());
  }

  pub fn set_highlight_uniforms(&self, world:&World) {
    let transforms = world.get_resource::<Transforms>().unwrap();
    let gl = world.get_resource::<Gl>().unwrap();
    let program = self.highlight;

    program.use_program(&gl);

    //Set the view transform's value
    program.set_view_matrix(&gl, &transforms.view_transform);
    
    //Set the projection transform's value
    program.set_projection_matrix(&gl, transforms.projection_transform.as_matrix());
  }
}

#[derive(Debug, Clone, Copy)]
pub struct DbgShaderProgram {
  pub program:Program,
}

impl DbgShaderProgram {
  pub fn new(world:&World) -> Self {
    let gl = world.get_resource::<Gl>().unwrap();
    
    let program = Program::new(&gl, "debug", "debug", FRAGMENT_SHADER).unwrap();

    DbgShaderProgram { 
      program,
    }
  }

  pub fn set_normal_uniforms(&self, world:&World) {
    let transforms = world.get_resource::<Transforms>().unwrap();
    let gl = world.get_resource::<Gl>().unwrap();
    let program = self.program;

    // program.use_program(gl);

    // //Set the view transform's value
    // program.set_view_matrix(gl, &transforms.view_transform);
    
    // //Set the projection transform's value
    // program.set_projection_matrix(gl, transforms.projection_transform.as_matrix());
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
