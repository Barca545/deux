use super::{material::Material, mesh::Mesh};
use std::ops::Range;
use wgpu::{IndexFormat, RenderPass};

// TODO: Document what a model is

pub struct Model {
  pub meshes:Vec<Mesh,>,
  pub materials:Vec<Material,>,
}

impl Model {
  pub fn new(meshes:Vec<Mesh,>, materials:Vec<Material,>,) -> Self {
    Model { meshes, materials, }
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq,)]
/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) for the ID of a [`Model`].
pub struct ModelId(pub usize,);

/// Exposes methods for rendering a single or multiple instances of a [`Model`].
pub trait DrawModel<'a,> {
  /// Render a [`Mesh`].
  fn draw_mesh(&mut self, mesh:&'a Mesh, material:&'a Material,);

  /// Render all [`Instance`](crate::renderer::scene::instance::Instance)s of a
  /// [`Mesh`] in the given range of the
  /// [`VertexBuffer`](crate::renderer::core::buffer::VertexBuffer).
  fn draw_mesh_instanced(&mut self, mesh:&'a Mesh, material:&'a Material, instances:Range<u32,>,);

  /// Render all [Instance](crate::renderer::scene::instance::Instance)s of a
  /// [`Model`] in the given range of the
  /// [`VertexBuffer`](crate::renderer::core::buffer::VertexBuffer).
  fn draw_model_instanced(&mut self, model:&'a Model, instances:Range<u32,>,);
}

// TODO: See if I need to include any other information on renderpasses like
// Lucien does in yakui
impl<'a, 'b,> DrawModel<'b,> for RenderPass<'a,>
where 'b:'a
{
  fn draw_mesh(&mut self, mesh:&'b Mesh, material:&'a Material,) {
    self.draw_mesh_instanced(mesh, material, 0..1,)
  }

  fn draw_mesh_instanced(&mut self, mesh:&'b Mesh, material:&'a Material, instances:Range<u32,>,) {
    // Buffer the vertices
    self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..,),);

    // Buffer the indices
    self.set_index_buffer(mesh.index_buffer.slice(..,), IndexFormat::Uint32,);

    // Set the Texture bind group
    self.set_bind_group(0, &material.bind_group, &[],);

    // Draw the mesh
    self.draw_indexed(mesh.indices_range(), 0, instances,)
  }

  fn draw_model_instanced(&mut self, model:&'b Model, instances:Range<u32,>,) {
    // Iterate over the model's submeshes and render each one.
    for mesh in &model.meshes {
      let material = &model.materials[mesh.material];
      self.draw_mesh_instanced(mesh, material, instances.clone(),);
    }
  }
}
