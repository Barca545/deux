use super::Widget;

pub trait Containter: Widget {
  ///Add a child UI element.
  fn register(&mut self, widget: impl Widget);
}
