use crate::renderer::sdl2_helpers::PhysicalSize;
use sdl2::{render::Canvas, video::Window as sdl2Window};
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};

// TODO: Document the purpose of this
pub struct GpuContext {
  canvas:Arc<Canvas<sdl2Window,>,>,
  surface:Surface<'static,>,
  device:Device,
  // TODO: Unsure Context should hold the queue
  queue:Queue,
  config:SurfaceConfiguration,
  // TODO: Do I need to store the here in this way? Is there another way to store it?
  size:PhysicalSize<u32,>,
}
