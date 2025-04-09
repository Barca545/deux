// TODO: Why does this exist?
// TODO: Should this even be in `math`

#[derive(Debug, Clone, Copy,)]
pub struct Dimensions {
  pub width:u32,
  pub height:u32,
  pub aspect:f32,
}

impl Dimensions {
  ///Create new [`Dimensions`].
  pub fn new(width:u32, height:u32,) -> Self {
    let aspect = width as f32 / height as f32;
    Dimensions {
      height,
      width,
      aspect,
    }
  }

  ///Resize the [`Dimensions`].
  pub fn resize(&mut self, width:Option<u32,>, height:Option<u32,>,) {
    if let Some(width,) = width {
      self.width = width;
    }

    if let Some(height,) = height {
      self.height = height;
    }

    self.aspect = self.width as f32 / self.height as f32;
  }
}
