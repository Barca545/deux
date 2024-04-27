use bytemuck::{Pod, Zeroable};
use std::hash::{Hash, Hasher};
use std::mem;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

// Refactor:
// -Use #[repr(C, packed)] instead of just #[repr(C)]?

///Vertex Attributes for a [`ModelVertex`].
const MODEL_ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

///Vertex Attributes for a [`DebugVertex`].
const DEBUG_ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];

pub trait Vertex: Copy + Clone + Hash + Pod + Zeroable {
  fn desc() -> VertexBufferLayout<'static>;
}

#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct ModelVertex {
  pub(crate) pos: [f32; 3],
  pub(crate) txt: [f32; 2],
}

impl PartialEq for ModelVertex {
  fn eq(&self, other: &Self) -> bool {
    self.pos == other.pos && self.txt == other.txt
  }
}

impl Eq for ModelVertex {}

impl Hash for ModelVertex {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pos[0].to_bits().hash(state);
    self.pos[1].to_bits().hash(state);
    self.pos[2].to_bits().hash(state);

    self.txt[0].to_bits().hash(state);
    self.txt[1].to_bits().hash(state);
  }
}

impl Vertex for ModelVertex {
  fn desc() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &MODEL_ATTRIBS,
    }
  }
}

impl From<(f32, f32, f32, f32, f32)> for ModelVertex {
  fn from(value: (f32, f32, f32, f32, f32)) -> Self {
    let pos: [f32; 3] = [value.0, value.1, value.2];
    let txt: [f32; 2] = [value.3, value.4];
    Self::new(pos, txt)
  }
}

impl ModelVertex {
  pub fn new(pos: [f32; 3], txt: [f32; 2]) -> Self {
    ModelVertex { pos, txt }
  }
}

pub struct UiVertex {}

#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct DebugVertex {
  pub(crate) pos: [f32; 3],
  pub(crate) clr: [f32; 3],
}

impl PartialEq for DebugVertex {
  fn eq(&self, other: &Self) -> bool {
    self.pos == other.pos && self.clr == other.clr
  }
}

impl Eq for DebugVertex {}

impl Hash for DebugVertex {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pos[0].to_bits().hash(state);
    self.pos[1].to_bits().hash(state);
    self.pos[2].to_bits().hash(state);

    self.clr[0].to_bits().hash(state);
    self.clr[1].to_bits().hash(state);
    self.clr[2].to_bits().hash(state);
  }
}

impl Vertex for DebugVertex {
  fn desc() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as BufferAddress,
      step_mode: VertexStepMode::Vertex,
      attributes: &DEBUG_ATTRIBS,
    }
  }
}

impl From<(f32, f32, f32, f32, f32, f32)> for DebugVertex {
  fn from(value: (f32, f32, f32, f32, f32, f32)) -> Self {
    let pos: [f32; 3] = [value.0, value.1, value.2];
    let clr: [f32; 3] = [value.3, value.4, value.5];
    Self::new(pos, clr)
  }
}

impl DebugVertex {
  pub fn new(pos: [f32; 3], clr: [f32; 3]) -> Self {
    DebugVertex { pos, clr }
  }
}
