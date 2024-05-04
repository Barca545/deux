use super::{buffer::InstanceBuffer, camera::Camera, DrawModel, Frame, InstanceRaw, Instances, Model, ModelId, ModelVertex, Texture};
use crate::{
  component_lib::{Position, PreviousPosition, SkinnedRenderable, StaticRenderable},
  data_storage::Arena,
  ecs::World,
  filesystem::{load_model, load_shader},
  math::Transforms,
  time::ServerTime,
  utility::calculate_render_position,
  view::Vertex,
};
use eyre::Result;
use std::{iter::once, sync::Arc};
use wgpu::{
  util::{BufferInitDescriptor, DeviceExt},
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer,
  BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, CompareFunction, DepthBiasState, DepthStencilState, Device,
  DeviceDescriptor, Face, Features, FragmentState, FrontFace, Instance, InstanceDescriptor, LoadOp, MultisampleState, Operations, PipelineLayout,
  PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment,
  RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType, ShaderModule,
  ShaderStages, StencilState, StoreOp, Surface, SurfaceConfiguration, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor,
  TextureViewDimension, VertexBufferLayout, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

// Refactor:
// -Delete the create_gl function from the create module
// -Does the Adapter/Device need to be released at the end of the program?
// -Swap the frame buffer?
// -Create load functions for the shaders
// -Do static meshes need a different pipeline?

pub static mut MODEL_NUM: usize = 0;

pub struct Renderer {
  //reference to the window
  //I kinda think it might be able to take in the above info in render
  window: Arc<Window>,
  surface: Surface<'static>,
  device: Device,
  queue: Queue,
  config: SurfaceConfiguration,
  size: PhysicalSize<u32>,
  pipeline: RenderPipeline,
  camera_bind_group: BindGroup,
  camera_buffer: Buffer,
  depth_texture: Texture,
  models: Arena<Model>,
  frame: Frame,
}

impl Renderer {
  pub async fn new(window: Arc<Window>) -> Self {
    let size = window.inner_size();

    //Create the instance
    let instance_desc = InstanceDescriptor::default();
    let instance = Instance::new(instance_desc);

    //Create the surface
    let surface = instance.create_surface(window.clone()).unwrap();

    //Set the adaptor options and request an adapter
    let mut options = RequestAdapterOptions::default();
    options.power_preference = PowerPreference::HighPerformance;
    let adapter = instance.request_adapter(&options).await.unwrap();

    //Create the device and command_queue
    let descriptor = DeviceDescriptor {
      label: None,
      required_features: Features::empty(),
      required_limits: Default::default(),
    };
    let (device, queue) = adapter.request_device(&descriptor, None).await.unwrap();

    //Set the texture format as sRGB
    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities
      .formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap_or(surface_capabilities.formats[0]);

    //Configure the surface's texture
    let config = SurfaceConfiguration {
      usage: TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_capabilities.present_modes[0],
      alpha_mode: surface_capabilities.alpha_modes[0],
      desired_maximum_frame_latency: 2,
      view_formats: Vec::new(),
    };
    surface.configure(&device, &config);

    //Create the camera buffer
    //Buffer will be empty until the first update call
    let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("camera buffer"),
      contents: bytemuck::cast_slice(&[[[0; 4]; 4]]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    //Create the camera bindgroup layout
    let camera_bindgroup_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: Some("camera_bind_group_layout"),
      entries: &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
          ty: BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    //Create the camera bindgroup
    let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some("camera bind group"),
      layout: &camera_bindgroup_layout,
      entries: &[BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    //Create the render pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("render pipeline layout"),
      bind_group_layouts: &[&Self::texture_bind_group_layout(&device), &camera_bindgroup_layout],
      push_constant_ranges: &[],
    });

    //Load and instantiate the shaders
    let model_shader = load_shader(&device, "ModelShader").unwrap();

    let pipeline = Self::create_render_pipeline(
      &device,
      pipeline_layout,
      config.format,
      Some(Texture::DEPTH_FORMAT),
      &[ModelVertex::desc(), InstanceRaw::desc()],
      model_shader,
    );

    //Create the depth texture
    let depth_texture = Texture::create_depth_texture(&device, &config);

    Renderer {
      surface,
      device,
      queue,
      config,
      size,
      window,
      pipeline,
      camera_bind_group,
      camera_buffer,
      depth_texture,
      models: Arena::new(),
      frame: Frame::default(),
    }
  }

  ///Get the handle of the [`Renderer`]'s [`Window`].
  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);

      //Update the depth texture
      self.depth_texture = Texture::create_depth_texture(&self.device, &self.config);
    }
  }

  pub fn update(&mut self, world: &World) {
    //Update the camera
    let transforms = world.get_resource::<Transforms>().unwrap();
    let mut camera = world.get_resource_mut::<Camera>().unwrap();
    camera.update_pv(&transforms);

    //Create a new frame
    let mut frame = Frame::new(&camera);

    let server_time = world.get_resource::<ServerTime>().unwrap();

    //Render skinned models
    let mut query = world.query();
    let entities = query.with_component::<SkinnedRenderable>().unwrap().run();
    //Add every instance of a model which needs to be rendered to the frame
    for entity in entities {
      let model_id = entity.get_component::<SkinnedRenderable>().unwrap();
      let position = entity.get_component::<Position>().unwrap();
      let previous_position = entity.get_component::<PreviousPosition>().unwrap();

      let render_position = calculate_render_position(*previous_position, *position, server_time.get_interpolation_factor());
      frame.add_instance(&model_id.0, &render_position);
    }

    //Render static models
    let mut query = world.query();
    let entities = query.with_component::<StaticRenderable>().unwrap().run();
    //Add every instance of a model which needs to be rendered to the frame
    for entity in entities {
      let model_id = entity.get_component::<StaticRenderable>().unwrap();
      let position = entity.get_component::<Position>().unwrap();
      frame.add_instance(&model_id.0, &position);
    }

    self.frame = frame;
  }

  pub fn render(&self) -> Result<()> {
    //Buffer the camera matrix
    self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&self.frame.pv_mat));

    //Get a texture to render to from the surface
    let output = self.surface.get_current_texture()?;

    //Create a texture view to control how the code renders to the texture
    let view = output.texture.create_view(&TextureViewDescriptor::default());

    //Create a command encoder for draw commands
    let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Render Encoder") });

    //Create a render pass descriptor
    let descriptor = RenderPassDescriptor {
      label: Some("Render Pass Test"),
      color_attachments: &[Some(RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: Operations {
          load: LoadOp::Clear(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
          }),
          store: StoreOp::Store,
        },
      })],
      //Attach the depth stencil
      depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
        view: &self.depth_texture.view,
        depth_ops: Some(Operations {
          load: LoadOp::Clear(1.0),
          store: StoreOp::Store,
        }),
        stencil_ops: None,
      }),
      timestamp_writes: None,
      occlusion_query_set: None,
    };

    let instance_buffers = self
      .frame
      .instances
      .iter()
      .map(|instances| InstanceBuffer::new(&self.device, instances))
      .collect::<Vec<_>>();

    //Create a render pass
    {
      let mut render_pass = encoder.begin_render_pass(&descriptor);
      //Set the pipeline
      render_pass.set_pipeline(&self.pipeline);

      //Set the texture and camera bindgroups
      render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

      for model in self.frame.models() {
        //Buffer the instances
        render_pass.set_vertex_buffer(1, instance_buffers[model].slice(..));

        //Draw
        render_pass.draw_model_instanced(&self.models[model], self.frame.instances[model].range())
      }
    }

    //Submit the pass to the queue
    self.queue.submit(once(encoder.finish()));
    output.present();
    Ok(())

    //Swap the frame buffer
  }

  ///Adds a [`Model`] to the [`Renderer`] and returns it's [`ModelId`]
  pub fn add_model(&mut self, name: &str) -> ModelId {
    let model = load_model(name, &self.device, &self.queue);
    let model = self.models.alloc(model);

    unsafe { MODEL_NUM += 1 };

    ModelId(model)
  }

  ///Add a new [`RenderPipeline`] to the [`Renderer`].
  pub fn add_render_pipeline(&mut self) {
    //Move the logic for adding a pipeline to the renderer here
    todo!()
  }

  ///Create a new [`RenderPipeline`].
  fn create_render_pipeline(
    device: &Device,
    pipeline_layout: PipelineLayout,
    color_format: TextureFormat,
    depth_format: Option<TextureFormat>,
    vertex_layouts: &[VertexBufferLayout],
    shader: ShaderModule,
  ) -> RenderPipeline {
    //Create the render pipeline
    device.create_render_pipeline(&RenderPipelineDescriptor {
      label: Some("render pipeline"),
      layout: Some(&pipeline_layout),
      vertex: VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: vertex_layouts,
      },
      fragment: Some(FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(ColorTargetState {
          format: color_format,
          blend: Some(BlendState::REPLACE),
          write_mask: ColorWrites::ALL,
        })],
      }),
      primitive: PrimitiveState {
        topology: PrimitiveTopology::TriangleList,
        strip_index_format: None,
        //Cull triangles whose verts are not arranged counter clockwise
        front_face: FrontFace::Ccw,
        cull_mode: Some(Face::Back),
        //Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        polygon_mode: PolygonMode::Fill,
        //Requires Features::DEPTH_CLIP_CONTROL
        unclipped_depth: false,
        //Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },
      //Instantiate depth testing
      depth_stencil: depth_format.map(|format| DepthStencilState {
        format,
        depth_write_enabled: true,
        depth_compare: CompareFunction::Less,
        stencil: StencilState::default(),
        bias: DepthBiasState::default(),
      }),
      multisample: MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview: None,
    })
  }

  ///Returns the [`BindGroupLayout`] for a [`Texture`].
  pub fn texture_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: Some("layout bindgroup layout"),
      entries: &[
        BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Texture {
            multisampled: false,
            view_dimension: TextureViewDimension::D2,
            sample_type: TextureSampleType::Float { filterable: true },
          },
          count: None,
        },
        BindGroupLayoutEntry {
          binding: 1,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Sampler(SamplerBindingType::Filtering),
          count: None,
        },
      ],
    })
  }
}
