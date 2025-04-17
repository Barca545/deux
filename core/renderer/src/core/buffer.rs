use super::{gpu_context::GpuContext, vertex::VertexBufferData};
use crate::Instance;
use std::ops::RangeBounds;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  Buffer, BufferAddress, BufferSlice, BufferUsages, Device,
};

// Refactor:
// - Revisit the labels
// - Do I need to take into the vertex attributes
// - Readd the functionality to make the instance buffer reallocated instead of
//   creating a new one each frame. Solid idea but reallocating the instance
//   buffer would cause it to destroy itself before queue submit was called

// TODO: Where are the vertex attributes

#[derive(Debug,)]
pub struct VertexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl VertexBuffer {
  pub fn new(device: &Device, vertices: &Vec<impl VertexBufferData,>,) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Vertex buffer",),
      contents: bytemuck::cast_slice(vertices,),
      usage: BufferUsages::VERTEX,
    },);
    VertexBuffer {
      buffer,
      len: vertices.len() as u32,
    }
  }

  /// Use only a portion of this [`Buffer`](wgpu::Buffer) for a given operation.
  /// Choosing a range with no end will use the rest of the `Buffer`. Using a
  /// unbounded range will use the entire `Buffer`.
  pub fn slice<S,>(&self, bounds: S,) -> BufferSlice
  where
    S: RangeBounds<BufferAddress,>,
  {
    self.buffer.slice(bounds,)
  }
}

#[derive(Debug,)]
// TODO: Confirm the thing the index buffer acts on is the vertex buffer.
/// Buffer of indices used to slice into a [`VertexBuffer`]. Used for [index
/// rendering](https://eliemichel.github.io/LearnWebGPU/basic-3d-rendering/input-geometry/index-buffer.html).
pub struct IndexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl IndexBuffer {
  pub fn new(device: &Device, indices: &Vec<u32,>,) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Index buffer",),
      contents: bytemuck::cast_slice(indices,),
      usage: BufferUsages::INDEX,
    },);
    IndexBuffer {
      buffer,
      len: indices.len() as u32,
    }
  }

  /// Use only a portion of this [`Buffer`](wgpu::Buffer) for a given operation.
  /// Choosing a range with no end will use the rest of the `Buffer`. Using a
  /// unbounded range will use the entire `Buffer`.
  pub fn slice<S,>(&self, bounds: S,) -> BufferSlice
  where
    S: RangeBounds<BufferAddress,>,
  {
    self.buffer.slice(bounds,)
  }
}

/// [Buffer] containing all of a [`Scene`](crate::scene::Scene)'s
/// [`Instance`](crate::core::instance::Instance)s. Used for [instance rendering](https://learnopengl.com/Advanced-OpenGL/Instancing)
#[derive(Debug,)]
pub struct InstanceBuffer {
  buffer: Buffer,
  len: u64,
}

impl InstanceBuffer {
  /// Create a new allocated [`InstanceBuffer`].
  pub fn new(ctx: &GpuContext, data: &Vec<Instance,>,) -> InstanceBuffer {
    // Set the internal buffer to a newly created buffer
    let buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Instance buffer",),
      contents: bytemuck::cast_slice(&data,),
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
    },);

    InstanceBuffer {
      buffer,
      len: data.len() as u64,
    }
  }

  /// Returns a reference to [`InstanceBuffer::buffer`], the `Buffer`'s
  /// underlying [`wgpu::Buffer`].
  fn buffer(&self,) -> &Buffer {
    &self.buffer
  }

  /// Return the number of [`Instance`]s in the `InstanceBuffer`.
  pub fn len(&self,) -> u64 {
    self.len
  }

  /// A slice of the [`InstanceBuffer`].
  /// # Parameters
  /// - Choosing a range with no end will use the rest of the
  /// - An unbounded range will use the entire `InstanceBuffer`.
  pub fn slice<S,>(&self, bounds: S,) -> BufferSlice
  where
    S: RangeBounds<BufferAddress,>,
  {
    self.buffer().slice(bounds,)
  }
}
