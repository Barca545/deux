#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
  pub width: i32,
  pub height: i32,
  pub aspect: f32,
}

impl Dimensions {
  ///Create new [`Dimensions`].
  pub fn new(width: i32, height: i32) -> Self {
    let aspect = width as f32 / height as f32;
    Dimensions { height, width, aspect }
  }

  ///Resize the [`Dimensions`].
  pub fn resize(&mut self, width: Option<i32>, height: Option<i32>) {
    if let Some(width) = width {
      self.width = width;
    }

    if let Some(height) = height {
      self.height = height;
    }

    self.aspect = self.width as f32 / self.height as f32;
  }
}
