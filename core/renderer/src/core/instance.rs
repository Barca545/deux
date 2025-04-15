use bytemuck::{Pod, Zeroable};
use math::{FlatMat4, Mat4, Vec3};
use std::{mem, ops::Range, vec};
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable,)]
/// Struct containing position information needed for rendering an [instance](https://learnopengl.com/Advanced-OpenGL/Instancing) of a
/// [`Mesh`](crate::scene::mesh::Mesh).
pub struct Instance {
  /// Position of the `Mesh` in space.
  position: FlatMat4,
}

// TODO: Instead of vertex maybe make the trait VertexUniformData or something?

impl Instance {
  // TODO: This is preferable to having the Instance implement Vertex but feels
  // clumsy. Maybe I should just rename it like uniform data or something

  /// [`VertexBufferLayout`] for an [`Instance`]. Needed for converting an
  /// `Instance` into a value which can placed into a
  /// [`VertexBuffer`](crate::core::buffer::VertexBuffer).
  pub const BUFFER_LAYOUT: VertexBufferLayout<'static,> = VertexBufferLayout {
    array_stride: mem::size_of::<Self,>() as BufferAddress,
    step_mode: VertexStepMode::Instance,
    attributes: &vertex_attr_array![5 => Float32x4, 6 => Float32x4, 7 =>
    Float32x4, 8 => Float32x4,],
  };

  /// Create a new [`Instance`].
  pub fn new(position: Vec3,) -> Self {
    let position = Mat4::new_translation(&position,).into();
    Instance { position, }
  }
}

// TODO: Confirm this is the use of this struct
/// Collection of a group of [`Instance`]s of a
/// [`Mesh`](crate::scene::mesh::Mesh). Used for collecting all instances of a
/// `Mesh` in the scene.
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

  /// Returns the number of [elements](crate::core::instance::Instance) in
  /// [`Instances`], also referred to as its 'length'.
  pub fn len(&self,) -> usize {
    self.0.len()
  }

  pub fn push(&mut self, instance: Instance,) {
    self.0.push(instance,);
  }
}

impl IntoIterator for Instances {
  type Item = Instance;

  type IntoIter = vec::IntoIter<Instance,>;

  fn into_iter(self,) -> Self::IntoIter {
    self.0.into_iter()
  }
}
