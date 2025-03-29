use crate::{
  data_lib::Position,
  renderer::scene::{
    camera::Camera,
    instance::{Instance, Instances},
    model::ModelId,
  },
};
use std::ops::Range;

// TODO: Figure out if this is what would be the commandbuffer file in
// yakui_wgpu

// TODO: Figure out a better way of getting the number instnaces in a frame than
// using that model number static which I need to ditch
const TODO_DELETE_ME:usize = 0;

#[derive(Debug, Default,)]
/// Struct containing all the data to be rendered in a `Frame`.
pub struct Frame {
  pub(crate) pv_mat:[[f32; 4]; 4],
  /// `Vec` containing the [`Instances`] of the [`Frame`]'s
  /// [`Model`](crate::renderer::scene::model)s.
  pub(crate) instances:Vec<Instances,>,
}

impl Frame {
  /// Create a new [`Frame`].
  pub fn new(camera:&Camera,) -> Self {
    Frame {
      pv_mat:camera.pv_mat(),
      // Create an array of vectors to hold the instance
      instances:vec![Instances::new(); TODO_DELETE_ME],
    }
  }

  /// Record a new [`Instance`] for a
  /// [`Model`](crate::renderer::scene::model) in the [`Frame`].
  pub fn record_instance(&mut self, id:&ModelId, position:&Position,) {
    self.instances[id.0].0.push(Instance::from(position,),);
  }

  /// Return the [`Range`] containing the [`Frame`]'s
  /// [`Model`](crate::renderer::scene::model)s.
  pub fn range_of_models(&self,) -> Range<usize,> {
    0..self.instances.len()
  }
}
