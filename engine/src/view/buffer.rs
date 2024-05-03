use super::Vertex;
use crate::view::InstanceRaw;
use std::ops::RangeBounds;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferAddress, BufferSlice, BufferUsages, Device};

// Refactor:
// -Revisit the labels

pub struct VertexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl VertexBuffer {
  pub fn new(device: &Device, vertices: &Vec<impl Vertex>) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("vertex buffer"),
      contents: bytemuck::cast_slice(vertices),
      usage: BufferUsages::VERTEX,
    });
    VertexBuffer {
      buffer,
      len: vertices.len() as u32,
    }
  }

  ///Use only a portion of this Buffer for a given operation. Choosing a range with no end will use the rest of the buffer. Using a totally unbounded range will use the entire buffer.
  pub fn slice<S>(&self, bounds: S) -> BufferSlice
  where
    S: RangeBounds<BufferAddress>,
  {
    self.buffer.slice(bounds)
  }
}

pub struct IndexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl IndexBuffer {
  pub fn new(device: &Device, indices: &Vec<u32>) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("index buffer"),
      contents: bytemuck::cast_slice(indices),
      usage: BufferUsages::INDEX,
    });
    IndexBuffer {
      buffer,
      len: indices.len() as u32,
    }
  }

  ///Use only a portion of this Buffer for a given operation. Choosing a range with no end will use the rest of the buffer. Using a totally unbounded range will use the entire buffer.
  pub fn slice<S>(&self, bounds: S) -> BufferSlice
  where
    S: RangeBounds<BufferAddress>,
  {
    self.buffer.slice(bounds)
  }
}

pub struct InstanceBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl InstanceBuffer {
  pub fn new(device: &Device, instances: &Vec<InstanceRaw>) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("instance buffer"),
      contents: bytemuck::cast_slice(&instances),
      usage: BufferUsages::VERTEX,
    });

    InstanceBuffer {
      buffer,
      len: instances.len() as u32,
    }
  }

  ///Use only a portion of this Buffer for a given operation. Choosing a range with no end will use the rest of the buffer. Using a totally unbounded range will use the entire buffer.
  pub fn slice<S>(&self, bounds: S) -> BufferSlice
  where
    S: RangeBounds<BufferAddress>,
  {
    self.buffer.slice(bounds)
  }
}
