use super::{
  widget_builder::{WidgetBuilder, WidgetType},
  Container, Response, UIConfigInfo, Widget,
};
use crate::math::{Dimensions, Rect};
use gl::Gl;

// Refactor:
// -Get the screen dimensions from Parent somehow?

///[`Container`] for wrapping [`Widget`]s.
pub struct DisplayBox {
  pub(super) config: UIConfigInfo,
  // ///Pointer to the parent [`Container`].
  // pub(super) parent: Rc<dyn Container>,
  pub(super) children: Vec<Box<dyn Widget>>,
  // pub(super) mesh: Mesh,
}

impl DisplayBox {
  pub fn new<'b>(parent: &'b mut dyn Container) -> WidgetBuilder<'b> {
    let widget_type = WidgetType::DisplayBox;
    WidgetBuilder::new(widget_type, parent)
  }
}

impl Widget for DisplayBox {
  fn update(&self) -> Vec<Response> {
    self.containter_update()
  }

  fn rect(&self) -> Rect {
    self.config.rect
  }
}

impl Container for DisplayBox {
  fn register(&mut self, widget: Box<dyn Widget>) {
    self.children.push(widget)
  }

  fn children(&self) -> &Vec<Box<dyn Widget>> {
    &self.children
  }
}
