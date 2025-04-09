use super::scene::model::Model;
use crate::{
  cache::{BindGroupCache, BindGroupKey, CachedBindGroup, MaterialCache, MaterialKey},
  scene::material::Material,
};
use storage::Arena;
use wgpu::{BindGroup, Buffer};

// TODO: Need a better description in documentation. Problem is this is
// ultimately a pretty random struct that just holds data that doesn't easily
// fit anywhere else
/// Collection of all the data needed for rendering.
pub struct RenderResources {
  /// Collection of cached [`BindGroup`](wgpu::BindGroup)s the
  /// [`Renderer`](super::renderer::Renderer) will use.
  bindgroups:BindGroupCache,
  /// Collection of cached [`Material`](crate::scene::material::Material)s the
  /// [`Renderer`](super::renderer::Renderer) will use.
  materials:MaterialCache,
  pub camera:CameraResources,
  pub models:Arena<Model,>,
}

impl RenderResources {
  pub fn new(camera:CameraResources,) -> Self {
    RenderResources {
      bindgroups:BindGroupCache::new(),
      materials:MaterialCache::new(),
      camera,
      // TODO: Optimize by using with_capacity since the number of models per stage will be known
      // at initialization
      models:Arena::new(),
    }
  }

  pub fn insert_bindgroup(&mut self, bindgroup:BindGroup,) -> BindGroupKey {
    self.bindgroups.insert(CachedBindGroup(bindgroup,),)
  }

  pub fn get_bindgroup(&self, id:BindGroupKey,) -> &BindGroup {
    &self.bindgroups.get(id,).0
  }

  pub fn insert_material(&mut self, material:Material,) -> MaterialKey {
    self.materials.insert(material,)
  }

  pub fn get_material(&self, id:MaterialKey,) -> &Material {
    &self.materials.get(id,)
  }
}

// TODO: Need better documentation
/// Data needed to use the [`Camera`](crate::renderer::scene::camera::Camera)
/// for renderering.
pub struct CameraResources {
  pub bindgroup:BindGroup,
  pub buffer:Buffer,
}
