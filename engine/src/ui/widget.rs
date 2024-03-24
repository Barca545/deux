use super::{response::Response, ui::UI};

// Refactor:
// -Does update need &mut Ui or is &Ui fine?

pub trait Widget: 'static {
  ///Create a [`Response`] for the implementor in the frame.
  fn update(&self, ui: &UI) -> Response;
}
