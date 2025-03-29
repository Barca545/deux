use crate::renderer::core::texture::Texture;
use wgpu::BindGroup;

pub struct Material {
  pub name:String,
  pub diffuse_texture:Texture,
  pub bind_group:BindGroup,
}

impl Material {
  pub fn new(name:&str, diffuse_texture:Texture, bind_group:BindGroup,) -> Self {
    Material {
      name:String::from(name,),
      diffuse_texture,
      bind_group,
    }
  }
}
