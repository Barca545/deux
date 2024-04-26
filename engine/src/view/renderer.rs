use super::{
  camera::Camera,
  render_gl::{
    buffer::{IndexBuffer, VertexBuffer},
    ModelVertex, Texture,
  },
};
use crate::{math::Transforms, view::render_gl::Vertex};
use eyre::Result;
use std::{iter::once, sync::Arc};
use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Color,
  ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device, DeviceDescriptor, Face, Features, FragmentState, FrontFace, IndexFormat, Instance,
  InstanceDescriptor, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology, Queue,
  RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType, ShaderModuleDescriptor,
  ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
  VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

// Refactor:
// -Delete the create_gl function from the create module
// -Does the Adapter/Device need to be released at the end of the program?
// -Swap the frame buffer?
// -Move creating the buffer out of the new method
// -Move creating the pipeline[s] out of the new method
// -Move the buffers onto a mesh?
// -For pipeline should use the desc of the vert type, not the Self::method format
// -Camera and transforms can't be on this struct?

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
  vertex_buffer: VertexBuffer,
  index_buffer: IndexBuffer,
  diffuse_bind_group: BindGroup,
  camera: Camera,
  transforms: Transforms,
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

    //Set the texture as sRGB format
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

    //Create the texture
    let texture = Texture::new(&device, &queue, "ground");

    //Bind the texture
    let texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: Some((texture.label.to_owned() + "layout").as_str()),
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
    });

    let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some(texture.label),
      layout: &texture_bind_group_layout,
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: BindingResource::TextureView(&texture.view),
        },
        BindGroupEntry {
          binding: 1,
          resource: BindingResource::Sampler(&texture.sampler),
        },
      ],
    });

    //Create the render pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("render pipeline layout"),
      bind_group_layouts: &[&texture_bind_group_layout],
      push_constant_ranges: &[],
    });

    //Load and instantiate the shaders
    let shader = device.create_shader_module(ShaderModuleDescriptor {
      label: Some("shader"),
      source: ShaderSource::Wgsl(include_str!("C:\\Users\\jamar\\Documents\\Hobbies\\Coding\\deux\\assets\\shaders\\ModelShader.wgsl").into()),
    });

    //Create the render pipeline
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
      label: Some("render pipeline"),
      layout: Some(&pipeline_layout),
      vertex: VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[ModelVertex::desc()],
      },
      fragment: Some(FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(ColorTargetState {
          format: config.format,
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
      depth_stencil: None,
      multisample: MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview: None,
    });

    //Create the vertex buffer
    let vertices = vec![
      ModelVertex::from((-0.0868241, 0.49240386, 0.0, 0.4131759, 0.99240386)),
      ModelVertex::from((-0.49513406, 0.06958647, 0.0, 0.0048659444, 0.56958647)),
      ModelVertex::from((-0.21918549, -0.44939706, 0.0, 0.28081453, 0.05060294)),
      ModelVertex::from((0.35966998, -0.3473291, 0.0, 0.85967, 0.1526709)),
      ModelVertex::from((0.44147372, 0.2347359, 0.0, 0.9414737, 0.7347359)),
    ];

    let indices = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];

    //Create the vertex and index buffers
    let vertex_buffer = VertexBuffer::new(&device, vertices);
    let index_buffer = IndexBuffer::new(&device, indices);

    //Create the camera and transforms
    let camera = Camera::default();
    let transforms = Transforms::new((config.width / config.height) as f32);

    Renderer {
      surface,
      device,
      queue,
      config,
      size,
      window,
      pipeline,
      vertex_buffer,
      index_buffer,
      diffuse_bind_group,
      camera,
      transforms,
    }
  }

  ///Add a new [`RenderPipeline`] to the [`Renderer`].
  pub fn add_pipeline(&mut self) {
    //Move the logic for adding a pipeline to the renderer here
    todo!()
  }

  ///Add a new [`Buffer`] to the [`Renderer`].
  pub fn add_buffer(&mut self) {
    //Move the logic for adding a buffer to the renderer here
    todo!()
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
    }
  }

  pub fn update(&mut self) {}

  pub fn render(&self) -> Result<()> {
    //Get a new texture to render to from the surface
    let output = self.surface.get_current_texture()?;

    //Create a texture view to control how the code renders to the texture
    let view = output.texture.create_view(&TextureViewDescriptor::default());

    //Create a command encoder for draw commands
    let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Render Encoder") });

    //Create a render pass discritor
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
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
    };

    //Create a render pass
    {
      let mut render_pass = encoder.begin_render_pass(&descriptor);
      //Set the pipeline and bindgroup
      render_pass.set_pipeline(&self.pipeline);
      render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

      //Bind the buffers and draw
      render_pass.set_vertex_buffer(0, self.vertex_buffer.slice());
      render_pass.set_index_buffer(self.index_buffer.slice(), IndexFormat::Uint16);
      render_pass.draw_indexed(0..self.index_buffer.len, 0, 0..1)
    }

    //Submit the pass to the queue
    self.queue.submit(once(encoder.finish()));
    output.present();
    Ok(())

    //Swap the frame buffer
  }
}
