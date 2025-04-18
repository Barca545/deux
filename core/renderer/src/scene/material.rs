use crate::{core::texture::Texture, utils::resources::BindGroupKey};
use wgpu::BindGroupLayout;

// TODO: I genuinely don't know how to document what a material is...
// I guess it's like a trait encompassing the abstract concept of the data the
// renderer neesd to render a type of mesh?
#[derive(Debug,)]
/// An Opaque [`Material`].
pub struct OpaqueMaterial {
  // TODO: If the bindgroup owns the textures should the material
  /// The name of the `Material` used for debugging.
  pub name: String,
  /// The base color of the `Material` without considering the effects of
  /// lighting.
  albedo_texture: Texture,
  // diffuse_texture:Texture,
  // specular_texture:Texture,
  /// Handle to the `Material`'s [`BindGroup`](wgpu::BindGroup).
  bindgroup: BindGroupKey,
  // mtype: MaterialType,
}

#[derive(Debug,)]
struct MaterialType {
  pub layout: BindGroupLayout,
  // pub pipeline: RenderPipelineKey,
}

#[derive(Debug,)]
pub enum Material {
  Opaque(OpaqueMaterial,),
}

impl Material {
  pub fn new(
    name: &str,
    albedo_texture: Texture,
    bindgroup: BindGroupKey,
    // pipeline: PipelineCacheId,
  ) -> Self {
    Material::Opaque(OpaqueMaterial {
      name: name.to_string(),
      albedo_texture,
      bindgroup,
      // mtype: MaterialType {
      //   // I *think* this can be preset
      //   layout: todo!(),
      //   // pipeline,
      // },
    },)
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

  // /// Returns a reference to `Material`'s the [`MaterialType`].
  // pub fn mtype(&self,) -> &MaterialType {
  //   match self {
  //     Material::Opaque(material,) => &material.mtype,
  //   }
  // }
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
