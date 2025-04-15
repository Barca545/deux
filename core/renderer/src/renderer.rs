use super::{
  core::{buffer::InstanceBuffer, gpu_context::GpuContext},
  renderpass::RenderPass,
  scene::{camera::Camera, model::ModelId},
  utils::load::load_model,
};
use crate::{
  core::{
    texture::Texture,
    vertex::{ModelVertex, Vertex},
  },
  drawcall::{InternalDrawCall, Scene},
  utils::{
    load::load_shader,
    resources::{CameraResources, RenderResources},
  },
  Instance,
};
use eyre::Result;
use math::FlatMat4;
use std::{iter::once, num::NonZero};
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
  BindGroupLayoutEntry, BindingType, BlendState, BufferBinding, BufferBindingType, BufferUsages,
  ColorTargetState, ColorWrites, CommandEncoderDescriptor, CompareFunction, DepthBiasState,
  DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions,
  PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
  RenderPipelineDescriptor, SamplerBindingType, ShaderStages, StencilState, TextureFormat,
  TextureSampleType, TextureViewDimension, VertexBufferLayout, VertexState,
};
use windowing::{sdl2_utils::PhysicalSize, windowing::Window};

// Refactor:
// - Does the Adapter/Device need to be released at the end of the program?
// - Swap the frame buffer?
// - Do static meshes need a different pipeline? -- not sure if they have a
//   different shader

pub struct Renderer {
  pub(crate) ctx: GpuContext,
  /// Cached Resources needed for rendering.
  pub(crate) resources: RenderResources,
}

impl Renderer {
  /// Id of the Opaque [`RenderPipeline`](wgpu::RenderPipeline).
  /// Use to set the opaque `RenderPipeline` during a [`RenderPass`].
  const OPAQUE_PIPELINE: usize = 0;

  // Lasts the whole program so static
  /// The [`BindGroupLayoutDescriptor`] for the [`Camera`]'s data.
  /// Describes how the vertex shader will process `Camera` data.
  // TODO: The camera bindgroup will always be the same.
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

