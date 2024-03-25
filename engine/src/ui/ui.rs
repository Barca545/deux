use super::{
  base_widget::{HorizontalAnchor, Orientation, VerticalAnchor},
  response::Response,
  ui_config::UIConfigInfo,
  widget::Widget,
};
use crate::{
  math::{
    math::{Point2, Point3},
    Rect, Vec3,
  },
  view::{render_gl::Vertex, Mesh},
};
use gl::Gl;
use glfw::RenderContext;

// Refactor:
// -See if Rect can be reduced to just x/y value and max x = x min x = -x etc
// -Change default dimensions
// -Internal math should be in dp
// -Pick a dp value
// -In order to make it so that I don't have too many Responses for an element in a frame,
// -I don't think UI actually needs another height and width the config info should be enough
// -UI should have the option to be renderable in case I want to use it as a background
// -In order to use the position to render it I need a uniform in the shader that takes in the position
// -

// render_context

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

  pub fn register(&mut self, widget: impl Widget) {
    self.children.push(Box::new(widget))
  }

  ///Iterates through the [`UI`]'s `children` and calls their update method.
  pub fn update(&mut self) -> Vec<Response> {
    let mut responses = Vec::with_capacity(self.children.len());

    for widget in &self.children {
      let response = widget.update(self);
      responses.push(response)
    }

    //The below is only needed if the update method for some reason requires &mut Ui

    //   //Create temporary vectors to hold the children
    //   let mut children = Vec::with_capacity(self.children.len());
    //   let mut new_children = Vec::with_capacity(self.children.len());

    //   //Move children out of self.children and into the temporary children structure
    //   for widget in self.children.drain(..) {
    //     children.push(widget);
    //   }

    //   //Fetch the response from the widget and move the widget into the new_children vector
    //   for widget in children {
    //     let response = widget.update(self);
    //     responses.push(response);
    //     new_children.push(widget);
    //   }

    //   //Reset self.children
    //   self.children = new_children;
    responses
  }

  pub fn resize(&mut self) {}
}

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
