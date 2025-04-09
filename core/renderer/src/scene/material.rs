use crate::{cache::BindGroupKey, core::texture::Texture};
use wgpu::{BindGroupLayout, RenderPipeline};

// TODO: Rename bindgroupid so it doesn't cause collisions with the wgpu type

// TODO: I genuinely don't know how to document what a material is...
// I guess it's like a trait encompassing the abstract concept of the data the
// renderer neesd to render a type of mesh?
// pub trait Material {
//   /// Returns the name of the `Material`.
//   fn name(&self,) -> &String;
//   /// Returns a reference to `Material`'s [`BindGroupKey`].
//   fn bindgroup(&self,) -> BindGroupKey;
//   /// Returns a reference to `Material`'s the [`MaterialType`].
//   fn mtype(&self,) -> &MaterialType;
// }

#[derive(Debug,)]
/// An Opaque [`Material`].
pub struct OpaqueMaterial {
  // TODO: If the bindgroup owns the textures should the material
  /// The name of the `Material` used for debugging.
  pub name:String,
  /// The base color of the `Material` without considering the effects of
  /// lighting.
  albedo_texture:Texture,
  // diffuse_texture:Texture,
  // specular_texture:Texture,
  /// Handle to the `Material`'s [`BindGroup`](wgpu::BindGroup).
  bindgroup:BindGroupKey,
  mtype:MaterialType,
}

#[derive(Debug,)]
struct MaterialType {
  layout:BindGroupLayout,
  pipeline:RenderPipeline,
  state:RenderStage,
}

#[derive(Debug,)]
// TODO:Lucien says he things the render stage should own the pipeline, what
// does that look like in practice
pub enum RenderStage {
  Opaque,
  Transparent,
  Shadow,
}

#[derive(Debug,)]
pub enum Material {
  Opaque(OpaqueMaterial,),
}

impl Material {
  pub fn new(name:&str, albedo_texture:Texture, bind_group:BindGroupKey,) -> Self {
    todo!()
  }

  /// Returns the name of the `Material`.
  pub fn name(&self,) -> &String {
    match self {
      Material::Opaque(material,) => &material.name,
    }
  }

  /// Returns a reference to `Material`'s [`BindGroupKey`].
  pub fn bindgroup(&self,) -> BindGroupKey {
    match self {
      Material::Opaque(material,) => material.bindgroup,
    }
  }

  /// Returns a reference to `Material`'s the [`MaterialType`].
  pub fn mtype(&self,) -> &MaterialType {
    match self {
      Material::Opaque(material,) => &material.mtype,
    }
  }
}

// // TODO: Need a better name
// pub struct UnlitMaterial {
//   /// Name for debugging.
//   pub name:String,
//   /// The [`Texture`] component of the `Material`'s color before lighting.
//   pub base_color_texture:Texture,
//   /// A Handle to the `Material`'s [`BindGroup`](wgpu::BindGroup) entry in
// the   /// [`BindGroupCache`](crate::renderer::bindgroupcache::BindGroupCache).
//   bindgroup:BindGroupKey,
//   /// The [`MaterialType`] of the `Material`.
//   mtype:MaterialType,
// }

// impl UnlitMaterial {
//   pub fn new(
//     name:&str,
//     base_color_texture:Texture,
//     bindgroup:BindGroupKey,
//     mtype:MaterialType,
//   ) -> Self {
//     UnlitMaterial {
//       name:String::from(name,),
//       base_color_texture,
//       bindgroup,
//       mtype,
//     }
//   }
// }

// impl Material for UnlitMaterial {
//   fn name(&self,) -> &String {
//     &self.name
//   }

//   fn bindgroup(&self,) -> BindGroupKey {
//     self.bindgroup
//   }

//   fn mtype(&self,) -> &MaterialType {
//     &self.mtype
//   }
// }
