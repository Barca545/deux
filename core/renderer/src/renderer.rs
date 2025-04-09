use super::{
  core::{
    color::BLACK,
    frame::Frame,
    gpu_context::GpuContext,
    texture::Texture,
    vertex::{ModelVertex, Vertex},
  },
  resources::{CameraResources, RenderResources},
  scene::{
    camera::Camera,
    instance::{Instance, InstanceBuffer},
    model::ModelId,
  },
  utils::load::load_model,
};
use eyre::Result;
use math::FlatMat4;
use nina::world::World;
use std::iter::once;
use time::ServerTime;
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
  BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Color,
  CommandEncoderDescriptor, LoadOp, Operations, PipelineLayoutDescriptor,
  RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
  SamplerBindingType, ShaderStages, StoreOp, TextureFormat, TextureSampleType,
  TextureViewDescriptor, TextureViewDimension,
};
use windowing::sdl2_helpers::Window;

// Refactor:
// - Does the Adapter/Device need to be released at the end of the program?
// - Swap the frame buffer?
// - Create load functions for the shaders
// - Do static meshes need a different pipeline?
// - Create a safer way of generating model numbers which increment. Not a huge
//   priority. This is single threaded so ultimately safe.

// TODO: Test keeping it in the function like this keeps yielding new

pub struct Renderer {
  pub(crate) ctx:GpuContext,
  /// Cached Resources needed for Rendering.
  pub(crate) render_resources:RenderResources,
  // TODO: I think this is fine for now. Eventually I want to make the camera something more
  // functional.
  // /// [`Buffer`] where camera data is stored during rendering.
  // camera_buffer:Buffer,
  // /// Collection of pipelines used for rendering.
  // pipeline:RenderPipeline,
  // // TODO: The camera might be able to store it's own information about the bindgroup and
  // textures? // I had avoided it before when I wanted it to be an ECS thing but now its not a
  // resource there's // no real reason to strongly decouple it from graphics
  // camera_bind_group:BindGroup,
  // camera_buffer:Buffer,
  // // TODO: Unsure where depth texture goes
  // depth_texture:Texture,
  // models:Arena<Model,>,
  // frame:Frame,
}

impl Renderer {
  // Lasts the whole program so static
  /// The [`BindGroupLayoutDescriptor`] for the [`Camera`]'s data.
  /// Describes how the vertex shader will process `Camera` data.
  // TODO: The camera bindgroup will always be the same.
  const CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR:&'static BindGroupLayoutDescriptor<'static,> =
    &BindGroupLayoutDescriptor {
      label:Some("Camera Bindgroup Layout",),
      entries:&[BindGroupLayoutEntry {
        binding:0,
        visibility:ShaderStages::VERTEX,
        ty:BindingType::Buffer {
          ty:BufferBindingType::Uniform,
          has_dynamic_offset:false,
          min_binding_size:None,
        },
        count:None,
      },],
    };

  /// The [`BindGroupLayoutDescriptor`] for [`Texture`] data. Describes how the
  /// fragement shader will process `Texture` data.
  const TEXTURE_BINDGROUP_LAYOUT_DESCRIPTOR:&'static BindGroupLayoutDescriptor<'static,> =
    &BindGroupLayoutDescriptor {
      label:Some("Texture Bindgroup layout",),
      entries:&[
        BindGroupLayoutEntry {
          binding:0,
          visibility:ShaderStages::FRAGMENT,
          ty:BindingType::Texture {
            multisampled:false,
            view_dimension:TextureViewDimension::D2,
            sample_type:TextureSampleType::Float { filterable:true, },
          },
          count:None,
        },
        BindGroupLayoutEntry {
          binding:1,
          visibility:ShaderStages::FRAGMENT,
          ty:BindingType::Sampler(SamplerBindingType::Filtering,),
          count:None,
        },
      ],
    };

