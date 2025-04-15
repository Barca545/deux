use super::{gpu_context::GpuContext, instance::Instances, vertex::Vertex};
use std::ops::RangeBounds;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  wgt::BufferDescriptor,
  Buffer, BufferAddress, BufferSlice, BufferUsages, CommandEncoder, Device,
};

// Refactor:
// - Revisit the labels
// - Update documentation for all structs so it is clearer

#[derive(Debug,)]
pub struct VertexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl VertexBuffer {
  pub fn new(device: &Device, vertices: &Vec<impl Vertex,>,) -> Self {
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

// /// [Buffer] containing all the
// [`Instances`](crate::core::instance::Instances) /// to draw in a scene. Used for [instance rendering](https://learnopengl.com/Advanced-OpenGL/Instancing)
// pub struct InstanceBuffer {
//   buffer: Buffer,
//   pub len: u32,
// }

// impl InstanceBuffer {
//   pub fn new(device: &Device, instances: &Instances,) -> Self {
//     let buffer = device.create_buffer_init(&BufferInitDescriptor {
//       label: Some("Instance buffer",),
//       contents: bytemuck::cast_slice(&instances.0,),
//       usage: BufferUsages::VERTEX,
//     },);

//     InstanceBuffer {
//       buffer,
//       len: instances.0.len() as u32,
//     }
//   }

//   /// Use only a portion of this [`Buffer`](wgpu::Buffer) for a given
// operation.   /// Choosing a range with no end will use the rest of the
// `Buffer`. Using a   /// unbounded range will use the entire `Buffer`.
//   pub fn slice<S,>(&self, bounds: S,) -> BufferSlice
//   where
//     S: RangeBounds<BufferAddress,>,
//   {
//     self.buffer.slice(bounds,)
//   }
// }

/// [Buffer] containing all the [`Instances`](crate::core::instance::Instances)
/// to draw in a scene. Used for [instance rendering](https://learnopengl.com/Advanced-OpenGL/Instancing)
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
    data: &Instances,
  ) {
    match self.buffer {
      Some(ref buffer,) => {
        // Check if the remaining space
        if (self.cap - self.len - data.len() as u64) != 0 {
          // If there is remaining space map the buffer and write to it
          ctx
            .queue
            .write_buffer(buffer, self.len, bytemuck::cast_slice(&data.0,),);
        } else {
          // Reallocate if there is not enough remaining cap realloc
          self.realloc(ctx, encoder, data,);
        }
      }
      None => self.alloc(ctx, data,),
    };
  }

  /// Create a [`wgpu::Buffer`] to store incoming [`Instances`] d
  fn alloc(&mut self, ctx: &GpuContext, data: &Instances,) {
    // Set the internal buffer to a newly created buffer
    self.buffer = Some(ctx.device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Instance buffer",),
      contents: bytemuck::cast_slice(&data.0,),
      usage: BufferUsages::VERTEX
        | BufferUsages::COPY_DST
        | BufferUsages::COPY_SRC
        // | BufferUsages::MAP_WRITE,
    },),);
    // Updated the len of the InstanceBuffer
    self.len = data.len() as u64;
    self.cap = data.len() as u64
  }

  /// Reallocate the [`InstanceBuffer`]. Move the data in the `InstanceBuffer`
  fn realloc(&mut self, ctx: &GpuContext, encoder: &mut CommandEncoder, new_data: &Instances,) {
    // Calculate the size of the new buffer
    let size = self.len + new_data.len() as u64;

    // Get the src buffer
    let src = self.buffer().unwrap();

    // Allocate a new buffer with size = self.len + instances.len()
    let dst = ctx.device.create_buffer(&BufferDescriptor {
      label: Some("Instance buffer",),
      size,
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
      // | BufferUsages::MAP_WRITE,
      mapped_at_creation: false,
    },);

    // Copy the old data into the new buffer
    encoder.copy_buffer_to_buffer(src, 0, &dst, 0, self.len as u64,);

    // Drop the old buffer
    src.destroy();

    // Copy the new data into the new buffer
    ctx
      .queue
      .write_buffer(&dst, self.len as u64, bytemuck::cast_slice(&new_data.0,),);

    // Unmap the buffer otherwise it will cause issues with later stages
    // dst.unmap();

    // Update the values of the structure
    self.buffer = Some(dst,);
    self.len = size;
    self.cap = size;
  }

  // TODO: Get the buffer link working
  /// Returns a reference to the `Buffer`'s underlying [`wgpu::Buffer`].
  ///
  /// # Panics
  /// - Panics if [`buffer`](InstanceBuffer::buffer) is
  /// unallocated.
  fn buffer(&self,) -> Option<&Buffer,> {
    match self.buffer {
      Some(ref buffer,) => Some(buffer,),
      None => None,
    }
  }
}

// TODO: Confirm it's all elements
// UI buffer seems unnecessary since Yakui seems to just use normal indices and
// vertex buffers
// Do need to figure out why it has the cpu data vs gpu data thing going on
// Since the VertexBuffer accepts all data going into the vertex shader not just
// actual verticies should it be renamed?

// /// Buffer containing all elements of the UI.
// struct UiBuffer {
//   buffer:Buffer,
//   len:usize,
// }
