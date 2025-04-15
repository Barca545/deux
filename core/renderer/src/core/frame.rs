// use crate::scene::{
//   instance::{Instance, Instances},
//   model::ModelId,
// };
// use std::{collections::HashMap, ops::Range};

// // TODO: Figure out if this is what would be the commandbuffer file in
// // yakui_wgpu
// // TODO: Is this basically a renderpass?
// // TODO: Frame should be in utils or something

// #[derive(Debug, Default,)]
// /// Struct containing all the data to be rendered in a `Frame`.
// pub struct Frame {
//   /// `Vec` containing the [`Instances`] of the [`Frame`]'s
//   /// [`Model`](crate::scene::model)s.
//   pub(crate) instances:HashMap<ModelId, Instances,>,
// }

// impl Frame {
//   /// Create a new [`Frame`].
//   pub fn new(cap:usize,) -> Self {
//     Frame {
//       // Create an array of vectors to hold the instances
//       instances:HashMap::with_capacity(cap,),
//     }
//   }

//   /// Record a new [`Instance`] for a
//   /// [`Model`](crate::scene::model) in the [`Frame`].
//   pub fn record_instance(&mut self, id:&ModelId, instance:Instance,) {
//     // self.instances[id.0].0.push(Instance::from(position,),);
//     todo!()
//   }

//   /// Return the [`Range`] containing the [`Frame`]'s
//   /// [`Model`](crate::scene::model)s.
//   pub fn range_of_models(&self,) -> Range<usize,> {
//     0..self.instances.len()
//   }
// }
