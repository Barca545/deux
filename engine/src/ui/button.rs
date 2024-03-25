use super::ui::{UIConfigInfo, UI};
use crate::{
  errors::UIErrors,
  math::{calculate_model_transform, Mat4, Vec3},
  ui::ui::{Rect, WidgetRenderInfo},
  view::{render_gl::draw_elements, Mesh},
};
use eyre::Result;
use gl::Gl;

// Refactor:
// -Not correctly being scaled in relation to its parent
// -Button builder could somehow be extended via traits or something to create all UI elemets?

pub struct Button {
  name: String,
  pub config: UIConfigInfo,
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
        let mut rect = self.config.rect();

        // self.fit_in_parent(&mut rect, parent.config);
        // self.to_ndc(&mut rect, parent.config);

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

  //needs to be fixed
  ///Ensures the [`Rect`] is within its parent's bounds.
  fn fit_in_parent(&self, rect: &mut Rect, parent_config: UIConfigInfo) {
    //If the widget is
  }
  // fn fit_in_parent(&self, rect: &mut Rect, parent_config: UIConfigInfo) {
  //   //width/height will be the dimensions from self.config
  //   let width = self.config.dimensions.width as f32;
  //   let height = self.config.dimensions.height as f32;

  //   //Center the widget horizontally if it is wider than its parent
  //   if width > parent_config.dimensions.width as f32 {
  //     let x = parent_config.dimensions.width as f32 / 2.0;
  //     rect.left = x - (width / 2.0);
  //     rect.right = x + (width / 2.0);
  //   }
  //   //Shift the widget right if it would exceed the left bound of its parent
  //   else if rect.left < 0.0 {
  //     let horizontal_correction = 0.0 - rect.left;
  //     rect.left += horizontal_correction;
  //     rect.right += horizontal_correction;
  //   }
  //   //Shift the widget right if it would exceed the right bound of its parent
  //   else if rect.right > parent_config.dimensions.width as f32 {
  //     let horizontal_correction = parent_config.dimensions.width as f32 - rect.right;
  //     rect.left += horizontal_correction;
  //     rect.right += horizontal_correction;
  //   }

  //   //Center the widget vertically if it is taller than its parent
  //   if height > parent_config.dimensions.height as f32 {
  //     let y = parent_config.dimensions.height as f32 / 2.0;
  //     rect.top = y - (height / 2.0);
  //     rect.bottom = y + (height / 2.0);
  //   }
  //   //Shift the widget up if it would exceed the top of its parent
  //   else if rect.top < 0.0 {
  //     let vertical_correction = 0.0 - rect.top;
  //     rect.top += vertical_correction;
  //     rect.bottom += vertical_correction;
  //   }
  //   //Shift the widget up if it would exceed the bottom of its parent
  //   else if rect.bottom > parent_config.dimensions.height as f32 {
  //     let vertical_correction = parent_config.dimensions.height as f32 - rect.bottom;
  //     rect.top += vertical_correction;
  //     rect.bottom += vertical_correction;
  //   }
  // }

  /////Convert the [`Button`]'s coordinates to ndc
  // fn to_ndc(&self, rect: &mut Rect, config: UIConfigInfo) {
  //   //Convert the maxima to ndc
  //   rect.left = 2.0 * rect.left / config.screen_dimensions.width as f32 - 1.0;
  //   rect.right = 2.0 * rect.right / config.screen_dimensions.width as f32 - 1.0;
  //   rect.top = 1.0 - (2.0 * rect.top as f32) / config.screen_dimensions.height as f32;
  //   rect.bottom = 1.0 - (2.0 * rect.bottom as f32) / config.screen_dimensions.height as f32;
  // }
}
