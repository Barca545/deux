use crate::view::render_gl::{
  buffer::{IndexBuffer, VertexBuffer},
  Texture,
};
use std::ops::Range;
use wgpu::{BindGroup, IndexFormat, RenderPass};

pub struct Material {
  pub name: String,
  pub diffuse_texture: Texture,
  pub bind_group: BindGroup,
}

impl Material {
  pub fn new(name: &str, diffuse_texture: Texture, bind_group: BindGroup) -> Self {
    Material {
      name: String::from(name),
      diffuse_texture,
      bind_group,
    }
  }
}

pub struct Mesh {
  pub name: String,
  pub vertex_buffer: VertexBuffer,
  pub index_buffer: IndexBuffer,
  pub material: usize,
}

impl Mesh {
  pub fn new(name: &str, vertex_buffer: VertexBuffer, index_buffer: IndexBuffer, material: usize) -> Self {
    Mesh {
      name: name.to_string(),
      vertex_buffer,
      index_buffer,
      material,
    }
  }

  ///The range of indices to draw.
  pub fn indices_range(&self) -> Range<u32> {
    0..self.index_buffer.len
  }
}

pub struct Model {
  pub meshes: Vec<Mesh>,
  pub materials: Vec<Material>,
}

impl Model {
  pub fn new(meshes: Vec<Mesh>, materials: Vec<Material>) -> Self {
    Model { meshes, materials }
  }
}

///Exposes methods for rendering a single or multiple instances of a [`Model`].
pub trait DrawModel<'a> {
  fn draw_mesh(&mut self, mesh: &'a Mesh);
  fn draw_mesh_instanced(&mut self, mesh: &'a Mesh, instances: Range<u32>);
}

impl<'a, 'b> DrawModel<'b> for RenderPass<'a>
where
  'b: 'a,
{
  fn draw_mesh(&mut self, mesh: &'b Mesh) {
    self.draw_mesh_instanced(mesh, 0..1)
  }

  fn draw_mesh_instanced(&mut self, mesh: &'b Mesh, instances: Range<u32>) {
    //Buffer the vertices
    self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

    //Buffer the indices
    self.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint32);

    //Draw the mesh
    self.draw_indexed(mesh.indices_range(), 0, instances)
  }
}
