use super::{Container, HorizontalAlign, UIConfigInfo, UIConfigInfoBuilder, VerticalAlign};
use crate::{
  math::Dimensions,
  ui::{render_info::WidgetRenderInfo, DisplayBox},
  view::Mesh,
};
use eyre::Result;
use gl::Gl;

pub enum WidgetType {
  DisplayBox,
}

pub struct WidgetBuilder<'b> {
  widget_type: WidgetType,
  parent: &'b mut dyn Container,
  config: UIConfigInfoBuilder,
  //Only needed for ones that render
  gl: Option<&'b Gl>,
  texture_name: Option<&'b str>,
  screen_dimensions: Option<&'b Dimensions>,
}

impl<'b> WidgetBuilder<'b> {
  pub fn new(widget_type: WidgetType, parent: &'b mut dyn Container) -> WidgetBuilder<'b> {
    let config = UIConfigInfo::new();
    WidgetBuilder {
      widget_type,
      parent,
      config,
      gl: None,
      screen_dimensions: None,
      texture_name: None,
    }
  }

  pub fn mesh(&mut self, gl: &'b Gl, screen_dimensions: &'b Dimensions, texture_name: &'b str) -> &mut Self {
    self.gl = Some(gl);
    self.screen_dimensions = Some(screen_dimensions);
    self.texture_name = Some(texture_name);
    self
  }

  //might need to return a box that then gets downcast or something
  // or I make a builder trait that has an associated type?
  //actually if I register it with the parent in the build method...
  pub fn build(&mut self) -> Result<()> {
    let config = self.config.parent(self.parent).build()?;
    todo!()
    // match self.widget_type {
    //   WidgetType::DisplayBox => {
    //     //Create the fields of Display
    //     let rect = &config.ndc_rect(self.screen_dimensions.unwrap());
    //     let info = WidgetRenderInfo::new(self.gl.unwrap(), self.texture_name.unwrap(), rect);
    //     let mesh = Mesh::from(info);
    //     let children = Vec::new();

    //     //Register display with its parent
    //     let display = DisplayBox { config, children, mesh };
    //     self.parent.register(Box::new(display))
    //   }
    // }
    // Ok(())
  }

  pub fn horizontal_align(&mut self, align: HorizontalAlign) -> &mut Self {
    self.config.orientation.horizontal_align = align;
    self
  }

  pub fn vertical_align(&mut self, align: VerticalAlign) -> &mut Self {
    self.config.orientation.vertical_align = align;
    self
  }

  pub fn dimensions(&mut self, width: i32, height: i32) -> &mut Self {
    self.config.dimensions(width, height);
    self
  }
}
