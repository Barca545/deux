use crate::{
  component_lib::Position,
  math::{math::FlatMat4, Mat4, Vec3},
};
use bytemuck::{Pod, Zeroable};
use std::{mem, ops::Range};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

///Layout of an [`InstanceRaw`]'s matrix in the GPU. As a [`Mat4`] i.e. 4 vec4s, an `InstanceRaw` takes up four slots.
const INST_ATTRIBS: [VertexAttribute; 4] = vertex_attr_array![5 => Float32x4,6 => Float32x4,7 => Float32x4,8 => Float32x4,];

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod, Zeroable)]
///Struct containing position information needed for rendering an instance of a `Mesh`.
pub struct InstanceRaw {
  model: FlatMat4,
}

impl InstanceRaw {
  ///Create a new [`InstanceRaw`].
  pub fn new(position: Vec3) -> Self {
    let model = Mat4::new_translation(&position).into();
    InstanceRaw { model }
  }

  ///Get the [`VertexBufferLayout`] of the [`InstanceRaw`] buffer.
  pub fn desc() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as BufferAddress,
      step_mode: VertexStepMode::Instance,
      attributes: &INST_ATTRIBS,
    }
  }
}

impl From<Position> for InstanceRaw {
  fn from(position: Position) -> Self {
    InstanceRaw::new(position.0)
  }
}

impl From<&Position> for InstanceRaw {
  fn from(position: &Position) -> Self {
    InstanceRaw::new(position.0)
  }
}

pub trait Instances {
  fn range(&self) -> Range<u32>;
}

impl Instances for Vec<InstanceRaw> {
  ///Returns a [`Range`] over the length of the instances.
  fn range(&self) -> Range<u32> {
    0..self.len() as u32
  }
}
