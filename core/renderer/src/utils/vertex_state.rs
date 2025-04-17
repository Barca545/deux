use crate::{
  core::vertex::{ModelVertex, VertexBufferData},
  Instance,
};
use wgpu::VertexBufferLayout;

/// The slot the [`VertexBuffer`](crate::core::buffer::VertexBuffer) is placed
/// into [`VertexState::buffers`](wgpu::VertexState::buffers) when creating an
/// Opaque [`RenderPipeline`](wgpu::RenderPipeline).
pub const VERTEX_BUFFER_SLOT: u32 = 0;

/// The slot the [`InstanceBuffer`](crate::core::buffer::InstanceBuffer) is
/// placed into [`VertexState::buffers`](wgpu::VertexState::buffers) when
/// creating an Opaque [`RenderPipeline`](wgpu::RenderPipeline).
pub const INSTANCE_BUFFER_SLOT: u32 = 1;

/// The [`VertexState::buffers`](wgpu::VertexState::buffers) for the Opaque
/// [`RenderPipeline`](wgpu::RenderPipeline)
pub const VERTEX_STATE_BUFFERS: [VertexBufferLayout; 2] =
  [ModelVertex::BUFFER_LAYOUT, Instance::BUFFER_LAYOUT,];

#[cfg(test)]
mod tests {
  use crate::{core::vertex::VertexBufferData, Instance};
  use std::mem;

  #[test]
  fn bufferstate() {
    assert_eq!(
      Instance::BUFFER_LAYOUT.step_mode,
      wgpu::VertexStepMode::Instance
    );
    dbg!(mem::size_of::<Instance,>());
    assert_eq!(Instance::BUFFER_LAYOUT.array_stride, 64);
    dbg!(Instance::BUFFER_LAYOUT);
  }
}
