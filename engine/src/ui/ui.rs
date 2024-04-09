use super::{response::Response, ui_config::UIConfigInfo, widget::Widget, Container};
use crate::math::Rect;
use glfw::RenderContext;

// Refactor:
// -In order to make it so that I don't have too many Responses for an element in a frame,
// -UI should have the option to be renderable in case I want to use it as a background
// -Unsure if the UI needs a render contex since it should never be processing inputs?

// #[derive(Debug)]
pub struct UI {
  pub(super) config: UIConfigInfo,
  ctx: RenderContext,
  children: Vec<Box<dyn Widget>>,
}

impl UI {
  pub fn new(config: UIConfigInfo, ctx: RenderContext) -> Self {
    let children = Vec::new();
    UI { config, ctx, children }
  }

  //   //The below is only needed if the update method for some reason requires &mut Ui

  //   //   //Create temporary vectors to hold the children
  //   //   let mut children = Vec::with_capacity(self.children.len());
  //   //   let mut new_children = Vec::with_capacity(self.children.len());

  //   //   //Move children out of self.children and into the temporary children structure
  //   //   for widget in self.children.drain(..) {
  //   //     children.push(widget);
  //   //   }

  //   //   //Fetch the response from the widget and move the widget into the new_children vector
  //   //   for widget in children {
  //   //     let response = widget.update(self);
  //   //     responses.push(response);
  //   //     new_children.push(widget);
  //   //   }

  //   //   //Reset self.children
  //   //   self.children = new_children;
  //   responses
  // }

  pub fn resize(&mut self) {}
}

impl Widget for UI {
  ///Iterates through the [`UI`]'s `children` and calls their update method.
  fn update(&self) -> Vec<Response> {
    self.containter_update()
  }

  fn rect(&self) -> Rect {
    self.config.rect
  }
}

impl Container for UI {
  fn register(&mut self, widget: Box<dyn Widget>) {
    self.children.push(widget)
  }

  fn children(&self) -> &Vec<Box<dyn Widget>> {
    &self.children
  }
}
