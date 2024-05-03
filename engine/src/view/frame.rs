use super::MODEL_NUM;
use crate::{
  component_lib::Position,
  view::{camera::Camera, InstanceRaw},
};
use std::{hash::Hash, ops::Range};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
///Alias for the id of a `Model`.
pub struct ModelId(pub usize);

#[derive(Debug, Default)]
///Struct containing all the data to be rendered in a frame.
pub struct Frame {
  pub(crate) pv_mat: [[f32; 4]; 4],
  ///`Vec` containing all a [`Frame`]'s `Model`s' [`InstanceRaw`]s.
  pub(crate) instances: Vec<Vec<InstanceRaw>>,
}

impl Frame {
  ///Create a new [`Frame`].
  pub fn new(camera: &Camera) -> Self {
    let mut instances = Vec::new();
    instances.resize(unsafe { MODEL_NUM }, vec![]);

    Frame {
      pv_mat: camera.pv_mat(),
      instances,
    }
  }

  ///Add a new [`InstanceRaw`] to a `Model` in the [`Frame`].
  pub fn add_instance(&mut self, id: &ModelId, position: &Position) {
    self.instances[id.0].push(InstanceRaw::from(position));
  }

  ///Return the [`Range`] containing the [`Frame`]'s models.
  pub fn models(&self) -> Range<usize> {
    0..self.instances.len()
  }
}
