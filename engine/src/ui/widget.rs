use super::response::Response;
use crate::math::Rect;

// Refactor:
// -Does update need &mut Ui or is &Ui fine?
// -Implement for UI
// -Children can output a vector of one
// -Does update require parents?

///Widgets must hold a reference to their parent. Because the `update` method requires it.
pub trait Widget: 'static {
  ///Create a [`Response`] for the implementor in the frame.
  fn update(&self) -> Vec<Response>;

  ///Return the Widget's [`Rect`].
  fn rect(&self) -> Rect;

  // fn resize(&mut self);
}
