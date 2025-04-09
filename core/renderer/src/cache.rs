use storage::{Cache, CacheKey, Cacheable};
use wgpu::{BindGroup, Id};

use crate::scene::material::Material;

#[derive(Debug,)]
/// Wrapper around [`BindGroup`](wgpu::BindGroup) so it can implement
/// [`Cacheable`].
pub struct CachedBindGroup(pub BindGroup,);
impl Cacheable for CachedBindGroup {
  type Output = Id<BindGroup,>;

  fn hash(&self,) -> Self::Output {
    self.0.global_id()
  }
}

pub type BindGroupCache = Cache<CachedBindGroup,>;
/// The key to a given [`BindGroup`](wgpu::BindGroup)'s entry in the
/// [`BindGroupCache`].
pub type BindGroupKey = CacheKey<CachedBindGroup,>;

pub type MaterialCache = Cache<Material,>;
/// The key to a given [`Material`]'s entry in the [`MaterialCache`].
pub type MaterialKey = CacheKey<Material,>;

impl Cacheable for Material {
  type Output = String;

  fn hash(&self,) -> Self::Output {
    match self {
      Material::Opaque(mat,) => mat.name.clone(),
    }
  }
}
