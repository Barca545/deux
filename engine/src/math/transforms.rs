use gl::Gl; 
use glm::{vec3,rotate,identity,translate, look_at};
use nalgebra::Perspective3;

use crate::view::camera::Camera;
use super::math::{radians, Mat4, Vec3};

pub struct Transforms{
  projection_transform:Perspective3<f32>,
  view_transform:Mat4,
  fov:f32
}

impl Transforms{
  pub fn new(aspect:&f32,camera:&Camera)->Self{
    let fov = radians(45.0);
    let projection_transform = Self::calculate_projection_transform(fov,aspect);
    let view_transform:Mat4 = Self::calculate_view_transform(camera);
    
    Transforms { 
      projection_transform,
      view_transform,
      fov,
    }
  }

  //can update functions like this be stuck on a trait?
  pub fn update(&mut self,aspect:&f32,camera:&Camera){
    self.projection_transform = Self::calculate_projection_transform(self.fov,aspect);
    self.view_transform = Self::calculate_view_transform(camera);
  }

  pub fn get_model_transform(&self,position:&Vec3)->Mat4{
    Self::calculate_model_transform(position)
  }

  pub fn get_projection_transform(&self)->Perspective3<f32>{
    self.projection_transform
  }
  
  pub fn get_view_transform(&self)->Mat4{
    self.view_transform
  }

  pub fn calculate_projection_transform(fov:f32,aspect:&f32)->Perspective3<f32>{
    //Do this without cloning?
    let projection = Perspective3::new(aspect.clone(),fov, 0.1, 100.0);
    projection
  }

  pub fn calculate_view_transform(camera:&Camera)->Mat4{
    let view = look_at(
      &camera.position, 
      &camera.target, 
      &camera.up
    );
    view
  }

  pub fn calculate_model_transform(position:&Vec3)->Mat4{
    let identity:Mat4 = identity::<f32,4>();
    let position:Mat4 = translate(&identity, position);
    let axis:Vec3 = vec3(1.0,0.0, 0.0);
    let model:Mat4= rotate(&position, radians(0.0), &axis);
    model
  }
}

//move this trait to the view crate
pub trait Renderable {
  fn render(&self,gl:&Gl,transforms:&Transforms,position:&Vec3);
}
