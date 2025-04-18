use crate::{scene::mesh::Mesh, utils::resources::MaterialKey};
use storage::ArenaId;

#[derive(Debug, Default,)]
// TODO: Document what a model is
pub struct Model {
  // TODO: Does a model need an ID?
  pub meshes: Vec<Mesh,>,
  // TODO: If Mesh holds its material does the model also need to hold it?
  pub materials: Vec<MaterialKey,>,
}

impl Model {
  pub fn new(meshes: Vec<Mesh,>, materials: Vec<MaterialKey,>,) -> Self {
    Model { meshes, materials, }
  }
}

/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) for the ID of a [`Model`].
pub type ModelId = ArenaId<Model,>;
// #[derive(Debug, Clone, Copy,)]
// pub struct ModelId(usize,);

// TODO: Figure out what the a lifetime represents and give it a better name
// I think it might be the lifetime of the models?
// /// Exposes methods for rendering a single or multiple instances of a
// [`Model`]. pub trait DrawModel<'models,> {
//   /// Render a [`Mesh`].
//   fn draw_mesh(
//     &mut self,
//     // TODO: Need to pass in the bindgroupcache not sure this is the best way
//     bindgroups:&'models BindGroupCache,
//     mesh:&'models Mesh,
//     material:&'models Material,
//   );

//   /// Render all [`Instance`](crate::renderer::scene::instance::Instance)s of
// a   /// [`Mesh`] in the given range of the
//   /// [`VertexBuffer`](crate::renderer::core::buffer::VertexBuffer).
//   fn draw_mesh_instanced(
//     &mut self,
//     // TODO: Need to pass in the bindgroupcache not sure this is the best way
//     bindgroups:&'models BindGroupCache,
//     mesh:&'models Mesh,
//     material:&'models Material,
//     instances:Range<u32,>,
//   );

//   /// Render all [Instance](crate::renderer::scene::instance::Instance)s of a
//   /// [`Model`] in the given range of the
//   /// [`VertexBuffer`](crate::renderer::core::buffer::VertexBuffer).
//   fn draw_model_instanced(
//     &mut self, // TODO: Need to pass in the bindgroupcache not sure this is
// the best way     bindgroups:&'models BindGroupCache,
//     model:&'models Model,
//     instances:Range<u32,>,
//   );
// }

// // TODO: See if I need to include any other information on renderpasses like
// // Lucien does in yakui
// impl<'pass, 'models,> DrawModel<'models,> for RenderPass<'pass,>
// where 'models:'pass
// {
//   fn draw_mesh(
//     &mut self,
//     // TODO: Need to pass in the bindgroupcache not sure this is the best way
//     bindgroups:&'models BindGroupCache,
//     mesh:&'models Mesh,
//     material:&'pass Material,
//   ) {
//     self.draw_mesh_instanced(bindgroups, mesh, material, 0..1,)
//   }

//   fn draw_mesh_instanced(
//     &mut self,
//     // TODO: Need to pass in the bindgroupcache not sure this is the best way
//     bindgroups:&'models BindGroupCache,
//     mesh:&'models Mesh,
//     material:&'pass Material,
//     instances:Range<u32,>,
//   ) {
//     // Buffer the vertices
//     self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..,),);

//     // Buffer the indices
//     self.set_index_buffer(mesh.index_buffer.slice(..,),
// IndexFormat::Uint32,);

//     // Set the Texture bind group
//     self.set_bind_group(0, bindgroups.get(material.bindgroup(),).0, &[],);

//     // Draw the mesh
//     self.draw_indexed(mesh.indices_range(), 0, instances,)
//   }

//   fn draw_model_instanced(
//     &mut self,
//     // TODO: Need to pass in the bindgroupcache not sure this is the best way
//     bindgroups:&'models BindGroupCache,
//     model:&'models Model,
//     instances:Range<u32,>,
//   ) {
//     // Iterate over the model's submeshes and render each one.
//     for mesh in &model.meshes {
//       let material = &model.materials[mesh.material];
//       self.draw_mesh_instanced(bindgroups, mesh, material,
// instances.clone(),);     }
//   }
// }
