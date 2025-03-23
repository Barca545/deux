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

#[derive(Debug,)]
pub struct PhysicalPosition {
  pub x:f64,
  pub y:f64,
}

impl PhysicalPosition {
  #[inline]
  pub const fn new(x:f64, y:f64,) -> Self {
    PhysicalPosition { x, y, }
  }

  #[inline]
  /// Converts the given screen coordinates into [normalized device coordinates](https://learnopengl.com/Getting-started/Coordinate-Systems).
  pub fn from_screen_coords(x:i32, y:i32, dimensions:PhysicalSize<u32,>,) -> Self {
    let mut x = x as f64;
    let mut y = y as f64;
    x = 2.0 * x as f64 / dimensions.width as f64 - 1.0; //range [-1,1]
    y = 1.0 - (2.0 * y as f64) / dimensions.height as f64; //range [-1,1]
    PhysicalPosition::new(x, y,)
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
