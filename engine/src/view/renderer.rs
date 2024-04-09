use crate::ecs::World;
use eyre::Result;
use std::{iter::once, sync::Arc};
use wgpu::{
  Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, LoadOp, Operations, PowerPreference, Queue,
  RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};

// Refactor:
// -Should the stencil test stuff actually be set in the create GL function
// -Is there a reason I call gl context version in create gl? it's never used
// -Delete the create gl function from the create module
// -RenderContext is useful for getting the glfw struct but not the actual window
// -Does the Adapter/Device need to be released at the end of the program?
// -I think this is basically the wgpu state thing I just created

pub struct Renderer {
  //reference to the window
  //reference to transforms?
  //I kinda think it might be able to take in the above info in render
  window: Arc<Window>,
  surface: Surface<'static>,
  device: Device,
  queue: Queue,
  config: SurfaceConfiguration,
  size: PhysicalSize<u32>,
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

    Renderer {
      surface,
      device,
      queue,
      config,
      size,
      window,
    }
  }

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
      let render_pass = encoder.begin_render_pass(&descriptor);
    }

    //Submit the pass to the queue
    self.queue.submit(once(encoder.finish()));
    output.present();
    Ok(())

    //Swap the frame buffer
  }
}
