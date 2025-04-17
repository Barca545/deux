use super::model::ModelId;
use crate::Instance;
use std::{collections::HashMap, ops::Range};

pub struct SceneBuilder {
  inner: HashMap<ModelId, Vec<Instance,>,>,
}

impl SceneBuilder {
  /// Create a new `Scene`.
  pub fn new() -> Self {
    SceneBuilder {
      inner: HashMap::new(),
    }
  }

  /// Add a new [`Instance`](crate::core::instance::Instance) to the `Scene`.
  pub fn insert(&mut self, model: &ModelId, instance: Instance,) {
    match self.inner.get_mut(model,) {
      Some(instances,) => instances.push(instance,),
      None => {
        self.inner.insert(*model, vec![instance],);
      }
    }
  }

  pub fn build(self,) -> Scene {
    let mut drawcalls = Vec::new();
    let mut scene_instances = Vec::new();

    // Iterate over each model in the scene and produce a DrawCall for its instances
    for (model, instances,) in &self.inner {
      // Append the instances to the scene's instances
      scene_instances.extend(instances,);

      let start = scene_instances.len() as u32 - 1;

      drawcalls.push(DrawCall {
        model: *model,
        slice: Range {
          start,
          end: start + instances.len() as u32,
        },
      },);
    }

    Scene {
      instances: scene_instances,
      drawcalls: drawcalls,
    }
  }
}

/// A record of each [`Model`](crate::core::model::Model) and all
/// [`Instance`](crate::core::instance::Instance)s of it in a frame. Consumed by
/// the [`Renderer`](crate::renderer::Renderer) during rendering.
pub struct Scene {
  /// All [`Instance`]s of all [`ModelId`]s in the `Scene`.
  instances: Vec<Instance,>,
  /// All [`DrawCall`]s in the `Scene`.
  drawcalls: Vec<DrawCall,>,
}

impl Scene {
  /// Return a refernce to the `Scene`'s [`DrawCall`](crate::scene::DrawCall)s.
  pub(crate) fn drawcalls(&self,) -> &Vec<DrawCall,> {
    &self.drawcalls
  }

  /// Return a refernce to the `Scene`'s
  /// [`Instance`](crate::core::instance::Instance)s
  pub(crate) fn instances(&self,) -> &Vec<Instance,> {
    &self.instances
  }
}

pub struct DrawCall {
  /// Handle to the [`Model`](crate::scene::model::Model) the `DrawCall` is
  /// rendering.
  pub model: ModelId,
  /// The range of the [`InstanceBuffer`](crate::core::buffer::InstanceBuffer)
  /// the `DrawCall`'s [`Instance`](crate::core::instance::Instance)s of the
  /// occupy.
  pub slice: Range<u32,>,
}
