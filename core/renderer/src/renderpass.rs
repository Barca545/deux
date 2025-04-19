use crate::{
  core::{buffer::InstanceBuffer, gpu_context::GpuContext, texture::Texture},
  scene::{material::Material, mesh::Mesh, model::ModelId},
  utils::{
    resources::RenderResources,
    vertex_state::{INSTANCE_BUFFER_SLOT, VERTEX_BUFFER_SLOT},
  },
};
use std::ops::Range;
use wgpu::{
  BindGroup, Color, CommandEncoder, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
  RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, SurfaceTexture, TextureView,
  TextureViewDescriptor,
};

pub struct RenderPass<'encoder,> {
  /// wgpu [`RenderPass`](wgpu::RenderPass). Stores render commands and draws
  /// them to a render target. [`wgpu::RenderPass::end`]
  renderpass: wgpu::RenderPass<'encoder,>,
  /// Reference to the [`Renderer`](super::renderer::Renderer)'s
  /// [`RenderResources`]. Provides information from the `Renderer` needed for
  /// drawing.
  resources: &'encoder RenderResources,
}

impl<'pass,> RenderPass<'pass,> {
  /// Create a new [`RenderPass`].
  pub fn new<'encoder,>(
    ctx: &GpuContext,
    encoder: &'encoder mut CommandEncoder,
    resources: &'encoder RenderResources,
    output: &SurfaceTexture,
    label: &'encoder str,
  ) -> RenderPass<'encoder,> {
    let view = RenderPass::create_texture_view(output,);
    let depth_view = Texture::create_depth_texture(ctx,).view;

    let desc = RenderPassDescriptor {
      label: Some(label,),
      color_attachments: &[Some(RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: Operations {
          load: LoadOp::Clear(Color::BLACK,),
          store: StoreOp::Store,
        },
      },),],
      // Attach the depth stencil
      depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
        view: &depth_view,
        depth_ops: Some(Operations {
          load: LoadOp::Clear(1.0,),
          store: StoreOp::Store,
        },),
        stencil_ops: None,
      },),
      timestamp_writes: None,
      occlusion_query_set: None,
    };

    let pass = encoder.begin_render_pass(&desc,).forget_lifetime();
    RenderPass {
      renderpass: pass,
      resources,
    }
  }

  // /// Create a [`CommandEncoder`] for the [`RenderPass`] to use.
  // fn create_command_encoder(ctx: &GpuContext,) -> CommandEncoder {
  //   // Create a command encoder for draw commands
  //   ctx
  //     .device
  //     .create_command_encoder(&CommandEncoderDescriptor {
  //       label: Some("Render Encoder",),
  //     },)
  // }

  // TODO: document. I am pretty sure this is the surface the draw commands
  // actually target
  fn create_texture_view(output: &SurfaceTexture,) -> TextureView {
    // Create a textureview to control how the code renders to the texture
    output
      .texture
      .create_view(&TextureViewDescriptor::default(),)
  }

  /// Set the ID of the active [`RenderPipeline`](wgpu::RenderPipeline).
  /// Subsequent operations will use this `RenderPipeline`.
  pub fn set_pipeline(&mut self, pipeline_id: usize,) {
    self
      .renderpass
      .set_pipeline(self.resources.get_pipeline(pipeline_id,),);
  }

  /// Set the ID of the active [`BindGroup`] in the
  /// active [`RenderPipeline`](wgpu::RenderPipeline).
  pub fn set_bind_group(&mut self, index: u32, bindgroup: &'pass BindGroup,) {
    // Currently I'm not dealing with bindgroup offsets. This is always easy enough
    // to change later.
    self.renderpass.set_bind_group(index, bindgroup, &[],);
  }

  /// Assign a [`InstanceBuffer`](crate::core::buffer::InstanceBuffer) to a
  /// slot in the currently bound [`RenderPipeline`](wgpu::RenderPipeline).
  /// Specifically, `slot` refers to the index of the matching descriptor in
  /// [`VertexState::buffers`](wgpu::VertexState::buffers).
  pub fn set_instance_buffer(&mut self, buffer: &'pass InstanceBuffer,) {
    // There's nothing to buffer if there are no instances.
    if buffer.len() > 0 {
      self
        .renderpass
        .set_vertex_buffer(INSTANCE_BUFFER_SLOT, buffer.slice(..,),)
    }
  }

  // TODO: Document the draw methods
  // TODO: look at learn wgpu to find out where this *should* be used
  #[warn(missing_docs)]
  pub fn draw_mesh(&mut self, mesh: &'pass Mesh, material: &'pass Material,) {
    self.draw_mesh_instanced(mesh, material, 0..1,)
  }

  #[warn(missing_docs)]
  pub fn draw_mesh_instanced(
    &mut self,
    mesh: &'pass Mesh,
    material: &Material,
    instances: Range<u32,>,
  ) {
    // Set the pipeline for the materials
    // TODO: I am unsure if here should have something to do with altering the
    // pipeline
    self
      .renderpass
      .set_vertex_buffer(VERTEX_BUFFER_SLOT, mesh.vertex_buffer.slice(..,),);

    // Buffer the indices
    self
      .renderpass
      .set_index_buffer(mesh.index_buffer.slice(..,), IndexFormat::Uint32,);

    // Set the Texture bind group
    self
      .renderpass
      .set_bind_group(1, self.resources.get_bindgroup(material.bindgroup(),), &[],);

    // Draw the mesh
    self
      .renderpass
      .draw_indexed(mesh.indices_range(), 0, instances,)
  }

  pub fn draw_model_instanced(&mut self, model: ModelId, instances: &Range<u32,>,) {
    // Iterate over the model's submeshes and render each one.
    for mesh in &self.resources.models.get(&model,).meshes {
      let material = self.resources.get_material(mesh.material,);
      self.draw_mesh_instanced(mesh, material, instances.clone(),);
    }
  }
}