    Renderer {
      // Create the camera resources and use it to create the render resources
      resources: RenderResources::new(CameraResources {
        bindgroup: ctx.device.create_bind_group(&BindGroupDescriptor {
          label: Some("Main Camera Bindgroup",),
          layout: &ctx
            .device
            .create_bind_group_layout(Self::CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR,),
          entries: &[BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
          },],
        },),
        buffer: camera_buffer,
      },),
      ctx,
    }
  }

  /// Create a new [`Bindgroup`](wgpu::BindGroup) to hold a
  /// [`Texture`](crate::core::texture::Texture).
  pub fn create_texture_bindgroup(&self,) -> BindGroup {
    self.ctx.device.create_bind_group(&BindGroupDescriptor {
      // TODO: Need a better debug name
      label: Some("Texture Bindgroup",),
      layout: &self.create_texture_bindgroup_layout(),
      entries: &[],
    },)
  }

  /// Create a new [`BindGroupLayout`] for a
  /// [`Texture`](crate::core::texture::Texture) [`Bindgroup`](wgpu::BindGroup).
  pub fn create_texture_bindgroup_layout(&self,) -> BindGroupLayout {
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
          buffers: &[ModelVertex::BUFFER_LAYOUT, Instance::BUFFER_LAYOUT,],
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
          // TODO: How do I know this is the correct color format
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

  // /// Add a new [`RenderPipeline`](wgpu::RenderPipeline) to the `Renderer`.
  // pub fn create_opaque_pipeline(&mut self, name:&str,
  // layouts:&[BindGroupLayout], shader:&str,) {   // Create the render pipeline
  // layout   let pipeline_layout = self
  //     .ctx
  //     .device
  //     .create_pipeline_layout(&PipelineLayoutDescriptor {
  //       // TODO: Arguably these need more descriptive names
  //       label:Some("Render Pipeline Layout",),
  //       bind_group_layouts:&[
  //         &self
  //           .ctx
  //           .device
  //           .
  // create_bind_group_layout(Self::TEXTURE_BINDGROUP_LAYOUT_DESCRIPTOR,),
  //         &&self
  //           .ctx
  //           .device
  //           .create_bind_group_layout(Self::CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR,
  // ),       ],
  //       push_constant_ranges:&[],
  //     },);

  //   // Load and instantiate the pipeline's shader
  //   let model_shader = load_shader(&self.ctx.device, shader,).unwrap();

  //   let pipeline = Self::create_render_pipeline(
  //     &self.ctx.device,
  //     pipeline_layout,
  //     // TODO: How do I know this is the correct color format
  //     TextureFormat::Rgba16Float,
  //     Some(Texture::DEPTH_FORMAT,),
  //     &[ModelVertex::DESCRIPTOR, Instance::DESCRIPTOR,],
  //     model_shader,
  //   );

  //   // TODO: Can depth textures be reused for different pipelines?

  //   // Create the depth texture
  //   let depth_texture = Texture::create_depth_texture(&self.ctx.device,
  // &todo!(),); }

  // TODO: Since I am not giving the renderer ownership of the window I don't
  // think this is needed
  // /// Get the handle of the [`Renderer`]'s [`Window`](sdl2::video::Window).
  // pub fn window(&self,) -> &sdl2Window {
  //   self.canvas.window()
  // }

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

  ///// Prepare the [`CommandEncoder`](wgpu::CommandEncoder) for rendering.
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

    // TODO: If this works make it a render resource
    let mut buffer = InstanceBuffer::new();
    {
      // Prep all the drawcalls

      let mut internal_drawcalls = Vec::new();

      for draw_call in scene.calls {
        // TODO: Hand off populating the instance buffer to the update render function
        // and replace drawcall with what is currently the internal draw call

        // Capture the start of this entry in the instance buffer
        let start = buffer.len();

        // Push the instances into the Instance buffer
        // TODO: I think this is causing an error on the first frame for...reasons

        buffer.push_instances(&self.ctx, &mut encoder, &draw_call.instances,);

        // Capture the end of this entry in the instance buffer
        let end = buffer.len();

        // Prepare an internal drawcall
        internal_drawcalls.push(InternalDrawCall {
          model: draw_call.model,
          instances: start as u32..end as u32,
        },);
      }
      // Here do the actual drawing

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

      // TODO: Something is failing to generate instances I don't think this should
      // ever be 0
      // Pass the index buffer to the renderpass
      renderpass.set_instance_buffer(1, &buffer,);

      for drawcall in internal_drawcalls {
        // Set the texture and camera bindgroups
        renderpass.set_bind_group(1, &self.resources.camera.bindgroup,);

        // Draw
        renderpass.draw_model_instanced(drawcall.model, drawcall.instances,);
      }
    }

    self.ctx.queue.submit(once(encoder.finish(),),);
    output.present();
    // device.poll(wgpu::Maintain::Wait);

    Ok((),)
  }

  /// Adds a [`Model`] to the [`Renderer`] and returns its [`ModelId`]
  pub fn add_model(&mut self, name: &str,) -> ModelId {
    // TODO: Load model can take a graphics contex
    let model = load_model(self, name,);

    // Initially this returned a model which I then stored but the model data is all
    // buffered All I need is a handle to it

    self.resources.models.alloc(model,)
  }

  // /// Create a new [`RenderPipeline`].
  // fn create_render_pipeline(
  //   device:&Device,
  //   pipeline_layout:PipelineLayout,
  //   color_format:TextureFormat,
  //   depth_format:Option<TextureFormat,>,
  //   vertex_layouts:&[VertexBufferLayout],
  //   shader:ShaderModule,
  // ) -> RenderPipeline {
  //   //Create the render pipeline
  //   device.create_render_pipeline(&RenderPipelineDescriptor {
  //     label:Some("render pipeline",),
  //     layout:Some(&pipeline_layout,),
  //     vertex:VertexState {
  //       module:&shader,
  //       entry_point:"vs_main",
  //       buffers:vertex_layouts,
  //     },
  //     fragment:Some(FragmentState {
  //       module:&shader,
  //       entry_point:"fs_main",
  //       targets:&[Some(ColorTargetState {
  //         format:color_format,
  //         blend:Some(BlendState::REPLACE,),
  //         write_mask:ColorWrites::ALL,
  //       },),],
  //     },),
  //     primitive:PrimitiveState {
  //       topology:PrimitiveTopology::TriangleList,
  //       strip_index_format:None,
  //       //Cull triangles whose verts are not arranged counter clockwise
  //       front_face:FrontFace::Ccw,
  //       cull_mode:Some(Face::Back,),
  //       //Setting this to anything other than Fill requires
  // Features::NON_FILL_POLYGON_MODE       polygon_mode:PolygonMode::Fill,
  //       //Requires Features::DEPTH_CLIP_CONTROL
  //       unclipped_depth:false,
  //       //Requires Features::CONSERVATIVE_RASTERIZATION
  //       conservative:false,
  //     },
  //     //Instantiate depth testing
  //     depth_stencil:depth_format.map(|format| DepthStencilState {
  //       format,
  //       depth_write_enabled:true,
  //       depth_compare:CompareFunction::Less,
  //       stencil:StencilState::default(),
  //       bias:DepthBiasState::default(),
  //     },),
  //     multisample:MultisampleState {
  //       count:1,
  //       mask:!0,
  //       alpha_to_coverage_enabled:false,
  //     },
  //     multiview:None,
  //   },)
  // }
}
