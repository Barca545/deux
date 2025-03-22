use sdl2::{pixels::Color, video::Window as sdl2Window};

/// The color Black in RGB form.
pub const BLACK:Color = Color::RGB(0, 0, 0,);

// Helper data structures for managing windowing and other features.

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash,)]
/// Screen Dimensions represented in physical pixels.
// TODO: Confirm this needs to use a generic
pub struct PhysicalSize<P,> {
  pub width:P,
  pub height:P,
}

impl<P,> PhysicalSize<P,> {
  #[inline]
  pub const fn new(width:P, height:P,) -> Self {
    PhysicalSize { width, height, }
  }
}

pub struct PhysicalPosition {
  pub x:f64,
  pub y:f64,
}

impl PhysicalPosition {
  #[inline]
  pub const fn new(x:f64, y:f64,) -> Self {
    PhysicalPosition { x, y, }
  }
}

// I wonder if it's worth rexporting window here in a format wgpu can play nice
// with

// TODO: Try to make a trait that implements the stuff wgu expects?
pub trait Window {
  fn inner_size(&self,) -> PhysicalSize<u32,>;
}

impl Window for sdl2Window {
  fn inner_size(&self,) -> PhysicalSize<u32,> {
    let size = self.size();
    PhysicalSize {
      width:size.0,
      height:size.1,
    }
  }
}
