use crate::Instance;

use super::{gpu_context::GpuContext, vertex::VertexBufferData};
use std::ops::RangeBounds;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  wgt::BufferDescriptor,
  Buffer, BufferAddress, BufferSlice, BufferUsages, CommandEncoder, Device,
};

// Refactor:
// - Revisit the labels
// - Do I need to take into the vertex attributes

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
  buffer: Option<Buffer,>,
  cap: u64,
  len: u64,
}

impl InstanceBuffer {
  /// Create a new unallocated [`InstanceBuffer`].
  pub fn new() -> Self {
    InstanceBuffer {
      buffer: None,
      cap: 0,
      len: 0,
    }
  }

  pub fn len(&self,) -> u64 {
    self.len
  }

  /// A slice of the [`InstanceBuffer`].
  /// # Parameters
  /// - Choosing a range with no end will use the rest of the
  /// - And unbounded range will use the entire `InstanceBuffer`.
  pub fn slice<S,>(&self, bounds: S,) -> BufferSlice
  where
    S: RangeBounds<BufferAddress,>,
  {
    self.buffer().unwrap().slice(bounds,)
  }

  /// Buffer data into a new [`InstanceBuffer`].
  pub fn push_instances(
    &mut self,
    ctx: &GpuContext,
    encoder: &mut CommandEncoder,
    data: &Vec<Instance,>,
  ) {
    // Offsets are in bytes so to get the offset to start at you have to multiple
    // the len by the size of the contents
    let offset = self.len * size_of::<Instance,>() as u64;

    match self.buffer {
      Some(ref buffer,) => {
        // Check if the remaining space
        if self.cap >= self.len + data.len() as u64 {
          // If there is remaining space map the buffer and write to it
          ctx
            .queue
            .write_buffer(buffer, offset, bytemuck::cast_slice(&data,),);

          // Update the len
          self.len += data.len() as u64;
        } else {
          // Reallocate if there is not enough remaining cap realloc
          self.realloc(ctx, encoder, data,);
        }
      }
      None => self.alloc(ctx, data,),
    };
  }

  /// Create a [`wgpu::Buffer`] to store incoming [`Instance`]s.
  fn alloc(&mut self, ctx: &GpuContext, data: &Vec<Instance,>,) {
    // Set the internal buffer to a newly created buffer
    self.buffer = Some(ctx.device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Instance buffer",),
      contents: bytemuck::cast_slice(&data,),
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
    },),);

    // Updated the len of the InstanceBuffer
    self.len = data.len() as u64;
    self.cap = data.len() as u64
  }

  /// Reallocate the [`InstanceBuffer`]. Move the data in the `InstanceBuffer`
  /// to the newly allocated buffer.
  fn realloc(
    &mut self, ctx: &GpuContext, encoder: &mut CommandEncoder, new_data: &Vec<Instance,>,
  ) {
    // Calculate the size of the new buffer
    // Size needs to be multiplied by the size of the Instance data because the
    // buffer interprets it as bytes
    let new_len = self.len + new_data.len() as u64;

    // Get the src buffer
    let src = self.buffer().unwrap();

    // Allocate a new buffer with size = self.len + instances.len()
    let dst = ctx.device.create_buffer(&BufferDescriptor {
      label: Some("Instance buffer",),
      // Size has to be multipled by the size of instance because len is in number of items not
      // bytes as the function expects
      size: new_len * size_of::<Instance,>() as u64,
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
      mapped_at_creation: false,
    },);

    // Offsets are in bytes so to get the offset to start at you have to multiple
    // the len by the size of the contents
    let copy_size = self.len * size_of::<Instance,>() as u64;
    // Copy the old data into the new buffer
    encoder.copy_buffer_to_buffer(src, 0, &dst, 0, copy_size,);

    // Drop the old buffer
    src.destroy();

    // Write the new data into the new buffer
    ctx
      .queue
      .write_buffer(&dst, copy_size, bytemuck::cast_slice(&new_data,),);

    // Update the values of the structure
    self.buffer = Some(dst,);
    self.len = new_len;
    self.cap = new_len;
  }

  /// Returns a reference to [`InstanceBuffer::buffer`], the `Buffer`'s
  /// underlying [`wgpu::Buffer`].
  ///
  /// # Panics
  /// Panics if `buffer` is `None`.
  fn buffer(&self,) -> Option<&Buffer,> {
    match self.buffer {
      Some(ref buffer,) => Some(buffer,),
      None => None,
    }
  }

  // TODO: When the buffer becomes a resource this must be called at the end or
  // beginning of each frame
  /// Clear the `InstanceBuffer`.
  pub fn clear(&mut self,) {
    self.len = 0;
  }
}
