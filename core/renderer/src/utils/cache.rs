use crate::scene::material::Material;
use macros::create_cache_type_aliases;
use storage::Cacheable;
use wgpu::{BindGroup, BindGroupLayout};

create_cache_type_aliases!(BindGroup);
create_cache_type_aliases!(Material);
create_cache_type_aliases!(BindGroupLayout);

// Cacheable is implemented for every wgpu type because they're Clone + Hash +
// Eq

impl Cacheable for Material {
  type Output = String;

  fn hash(&self,) -> Self::Output {
    match self {
      Material::Opaque(opaque,) => opaque.name.clone(),
    }
  }
}
