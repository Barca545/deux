use super::vertex::VertexBufferData;
use bytemuck::{Pod, Zeroable};
use math::{FlatMat4, Mat4, Vec3};
use std::mem;
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Pod, Zeroable,)]
/// Struct containing position information needed for rendering an [instance](https://learnopengl.com/Advanced-OpenGL/Instancing) of a
/// [`Mesh`](crate::scene::mesh::Mesh).
pub struct Instance {
  /// Position of the `Mesh` in space.
  position: FlatMat4,
}

impl VertexBufferData for Instance {
  /// [`VertexBufferLayout`] for an [`Instance`]. Needed for converting an
  /// `Instance` into a value which can placed into a
  /// [`VertexBuffer`](crate::core::buffer::VertexBuffer).
  const BUFFER_LAYOUT: VertexBufferLayout<'static,> = VertexBufferLayout {
    array_stride: mem::size_of::<Self,>() as BufferAddress,
    step_mode: VertexStepMode::Instance,
    attributes: &vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 =>
    Float32x4, 8 => Float32x4,],
  };
}

impl Instance {
  /// Create a new [`Instance`].
  pub fn new(position: Vec3,) -> Self {
    let position = Mat4::new_translation(&position,).into();
    Instance { position, }
  }
}
