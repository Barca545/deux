use super::{Response, Widget};

// Refactor:
// -Create an add method that handles the construction logic

pub trait Container: Widget {
  ///Registers a child [`Widget`].
  fn register(&mut self, widget: Box<dyn Widget>);

  fn children(&self) -> &Vec<Box<dyn Widget>>;

  fn containter_update(&self) -> Vec<Response> {
    let mut responses = Vec::with_capacity(self.children().len());

    for widget in self.children() {
      let response = widget.update().pop().unwrap();
      responses.push(response)
    }

    responses
  }
}
