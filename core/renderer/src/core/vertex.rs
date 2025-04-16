use bytemuck::{Pod, Zeroable};
use std::{
  hash::{Hash, Hasher},
  mem,
};
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode};

// Refactor:
// - Use #[repr(C, packed)] instead of just #[repr(C)]?
// - Why do verts need to be hashable? Is it to preserve uniqueness?
// - Does hash *need* to be used for a vertex?

/// Describes the behavior of data submitted to a
/// [`VertexBuffer`](super::buffer::VertexBuffer). Needed so the `VertexBuffer`
/// accepts all data types without requiring a unique version for each
/// type.
///
/// Data which is submitted to the `VertexBuffer` must implement [`Pod`] +
/// [`Zeroable`] so they can be converted into `&[u8]` using
/// [`bytemuck::cast_slice`] when inserted into a `VertexBuffer`.
// TODO: Could this be implemented by the buffers themselves?
pub trait VertexBufferData: Copy + Clone + Pod + Zeroable {
  const BUFFER_LAYOUT: VertexBufferLayout<'static,>;
}

#[derive(Copy, Clone, Debug, Pod, Zeroable,)]
#[repr(C)]
/// Vertex belonging to an in-game [`Model`](crate::scene::model::Model).
pub struct ModelVertex {
  /// The location of the vertex.
  pub(crate) pos: [f32; 3],
  /// The texture coordinates of the vertex.
  pub(crate) txt: [f32; 2],
}

impl PartialEq for ModelVertex {
  fn eq(&self, other: &Self,) -> bool {
    self.pos == other.pos && self.txt == other.txt
  }
}

impl Eq for ModelVertex {}

impl Hash for ModelVertex {
  fn hash<H: Hasher,>(&self, state: &mut H,) {
    self.pos[0].to_bits().hash(state,);
    self.pos[1].to_bits().hash(state,);
    self.pos[2].to_bits().hash(state,);

    self.txt[0].to_bits().hash(state,);
    self.txt[1].to_bits().hash(state,);
  }
}

impl VertexBufferData for ModelVertex {
  // TODO: Describe what this is needed for (second sentence) better
  /// [`VertexBufferLayout`] for a [`ModelVertex`]. Needed for converting a
  /// `ModelVertex` into a value which can placed into a
  /// [`VertexBuffer`](super::buffer::VertexBuffer).
  const BUFFER_LAYOUT: VertexBufferLayout<'static,> = VertexBufferLayout {
    array_stride: mem::size_of::<Self,>() as BufferAddress,
    step_mode: VertexStepMode::Vertex,
    attributes: &vertex_attr_array![0 => Float32x3, 1 => Float32x2],
  };
}

impl From<(f32, f32, f32, f32, f32,),> for ModelVertex {
  fn from(value: (f32, f32, f32, f32, f32,),) -> Self {
    let pos: [f32; 3] = [value.0, value.1, value.2,];
    let txt: [f32; 2] = [value.3, value.4,];
    Self::new(pos, txt,)
  }
}

impl ModelVertex {
  pub fn new(pos: [f32; 3], txt: [f32; 2],) -> Self {
    ModelVertex { pos, txt, }
  }
}

#[derive(Copy, Clone, Debug, Pod, Zeroable,)]
#[repr(C)]
pub struct DebugVertex {
  pub(crate) pos: [f32; 3],
  pub(crate) clr: [f32; 3],
}

impl PartialEq for DebugVertex {
  fn eq(&self, other: &Self,) -> bool {
    self.pos == other.pos && self.clr == other.clr
  }
}

impl Eq for DebugVertex {}

impl Hash for DebugVertex {
  fn hash<H: Hasher,>(&self, state: &mut H,) {
    self.pos[0].to_bits().hash(state,);
    self.pos[1].to_bits().hash(state,);
    self.pos[2].to_bits().hash(state,);

    self.clr[0].to_bits().hash(state,);
    self.clr[1].to_bits().hash(state,);
    self.clr[2].to_bits().hash(state,);
  }
}

impl VertexBufferData for DebugVertex {
  // TODO: Describe what this is needed for (second sentence) better
  /// [`VertexBufferLayout`] for a [`DebugVertex`]. Needed for converting a
  /// `DebugVertex` into a value which can placed into a
  /// [`VertexBuffer`](super::buffer::VertexBuffer).
  const BUFFER_LAYOUT: VertexBufferLayout<'static,> = VertexBufferLayout {
    array_stride: mem::size_of::<Self,>() as BufferAddress,
    step_mode: VertexStepMode::Vertex,
    attributes: &vertex_attr_array![0 => Float32x3, 1 => Float32x3],
  };
}

impl From<(f32, f32, f32, f32, f32, f32,),> for DebugVertex {
  fn from(value: (f32, f32, f32, f32, f32, f32,),) -> Self {
    let pos: [f32; 3] = [value.0, value.1, value.2,];
    let clr: [f32; 3] = [value.3, value.4, value.5,];
    Self::new(pos, clr,)
  }
}

impl DebugVertex {
  pub fn new(pos: [f32; 3], clr: [f32; 3],) -> Self {
    DebugVertex { pos, clr, }
  }
}
