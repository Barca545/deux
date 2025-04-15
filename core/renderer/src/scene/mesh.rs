use crate::{
  core::buffer::{IndexBuffer, VertexBuffer},
  utils::cache::MaterialKey,
};
use std::ops::Range;

#[derive(Debug,)]
/// The [`VertexBuffer`] and [IndexBuffer] which define a single [mesh](https://en.wikipedia.org/wiki/Polygon_mesh) in the
/// game.
pub struct Mesh {
  /// Name of the `Mesh`. Used for debugging.
  pub name: String,
  pub vertex_buffer: VertexBuffer,
  pub index_buffer: IndexBuffer,
  /// The identifier for the `Mesh`'s [`Material`](super::material::Material).
  pub material: MaterialKey,
}

impl Mesh {
  pub fn new(
    name: &str,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    material: MaterialKey,
  ) -> Self {
    Mesh {
      name: name.to_string(),
      vertex_buffer,
      index_buffer,
      material,
    }
  }

  /// The range of indices to draw.
  pub fn indices_range(&self,) -> Range<u32,> {
    0..self.index_buffer.len
  }
}
