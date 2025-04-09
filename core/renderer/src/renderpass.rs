// pub struct OpaquePass {
//   pipeline:RenderPipeline,
//   // camera_buffer:
//   // camera_bindgroup
// }

// pub struct OpaquePass<'p,>(RenderPass<'p,>,);

// impl<'p,> OpaquePass<'p,> {
//   /// Create a new `OpaquePass`.
//   pub fn new(/*Probably take the camera + screen dimensions */) -> Self {
//     todo!()
//   }

//   fn opaque_pass_descriptor(/*Create a new opaque pass descriptor from the
// screen dimensions*/   // TODO: Unsure this is actually static
//   // Needs a lot of other params too...
//   ) -> RenderPassDescriptor<'static, 'static,> {
//     // Only update if the dimensions have changed
//     todo!()
//   }
// }

// pub trait RenderPass {
//   fn draw(&self,) {}
// }

use crate::{
  core::{frame::Frame, gpu_context::GpuContext},
  resources::RenderResources,
  scene::{material::Material, mesh::Mesh, model::Model},
};
use std::ops::Range;
use wgpu::{IndexFormat, RenderPassDescriptor};

pub struct RenderPassBuilder {
  label:String,
}

impl RenderPassBuilder {
  pub fn new(&self, ctx:GpuContext,) -> RenderPass {
    todo!()
  }
}

struct RenderPass<'pass,> {
  renderpass:wgpu::RenderPass<'pass,>,
  resources:&'pass RenderResources,
  frame:Frame,
}

impl<'pass,> RenderPass<'pass,> {
  // TODO: See if I need to include any other information on renderpasses like
  // Lucien does in yakui
  fn draw_mesh(&mut self, mesh:&'pass Mesh, material:&'pass Material,) {
    self.draw_mesh_instanced(mesh, material, 0..1,)
  }

  fn draw_mesh_instanced(&mut self, mesh:&'pass Mesh, material:&Material, instances:Range<u32,>,) {
    // Buffer the vertices
    self
      .renderpass
      .set_vertex_buffer(0, mesh.vertex_buffer.slice(..,),);

    // Buffer the indices
    self
      .renderpass
      .set_index_buffer(mesh.index_buffer.slice(..,), IndexFormat::Uint32,);

    // Set the Texture bind group
    self
      .renderpass
      .set_bind_group(0, self.resources.get_bindgroup(material.bindgroup(),), &[],);

    // Draw the mesh
    self
      .renderpass
      .draw_indexed(mesh.indices_range(), 0, instances,)
  }

  fn draw_model_instanced(&mut self, model:&'pass Model, instances:Range<u32,>,) {
    // Iterate over the model's submeshes and render each one.
    for mesh in &model.meshes {
      let material = self.resources.get_material(mesh.material,);
      self.draw_mesh_instanced(mesh, material, instances.clone(),);
    }
  }
}
