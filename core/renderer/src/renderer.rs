use super::{
  core::{buffer::InstanceBuffer, gpu_context::GpuContext},
  renderpass::RenderPass,
  scene::{camera::Camera, model::ModelId},
  utils::load::load_model,
};
use crate::{
  core::texture::Texture,
  scene::Scene,
  utils::{
    load::load_shader,
    resources::{CameraResources, RenderResources},
    vertex_state::VERTEX_STATE_BUFFERS,
  },
};
use eyre::Result;
use math::FlatMat4;
use std::iter;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
  BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType, BufferUsages, ColorTargetState,
  ColorWrites, CommandEncoderDescriptor, CompareFunction, DepthBiasState, DepthStencilState, Face,
  FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
  PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
  SamplerBindingType, ShaderStages, StencilState, TextureSampleType, TextureViewDimension,
  VertexState,
};
use windowing::{sdl2_utils::PhysicalSize, windowing::Window};

// Refactor:
// - Does the Adapter/Device need to be released at the end of the program?
// - Swap the frame buffer?
// - Do static meshes need a different pipeline? -- not sure if they have a
//   different shader
// - Might make sense to move the logic for creating bindgroups into it's own
//   module

pub struct Renderer {
  pub(crate) ctx: GpuContext,
  /// Cached Resources needed for rendering.
  pub(crate) resources: RenderResources,
}

impl Renderer {
  /// Id of the Opaque [`RenderPipeline`](wgpu::RenderPipeline).
  /// Use to set the opaque `RenderPipeline` during a [`RenderPass`].
  const OPAQUE_PIPELINE: usize = 0;

