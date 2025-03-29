use super::vertex::Vertex;
use std::ops::RangeBounds;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferAddress, BufferSlice, BufferUsages, Device};

// Refactor:
// - Revisit the labels
// - Update documentation for all structs so it is clearer
// - Vertex buffer contains all data going into the vertex shader (forex the
//   Instances) not just verticies, should it be renamed?

pub struct VertexBuffer {
  buffer:Buffer,
  pub len:u32,
}

impl VertexBuffer {
  pub fn new(device:&Device, vertices:&Vec<impl Vertex,>,) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label:Some("Vertex buffer",),
      contents:bytemuck::cast_slice(vertices,),
      usage:BufferUsages::VERTEX,
    },);
    VertexBuffer {
      buffer,
      len:vertices.len() as u32,
    }
  }

  /// Use only a portion of this Buffer for a given operation. Choosing a range
  /// with no end will use the rest of the buffer. Using a totally unbounded
  /// range will use the entire buffer.
  pub fn slice<S,>(&self, bounds:S,) -> BufferSlice
  where S: RangeBounds<BufferAddress,> {
    self.buffer.slice(bounds,)
  }
}

// TODO: Confirm the thing the index buffer acts on is the vertex buffer.
/// Buffer of indices used to slice into a [`VertexBuffer`]. Used for [index
/// rendering](https://eliemichel.github.io/LearnWebGPU/basic-3d-rendering/input-geometry/index-buffer.html).
pub struct IndexBuffer {
  buffer:Buffer,
  pub len:u32,
}

impl IndexBuffer {
  pub fn new(device:&Device, indices:&Vec<u32,>,) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label:Some("Index buffer",),
      contents:bytemuck::cast_slice(indices,),
      usage:BufferUsages::INDEX,
    },);
    IndexBuffer {
      buffer,
      len:indices.len() as u32,
    }
  }

  /// Use only a portion of this Buffer for a given operation. Choosing a range
  /// with no end will use the rest of the buffer. Using a totally unbounded
  /// range will use the entire buffer.
  pub fn slice<S,>(&self, bounds:S,) -> BufferSlice
  where S: RangeBounds<BufferAddress,> {
    self.buffer.slice(bounds,)
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
