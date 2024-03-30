use super::{Container, HorizontalAlign, Orientation, VerticalAlign};
use crate::{
  errors::UIErrors,
  math::{math::to_ndc, Dimensions, Rect, Vec3},
};
use eyre::Result;

// Refactor:
// -Change default dimensions
// -Internal math should be in dp.
//  Pick a dp value
// -Rename this Dimension info? or Shape?
// -make the dimensions of a child automatically fill the parent if none are specified
// Confirm if the position also needs to be scaled by the aspect ration

#[derive(Debug, Clone, Copy)]
pub struct UIConfigInfo {
  pub(super) dimensions: Dimensions,
  ///Position of the element's center
  pub(super) position: Vec3,
  ///Model Space Rectangle
  pub rect: Rect,
}

impl UIConfigInfo {
  //Pixels per units
  const DPI: i32 = 160;

  pub fn new() -> UIConfigInfoBuilder {
    let orientation = Orientation::default();
    UIConfigInfoBuilder {
      dimensions: None,
      parent_rect: None,
      orientation,
    }
  }

  ///Returns the [`Widget`]'s position in NDC ([-1,1]).
  pub fn ndc_position(&self, screen_dimensions: &Dimensions) -> Vec3 {
    let mut x = self.rect.min.x + (self.dimensions.width / 2) as f32;
    let mut y = self.rect.min.y + (self.dimensions.height / 2) as f32;
    let z = 0.0;

    //Convert the position to NDC
    (x, y) = to_ndc(x, y, screen_dimensions.width, screen_dimensions.height);

    let position = Vec3::new(x, y, z);
    position
  }

  ///Returns the `Widget`'s [`Rect`] in NDC ([-1,1]).
  pub fn ndc_rect(&self, screen_dimensions: &Dimensions) -> Rect {
    let mut width = self.dimensions.width as f32 / Self::DPI as f32;
    //Scale the height by the Window's aspect ratio to compensate for viewport distortion.
    let mut height = self.dimensions.height as f32 * screen_dimensions.aspect / Self::DPI as f32;

    //Convert the width and height to NDC
    (width, height) = to_ndc(width, height, screen_dimensions.width, screen_dimensions.height);

    Rect::new(width, height)
  }
}

pub struct UIConfigInfoBuilder {
  pub(super) dimensions: Option<Dimensions>,
  pub(super) orientation: Orientation,
  parent_rect: Option<Rect>,
}

impl UIConfigInfoBuilder {
  pub fn build(&self) -> Result<UIConfigInfo> {
    let orientation = self.orientation;

    let mut widget_rect;

    match self.parent_rect {
      Some(parent_rect) => {
        //Get the dimensions
        let dimensions;
        match self.dimensions {
          Some(widget_dimensions) => dimensions = widget_dimensions,
          None => {
            //Compute the dimensions as equal to the parent.
            let width = parent_rect.max.x - parent_rect.min.x;
            let height = parent_rect.max.y - parent_rect.min.y;
            dimensions = Dimensions::new(width as i32, height as i32);
          }
        }

        //Calculate the element rectangle
        widget_rect = Rect::new(dimensions.width as f32, dimensions.height as f32);

        //Calculate the left and right values of the rectangle's corners in normalized device coordinates (range [-1,1]) using the vertical orientation
        match orientation.horizontal_align {
          HorizontalAlign::Center => {}
          HorizontalAlign::Left => {
            widget_rect.min.x = parent_rect.min.x;
            widget_rect.max.x = widget_rect.min.x + dimensions.width as f32;
          }
          HorizontalAlign::Right => {
            widget_rect.max.x = parent_rect.max.x;
            widget_rect.min.x = widget_rect.max.x - dimensions.width as f32;
          }
        }

        //Calculate the position of the widget in normalized device coordinates (range [-1,1])
        match orientation.vertical_align {
          VerticalAlign::Center => {}
          VerticalAlign::Top => {
            widget_rect.max.y = parent_rect.max.y;
            widget_rect.min.y = widget_rect.max.y - dimensions.height as f32;
          }
          VerticalAlign::Bottom => {
            widget_rect.min.y = parent_rect.min.y;
            widget_rect.max.y = widget_rect.min.y + dimensions.height as f32;
          }
        }

        let x = widget_rect.min.x + (dimensions.width / 2) as f32;
        let y = widget_rect.min.y + (dimensions.height / 2) as f32;
        let z = 0.0;
        let position = Vec3::new(x, y, z);

        Ok(UIConfigInfo {
          dimensions,
          position,
          rect: widget_rect,
        })
      }
      None => return Err(UIErrors::NoParentElement.into()),
    }
  }

  pub fn parent(&mut self, parent: &dyn Container) -> &mut Self {
    self.parent_rect = Some(parent.rect());
    self
  }

  ///Prefer the `parent` method. This is really only useful for creating the `UI` struct for the whole game from the `Window`'s dimensions.
  pub fn parent_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
    self.parent_rect = Some(Rect::new(dimensions.width as f32, dimensions.height as f32));
    self
  }

  //functions to update the config info
  pub fn horizontal_align(&mut self, align: HorizontalAlign) -> &mut Self {
    self.orientation.horizontal_align = align;
    self
  }

  pub fn vertical_align(&mut self, align: VerticalAlign) -> &mut Self {
    self.orientation.vertical_align = align;
    self
  }

  pub fn dimensions(&mut self, width: i32, height: i32) -> &mut Self {
    self.dimensions = Some(Dimensions::new(width, height));
    self
  }
}

#[cfg(test)]
mod test {
  use super::UIConfigInfo;
  use crate::{
    math::Dimensions,
    ui::{HorizontalAlign, VerticalAlign},
  };

  #[test]
  fn rect_position() {
    //Create the screen wrapper
    let screen_dimensions = Dimensions::new(400, 400);

    //Create the parent wrapper
    let element = UIConfigInfo::new()
      .parent_dimensions(screen_dimensions)
      .dimensions(200, 200)
      .horizontal_align(HorizontalAlign::Left)
      .vertical_align(VerticalAlign::Bottom)
      .build()
      .unwrap();

    //Assertions
    assert_eq!(element.rect.min.x, -200.0);
    assert_eq!(element.rect.max.x, 0.0);
    assert_eq!(element.rect.max.y, 0.0);
    assert_eq!(element.rect.min.y, -200.0);
    assert_eq!(element.position.x, -100.0);
    assert_eq!(element.position.y, -100.0);
  }

  #[test]
  fn convert_to_ndc() {
    //Create the screen wrapper
    let screen_dimensions = Dimensions::new(400, 400);

    //Create the parent wrapper
    let element = UIConfigInfo::new()
      .parent_dimensions(screen_dimensions)
      .dimensions(200, 200)
      .horizontal_align(HorizontalAlign::Left)
      .vertical_align(VerticalAlign::Bottom)
      .build()
      .unwrap();
  }
}