  /// The [`BindGroupLayoutDescriptor`] for the [`Camera`]'s data.
  /// Describes how the vertex shader will process `Camera` data.
  const CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR: &BindGroupLayoutDescriptor<'static,> =
    &BindGroupLayoutDescriptor {
      label: Some("Camera Bindgroup Layout",),
      entries: &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
          ty: BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      },],
    };

  /// The [`BindGroupLayoutDescriptor`] for
  /// [`Texture`](crate::core::texture::Texture) data. Describes how the
  /// fragement shader will process `Texture` data.
  const TEXTURE_BINDGROUP_LAYOUT_DESCRIPTOR: &BindGroupLayoutDescriptor<'static,> =
    &BindGroupLayoutDescriptor {
      label: Some("Texture Bindgroup layout",),
      entries: &[
        BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Texture {
            multisampled: false,
            view_dimension: TextureViewDimension::D2,
            sample_type: TextureSampleType::Float { filterable: true, },
          },
          count: None,
        },
        BindGroupLayoutEntry {
          binding: 1,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Sampler(SamplerBindingType::Filtering,),
          count: None,
        },
      ],
    };

  /// Create a new `Renderer` struct.
  pub fn new(window: &Window,) -> Self {
    let ctx = pollster::block_on(GpuContext::new(window,),);

    // Create a buffer to hold camera data passed to the renderer
    let camera_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
      label: Some("Camera Buffer",),
      // Pad the buffer with dummy "camera"
      // This will be overwritten during the first render
      contents: bytemuck::cast_slice(&FlatMat4::default(),),
      // The camera corresponds to a uniform
      // The camera needs to allow data to be copied to it so the
      // camera data can be updated each frame.
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    },);

    // TODO: Figure out why this needs to be recreated if new instances are created per https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/#the-instance-buffer

    // Create the camera_bindgroup
    let camera_bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
      label: Some("Camera Bindgroup",),
      layout: &ctx
        .device
        .create_bind_group_layout(Self::CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR,),
      entries: &[BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      },],
    },);

    Renderer {
      // Create the camera resources and use it to create the render resources
      resources: RenderResources::new(CameraResources {
        bindgroup: camera_bind_group,
        buffer: camera_buffer,
      },),
      ctx,
    }
  }

  // TODO: Right now this is really only for albedo/base color textures. Will
  // probably need updating when new ones are added
  /// Create a new [`Bindgroup`](wgpu::BindGroup) to hold a
  /// [`Texture`](crate::core::texture::Texture).
  pub fn create_texture_bindgroup(&self, texture: &Texture,) -> BindGroup {
    self.ctx.device.create_bind_group(&BindGroupDescriptor {
      // TODO: Need a better debug name
      label: Some("Texture Bindgroup",),
      layout: &self.create_texture_bindgroup_layout(),
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&texture.view,),
        },
        BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&texture.sampler,),
        },
      ],
    },)
  }

  /// Create a new [`BindGroupLayout`] for a
  /// [`Texture`](crate::core::texture::Texture) [`Bindgroup`](wgpu::BindGroup).
  fn create_texture_bindgroup_layout(&self,) -> BindGroupLayout {
    self
      .ctx
      .device
      .create_bind_group_layout(Self::TEXTURE_BINDGROUP_LAYOUT_DESCRIPTOR,)
  }

  /// Add the default Opaque [`RenderPipeline`](wgpu::RenderPipeline) to the
  /// [`Renderer`](crate::renderer::Renderer).
  ///
  /// # Warning
  /// Must be called before rendering occurs. Otherwise rendering will fail.
  pub fn add_opaque_pipeline(&mut self, shader_name: &str,) {
    let pipeline = self.create_opaque_pipeline(shader_name,);
    self
      .resources
      .insert_pipeline(Self::OPAQUE_PIPELINE, pipeline,);
  }

  // TODO: For now this just create the one opaque render pipeline. Might
  // eventually need to be made more general
  // TODO: For now this will have hard coded shaders but as things start to get
  // unique shades this will need to change
  fn create_opaque_pipeline(&mut self, shader_name: &str,) -> RenderPipeline {
    // Load the shaders for the
    let shaders = load_shader(&self.ctx, shader_name,).unwrap();

    // Create the pipeline's layout
    let pipeline_layout = self
      .ctx
      .device
      .create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Opaque RenderPipelineLayout",),
        // TODO: A real opaque material will have a few textures
        // Declare the bind_group_layouts the pipeline will use (1 camera and 1 texure)
        bind_group_layouts: &[
          // Create the Camera BindGroup Layout
          &self
            .ctx
            .device
            .create_bind_group_layout(Self::CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR,),
          // Create the Base Color Texture BindGroup Layout
          &self
            .ctx
            .device
            .create_bind_group_layout(Self::TEXTURE_BINDGROUP_LAYOUT_DESCRIPTOR,),
        ],
        push_constant_ranges: &[],
      },);

    self
      .ctx
      .device
      .create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Opaque RenderPipeline",),
        layout: Some(&pipeline_layout,),
        vertex: VertexState {
          module: &shaders,
          entry_point: Some("vs_main",),
          buffers: &VERTEX_STATE_BUFFERS,
          compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(FragmentState {
          module: &shaders,
          entry_point: Some("fs_main",),
          targets: &[Some(ColorTargetState {
            format: self.ctx.config.format,
            blend: Some(BlendState::REPLACE,),
            write_mask: ColorWrites::ALL,
          },),],
          compilation_options: PipelineCompilationOptions::default(),
        },),
        primitive: PrimitiveState {
          topology: PrimitiveTopology::TriangleList,
          strip_index_format: None,
          // Cull triangles whose verts are not arranged counterclockwise
          front_face: FrontFace::Ccw,
          cull_mode: Some(Face::Back,),
          // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
          polygon_mode: PolygonMode::Fill,
          // Requires Features::DEPTH_CLIP_CONTROL
          unclipped_depth: false,
          // Requires Features::CONSERVATIVE_RASTERIZATION
          conservative: false,
        },
        depth_stencil: Some(DepthStencilState {
          format: Texture::DEPTH_FORMAT,
          depth_write_enabled: true,
          depth_compare: CompareFunction::Less,
          stencil: StencilState::default(),
          bias: DepthBiasState::default(),
        },),
        multisample: MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
      },)
  }

  // TODO: Can depth textures be reused for different pipelines?
  /// Update the size of the render target.
  pub fn resize(&mut self, new_size: PhysicalSize<u32,>,) {
    if new_size.width > 0 && new_size.height > 0 {
      self.ctx.config.width = new_size.width;
      self.ctx.config.height = new_size.height;
      self
        .ctx
        .surface
        .configure(&self.ctx.device, &self.ctx.config,);
    }
  }

  /// Convert a [`Scene`] into data which can be renderered and create a
  /// [`CommandEncoder`](wgpu::CommandEncoder) to pass it to the
  /// [`CommandQueue`](wgpu::Queue).
  pub fn render(&mut self, camera: &Camera, scene: Scene,) -> Result<(),> {
    // Update the camera buffer
    self.ctx.queue.write_buffer(
      &self.resources.camera.buffer,
      0,
      bytemuck::cast_slice(&camera.pv_mat(),),
    );

    // Create a command encoder for draw commands
    let mut encoder = self
      .ctx
      .device
      .create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder",),
      },);

    // Get a texture to render to from the surface
    let output = self.ctx.surface.get_current_texture().unwrap();

    // Set the instance buffer
    let instance_buffer = InstanceBuffer::new(&self.ctx, &scene.instances(),);
    {
      // Create a renderpass
      let mut renderpass = RenderPass::new(
        &self.ctx,
        &mut encoder,
        &self.resources,
        &output,
        "Opaque Pass",
      );

      // TODO: I am positive rebinding the pipeline and bindgroup each time is bad
      // Opaque draws: bind the opaque pipeline
      renderpass.set_pipeline(Self::OPAQUE_PIPELINE,);

      // Pass the index buffer to the renderpass
      renderpass.set_instance_buffer(&instance_buffer,);

      // Set the camera bindgroup
      renderpass.set_bind_group(0, &self.resources.camera.bindgroup,);

      // Here do the actual drawing
      for drawcall in scene.drawcalls() {
        // Draw
        renderpass.draw_model_instanced(drawcall.model, &drawcall.slice,);
      }
    }

    self.ctx.queue.submit(iter::once(encoder.finish(),),);
    output.present();

    Ok((),)
  }

  /// Adds a [`Model`](crate::scene::model::Model) to the [`Renderer`] and
  /// returns its [`ModelId`].
  pub fn add_model(&mut self, name: &str,) -> ModelId {
    let model = load_model(self, name,);
    self.resources.models.alloc(model,)
  }
}
