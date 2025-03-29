use crate::renderer::core::vertex::Vertex;
use bytemuck::{Pod, Zeroable};
use std::{
  hash::{Hash, Hasher},
  mem,
};
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode};

#[derive(Copy, Clone, Debug, Pod, Zeroable,)]
#[repr(C)]
pub struct UiVertex {
  /// The location of the vertex.
  pub(crate) pos:[f32; 3],
  /// The texture coordinates of the vertex.
  pub(crate) txt:[f32; 2],
  /// The color of the vertex.
  pub(crate) clr:[f32; 3],
}

impl Hash for UiVertex {
  fn hash<H:Hasher,>(&self, state:&mut H,) {
    self.pos[1].to_bits().hash(state,);
    self.pos[2].to_bits().hash(state,);
    self.pos[3].to_bits().hash(state,);

    self.txt[1].to_bits().hash(state,);
    self.txt[2].to_bits().hash(state,);
    self.txt[3].to_bits().hash(state,);

    self.clr[1].to_bits().hash(state,);
    self.clr[2].to_bits().hash(state,);
    self.clr[3].to_bits().hash(state,);
  }
}

impl Vertex for UiVertex {
  // TODO: Describe what this is needed for (second sentence) better
  /// [`VertexBufferLayout`] for a [`UiVertex`]. Needed for converting a
  /// `UiVertex` into a value which can placed into a
  /// [`VertexBuffer`](crate::renderer::core::buffer::VertexBuffer).
  const DESCRIPTOR:wgpu::VertexBufferLayout<'static,> = VertexBufferLayout {
    array_stride:mem::size_of::<Self,>() as BufferAddress,
    step_mode:VertexStepMode::Vertex,
    attributes:&vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3],
  };
}

impl UiVertex {
  pub fn new(pos:[f32; 3], txt:[f32; 2], clr:[f32; 3],) -> Self {
    UiVertex { pos, txt, clr, }
  }
}
