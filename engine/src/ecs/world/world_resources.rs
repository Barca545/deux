use crate::{view::render_gl::Program, ecs::{World, component_lib::{ModelUniformLocation, ViewUniformLocation, ProjectionUniformLocation}}, math::Transforms};

#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
  pub height:i32,
  pub width:i32,
  pub aspect:f32
}
impl ScreenDimensions {
  pub fn new(height:i32, width:i32) -> Self {
    let aspect = width as f32 / height as f32;
    ScreenDimensions {
      height,
      width,
      aspect
    }
  }
}

pub struct ShaderPrograms{
  pub normal:Program,
  pub highlight:Program,
}

impl ShaderPrograms {
  pub fn set_normal_uniforms(&self,world:&World){
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let view_uniform_loc = world.immut_get_resource::<ViewUniformLocation>().unwrap();
    let projection_uniform_loc = world.immut_get_resource::<ProjectionUniformLocation>().unwrap();

    self.normal.use_program();

    //bind the view transform
    self.normal.set_uniform_matrix4fv(view_uniform_loc.0, &transforms.get_view_transform());

    //bind the projection transform
    self.normal.set_uniform_matrix4fv(
      projection_uniform_loc.0,
      transforms.get_projection_transform().as_matrix()
    );
  }

  pub fn set_highlight_uniforms(&self,world:&World){
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let view_uniform_loc = world.immut_get_resource::<ViewUniformLocation>().unwrap();
    let projection_uniform_loc = world.immut_get_resource::<ProjectionUniformLocation>().unwrap();

    self.highlight.use_program();

    //bind the view transform
    self.highlight.set_uniform_matrix4fv(view_uniform_loc.0, &transforms.get_view_transform());

    //bind the projection transform
    self.highlight.set_uniform_matrix4fv(
      projection_uniform_loc.0,
      transforms.get_projection_transform().as_matrix()
    );
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Selected {
  NONE,
  HOVERED(usize),
  CLICKED(usize)
}
