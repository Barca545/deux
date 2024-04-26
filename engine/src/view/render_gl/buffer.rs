use super::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferSlice, BufferUsages, Device};

// Refactor:
// -Revisit the labels

pub struct VertexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl VertexBuffer {
  pub fn new(device: &Device, vertices: Vec<impl Vertex>) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("vertex buffer"),
      contents: bytemuck::cast_slice(&vertices),
      usage: BufferUsages::VERTEX,
    });
    VertexBuffer {
      buffer,
      len: vertices.len() as u32,
    }
  }

  pub fn slice(&self) -> BufferSlice {
    self.buffer.slice(..)
  }
}

pub struct IndexBuffer {
  buffer: Buffer,
  pub len: u32,
}

impl IndexBuffer {
  pub fn new(device: &Device, vertices: Vec<u16>) -> Self {
    let buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("index buffer"),
      contents: bytemuck::cast_slice(&vertices),
      usage: BufferUsages::INDEX,
    });
    IndexBuffer {
      buffer,
      len: vertices.len() as u32,
    }
  }

  pub fn slice(&self) -> BufferSlice {
    self.buffer.slice(..)
  }
}
