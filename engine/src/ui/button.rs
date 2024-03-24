use super::ui::{UIConfigInfo, UI};
use crate::{
  errors::UIErrors,
  ui::ui::{Rect, WidgetRenderInfo},
  view::{render_gl::draw_elements, Mesh},
};
use eyre::Result;
use gl::Gl;

// Refactor:
// -Not correctly being scaled in relation to its

pub struct Button {
  name: String,
  config: UIConfigInfo,
  mesh: Mesh,
}

impl Button {
  pub fn new<'b>(name: &'b str, config: UIConfigInfo) -> ButtonBuilder<'b> {
    ButtonBuilder::new(name, config)
  }

  pub fn draw(&self, gl: &Gl) {
    let mesh = &self.mesh;
    draw_elements(gl, mesh)
  }
}

pub struct ButtonBuilder<'b> {
  name: &'b str,
  config: UIConfigInfo,
  gl: Option<&'b Gl>,
  texture_name: Option<&'b str>,
  parent: Option<&'b UI>,
  render: bool,
}

impl<'b> ButtonBuilder<'b> {
  pub fn new(name: &'b str, config: UIConfigInfo) -> Self {
    ButtonBuilder {
      name,
      config,
      gl: Default::default(),
      texture_name: Default::default(),
      parent: Default::default(),
      render: false,
    }
  }

  pub fn parent(&mut self, parent: &'b UI) -> &mut Self {
    self.parent = Some(parent);
    self
  }

  ///Adds the [`WidgetRenderInfo`] needed to render the [`Button`]'s [`Mesh`].
  pub fn mesh_info(&mut self, gl: &'b Gl, texture_name: &'b str) -> &mut Self {
    //Doesn't actually add the information because parent would be needed
    self.gl = Some(gl);
    self.texture_name = Some(texture_name);
    self.render = true;
    self
  }

  pub fn build(&self) -> Result<Button> {
    if !self.render {
      return Err(UIErrors::NoRenderInformation.into());
    }

    match self.parent {
      Some(parent) => {
        let parent_config = parent.config;
        let width = self.config.dimensions.width as f32;
        let height = self.config.dimensions.height as f32;
        let rect = Rect::new(parent_config, width, height);
        let info = WidgetRenderInfo::new(self.gl.unwrap(), self.texture_name.unwrap(), &rect);
        let mesh = Mesh::from(info);
        Ok(Button {
          name: self.name.to_string(),
          config: self.config,
          mesh,
        })
      }
      None => return Err(UIErrors::NoParentElement.into()),
    }
  }
}
