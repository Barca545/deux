use crate::{
  data_lib::Position,
  math::{math::FlatMat4, Mat4, Vec3},
};
use bytemuck::{Pod, Zeroable};
use std::{
  mem,
  ops::{Range, RangeBounds},
};
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  vertex_attr_array, Buffer, BufferAddress, BufferSlice, BufferUsages, Device, VertexBufferLayout,
  VertexStepMode,
};

// TODO: I don't like this having a default implementation. Feels like it could
// cause random subtle errors. Is this necessary for some reason?
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Pod, Zeroable,)]
/// Struct containing position information needed for rendering an [instance](https://learnopengl.com/Advanced-OpenGL/Instancing) of a
/// [`Mesh`](super::mesh).
pub struct Instance {
  /// Position of the `Mesh` in space.
  position:FlatMat4,
}

impl Instance {
  // TODO: This is preferable to having the Instance implement Vertex but feels
  // clumsy.

  /// [`VertexBufferLayout`] for an [`Instance`]. Needed for converting an
  /// `Instance` into a value which can placed into a
  /// [`VertexBuffer`](crate::renderer::core::buffer::VertexBuffer).
  const DESCRIPTOR:VertexBufferLayout<'static,> = VertexBufferLayout {
    array_stride:mem::size_of::<Self,>() as BufferAddress,
    step_mode:VertexStepMode::Instance,
    attributes:&vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 =>
    Float32x4, 8 => Float32x4,],
  };

  /// Create a new [`Instance`].
  pub fn new(position:Vec3,) -> Self {
    let position = Mat4::new_translation(&position,).into();
    Instance { position, }
  }
}

impl From<Position,> for Instance {
  fn from(position:Position,) -> Self {
    Instance::new(position.0,)
  }
}

impl From<&Position,> for Instance {
  fn from(position:&Position,) -> Self {
    Instance::new(position.0,)
  }
}

// TODO: I kinda hate having this here. But I think having it in buffers is
// worse because I don't want core to have any dependencies on the other modules
pub struct InstanceBuffer {
  buffer:Buffer,
  pub len:u32,
}

impl InstanceBuffer {
  pub fn new(device:&Device, instances:&Vec<Instance,>,) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label:Some("Instance buffer",),
      contents:bytemuck::cast_slice(&instances,),
      usage:BufferUsages::VERTEX,
    },);

    InstanceBuffer {
      buffer,
      len:instances.len() as u32,
    }
  }

  /// Use only a portion of this Buffer for a given operation. Choosing a range
  /// with no end will use the rest of the buffer. Using a unbounded
  /// range will use the entire buffer.
  pub fn slice<S,>(&self, bounds:S,) -> BufferSlice
  where S: RangeBounds<BufferAddress,> {
    self.buffer.slice(bounds,)
  }
}

// TODO: Confirm this is the use of this struct
/// Collection of a group of [`Instance`]s of a [`Mesh`](super::mesh). Used for
/// collecting all instances of a mesh in the game world for rendering.
#[derive(Debug, Clone,)]
pub struct Instances(pub Vec<Instance,>,);

impl Instances {
  pub fn new() -> Self {
    Instances(Vec::new(),)
  }

  /// Returns a [`Range`] over the length of the `Instances`.
  pub fn range(&self,) -> Range<u32,> {
    0..self.0.len() as u32
  }
}
