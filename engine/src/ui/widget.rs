use crate::math::Rect;

use super::{response::Response, ui::UI};

// Refactor:
// -Does update need &mut Ui or is &Ui fine?
// -Implement for UI

pub trait Widget: 'static {
  ///Create a [`Response`] for the implementor in the frame.
  fn update(&self, ui: &UI) -> Response;

  ///Return the Widget's [`Rect`].
  fn rect(&self) -> Rect;

  // fn resize(&mut self);
}
