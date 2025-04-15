use crate::{core::instance::Instances, scene::model::ModelId};
use std::ops::Range;

/// Instruction for the [`Renderer`](crate::renderer::Renderer) to draw all
/// [`Instances`](crate::core::instance::Instance) of one
/// [`Model`](crate::scene::model::Model).
pub struct DrawCall {
  /// Handle to the [`Model`](crate::scene::model::Model) the `DrawCall` is
  /// rendering.
  pub model: ModelId,
  /// The [`Instances`](crate::core::instance::Instances) of the
  /// [`Model`](crate::scene::model::Model) in the scene.
  pub instances: Instances,
}

pub struct InternalDrawCall {
  /// Handle to the [`Model`](crate::scene::model::Model) the `DrawCall` is
  /// rendering.
  pub model: ModelId,
  /// The slice the [`Instances`](crate::core::instance::Instances) of the
  /// `InternalDrawCall` occupy in the
  /// [`InstanceBuffer`](crate::core::buffer::InstanceBuffer).
  pub instances: Range<u32,>,
}
