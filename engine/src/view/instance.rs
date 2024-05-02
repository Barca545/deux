use crate::math::{Mat4, Vec3};
use bytemuck::{Pod, Zeroable};
use std::{mem, ops::Range};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

//Get rid of the model transform from the Transforms section
//Since the projection for my game is so simple I think I can just make
//projection part of the camera again
//InstanceRaw will become part of the mesh?

//A Mat4 takes up 4 vertex slots as it is technically 4 vec4s.
//We need to define a slot for each vec4.
//We'll have to reassemble the mat4 in the shader.
const INST_ATTRIBS: [VertexAttribute; 4] = vertex_attr_array![5 => Float32x4,6 => Float32x4,7 => Float32x4,8 => Float32x4,];

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
///Struct containing position information needed for rendering an instance of a `Mesh`.
pub struct InstanceRaw {
  model: [[f32; 4]; 4],
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

pub trait Instances {
  fn range(&self) -> Range<u32>;
}

impl Instances for Vec<InstanceRaw> {
  ///Returns a [`Range`] over the length of the instances.
  fn range(&self) -> Range<u32> {
    0..self.len() as u32
  }
}
