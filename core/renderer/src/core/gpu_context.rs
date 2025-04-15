use wgpu::{
  Device, DeviceDescriptor, Features, Instance as GpuInstance, InstanceDescriptor, PowerPreference,
  Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureUsages,
};
use windowing::windowing::Window;

// TODO: Document the purpose of this
/// The GPU resources needed for rendering with wgpu.
pub struct GpuContext {
  // canvas:Arc<Canvas<sdl2Window,>,>,
  /// The rendering surface, representing the window or screen where the
  /// graphics will be displayed. It is the interface between wgpu and the
  /// platform's windowing system, enabling rendering onto the screen.
  pub surface: Surface<'static,>,
  // TODO: Do I need to keep this hanging around after creating the surface?
  pub config: SurfaceConfiguration,
  // TODO: Do I need an adaptor
  // /// The adapter that represents the GPU or a rendering backend. It provides
  // /// information about the capabilities of the hardware and is used to
  // /// request a logical device (`wgpu::Device`).
  // adaptor:Adapter,
  /// The [`Device`] serves as an interface to the GPU. It is
  /// responsible for creating resources such as buffers, textures, and
  /// pipelines, and manages the execution of commands. The `Device` provides
  /// a connection to the physical hardware represented by an
  /// [`Adapter`](wgpu::Adapter).
  pub device: Device,
  // TODO: Unsure Context should hold the queue
  /// The command [`Queue`] manages the submission of command buffers to the
  /// GPU for execution. It is used to send rendering and computation commands
  /// to the device. The `Queue` ensures commands are executed in the
  /// correct order and manages synchronization.
  pub queue: Queue,
  // TODO: Do I need to store the here in this way? Is there another way to store it?
  // size:PhysicalSize<u32,>,
}

impl GpuContext {
  pub async fn new(window: &Window,) -> Self {
    let size = window.inner_size();

    // Create the instance
    let instance_desc = InstanceDescriptor::default();
    let instance = GpuInstance::new(&instance_desc,);

    // Create the surface
    let surface = unsafe {
      instance
        .create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window.inner,).unwrap(),)
        .unwrap()
    };

    // Set the adaptor options and request an adapter
    let mut options = RequestAdapterOptions::default();
    options.power_preference = PowerPreference::HighPerformance;
    let adapter = instance.request_adapter(&options,).await.unwrap();

    // Create the device and command_queue
    let descriptor = DeviceDescriptor {
      label: None,
      required_features: Features::empty(),
      required_limits: Default::default(),
      // TODO: Figure out what manual config is needed
      memory_hints: wgpu::MemoryHints::MemoryUsage,
      trace: wgpu::Trace::Off,
    };
    let (device, queue,) = adapter.request_device(&descriptor,).await.unwrap();

    println!("Using backend: {:?}", adapter.get_info().backend);

    // Set the texture format as sRGB
    let surface_capabilities = surface.get_capabilities(&adapter,);
    let surface_format = surface_capabilities
      .formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb(),)
      .next()
      .unwrap_or(surface_capabilities.formats[0],);

    // Configure the surface's texture
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
    surface.configure(&device, &config,);

    // Create the GPU Context
    GpuContext {
      surface,
      config,
      device,
      queue,
    }
  }
}