  /// Create a new `Renderer` struct.
  fn new(window:&Window,) -> Self {
    let ctx = pollster::block_on(GpuContext::new(window,),);

    // Create a buffer to hold camera data passed to the renderer
    let camera_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
      label:Some("Camera Buffer",),
      // Pad the buffer with dummy "camera"
      // This will be overwritten during the first render
      contents:bytemuck::cast_slice(&FlatMat4::default(),),
      // The camera corresponds to a uniform
      // The camera needs to allow data to be copied to it so the
      // camera data can be updated each frame.
      usage:BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    },);

    Renderer {
      // Create the camera resources and use it to create the render resources
      render_resources:RenderResources::new(CameraResources {
        bindgroup:ctx.device.create_bind_group(&BindGroupDescriptor {
          label:Some("Main Camera Bindgroup",),
          layout:&ctx
            .device
            .create_bind_group_layout(Self::CAMERA_BINDGROUP_LAYOUT_DESCRIPTOR,),
          entries:&[],
        },),
        buffer:camera_buffer,
      },),
      ctx,
    }
  }

  // TODO: Is there any way to do this
  pub fn texture_bind_group_layout(&self,) -> BindGroupLayout {
    todo!()
  }

  // TODO: Do I needa create bindgroup function which takes...something

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

  // pub fn resize(&mut self, new_size:PhysicalSize<u32,>,) {
  //   if new_size.width > 0 && new_size.height > 0 {
  //     self.size = new_size;
  //     self.config.width = new_size.width;
  //     self.config.height = new_size.height;
  //     self.surface.configure(&self.device, &self.config,);

  //     //Update the depth texture
  //     self.depth_texture = Texture::create_depth_texture(&self.device,
  // &self.config,);   }
  // }

  /// Prepare the [`CommandEncoder`](wgpu::CommandEncoder) for rendering.
  pub fn render(&mut self, world:&World,) -> Result<(),> {
    // BEGINNING OF SETUP LOGIC

    // Call it once up here so each object has the same interpolation factor instead
    // of slightly different ones
    let interpolation_factor = world
      .get_resource::<ServerTime>()
      .get_interpolation_factor();

    // Update the camera

    let mut query = world.query();
    let player = &query.with_component::<Controllable>().unwrap().run()[0];
    let player_position = player.get_component::<Position>().unwrap();
    let player_previous_position = player.get_component::<PreviousPosition>().unwrap();
    let player_render_position = calculate_render_position(
      *player_previous_position,
      *player_position,
      interpolation_factor,
    )
    .0;

    let mut camera = world.get_resource_mut::<Camera>();

    camera.offset_camera_relative_to_position(player_render_position,);

    // Render skinned models
    let mut query = world.query();
    let entities = query.with_component::<SkinnedRenderable>().unwrap().run();

    // Create a Frame to draw to
    let mut frame = Frame::new(entities.len(),);

    for entity in entities {
      let model_id = &entity.get_component::<SkinnedRenderable>().unwrap().0;
      let position = entity.get_component::<Position>().unwrap();
      let previous_position = entity.get_component::<PreviousPosition>().unwrap();

      let instance = Instance::new(
        // TODO: I think calculate_render_position could be an associated function on the position
        // struct or something
        calculate_render_position(*previous_position, *position, interpolation_factor,).0,
      );

      // Group the instances for drawing
      frame.record_instance(&model_id, instance,);
    }

    // // Render static models
    // let mut query = world.query();
    // let entities = query.with_component::<StaticRenderable>().unwrap().run();
    // // Add every instance of a model which needs to be rendered to the frame
    // for entity in entities {
    //   let model_id = entity.get_component::<StaticRenderable>().unwrap();
    //   let position = entity.get_component::<Position>().unwrap();
    // }

    // BEGINNING OF RENDER LOGIC

    // TODO: Pipeline comes from the material so I actually need to fetch that first
    // So loop over all the entities and get their materials bind the pipelines

    // Update the camera buffer
    let camera = world.get_resource::<Camera>();
    self.ctx.queue.write_buffer(
      &self.render_resources.camera.buffer,
      0,
      bytemuck::cast_slice(&camera.pv_mat(),),
    );

    // Create a command encoder for draw commands
    let mut encoder = self
      .ctx
      .device
      .create_command_encoder(&CommandEncoderDescriptor {
        label:Some("Render Encoder",),
      },);

    // Get a texture to render to from the surface
    let output = self.ctx.surface.get_current_texture()?;

    // Create a textureview to control how the code renders to the texture
    let view = output
      .texture
      .create_view(&TextureViewDescriptor::default(),);

    // Create a render pass descriptor
    let descriptor = RenderPassDescriptor {
      label:Some("Diffuse Material Pass",),
      color_attachments:&[Some(RenderPassColorAttachment {
        view:&view,
        resolve_target:None,
        ops:Operations {
          load:LoadOp::Clear(BLACK,),
          store:StoreOp::Store,
        },
      },),],
      // Attach the depth stencil
      depth_stencil_attachment:Some(RenderPassDepthStencilAttachment {
        view:&Texture::create_depth_texture(
          &self.ctx.device,
          /* TODO */ todo!("Find a way to grescreen dimensions"),
        )
        .view,
        depth_ops:Some(Operations {
          load:LoadOp::Clear(1.0,),
          store:StoreOp::Store,
        },),
        stencil_ops:None,
      },),
      timestamp_writes:None,
      occlusion_query_set:None,
    };

    {
      let mut render_pass = encoder.begin_render_pass(&descriptor,);

      // Opaque draws: bind the opaque pipeline (Is there only one opaque pipeline?)
      render_pass.set_pipeline(/* TODO */ todo!(),);

      // Set the texture and camera bindgroups
      render_pass.set_bind_group(1, &self.render_resources.camera.bindgroup, &[],);

      // TODO: I think iterating over a hashmap is slow
      for (id, instances,) in frame.instances {
        // TODO: I think creating an instance buffer like this is slow
        // Create the instance buffer
        let buffer = InstanceBuffer::new(&self.ctx.device, &instances,);

        // Buffer the instances
        render_pass.set_vertex_buffer(1, buffer.slice(..,),);

        // Draw
        // TODO: How was this initally fetched
        render_pass.draw_model_instanced(
          &self.render_resources.bindgroups,
          self.render_resources.models.get(/* TODO */ todo!(),),
          instances.range(),
        );
      }
    }

    self.ctx.queue.submit(once(encoder.finish(),),);
    output.present();

    Ok((),)
  }

  /// Adds a [`Model`] to the [`Renderer`] and returns its [`ModelId`]
  pub fn add_model(&mut self, name:&str,) -> ModelId {
    // TODO: Load model can take a graphics contex
    let model = load_model(self, name,);

    // Initially this returned a model which I then stored but the model data is all
    // buffered All I need is a handle to it

    self.render_resources.models.alloc(model,)
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
