use super::{Dimensions, HorizontalAnchor, Orientation, VerticalAnchor, Widget};
use crate::{
  errors::UIErrors,
  math::{Rect, Vec3},
};
use eyre::Result;

// Refactor:
// -Dimension info? or Shape?
// -make the dimensions of a child automatically fill the parent if none are specified

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
    let dimensions = Dimensions::new(1280, 720);
    let orientation = Orientation::default();
    UIConfigInfoBuilder {
      dimensions,
      parent_rect: None,
      orientation,
    }
  }

  pub fn position(&self) -> Vec3 {
    let x = self.rect.min.x + (self.dimensions.width / 2) as f32;
    let y = self.rect.min.y + (self.dimensions.height / 2) as f32;
    let z = 0.0;
    let position = Vec3::new(x, y, z);
    position
  }

  ///Returns the element's bounding [`Rect`].
  /// During calculation, scales the `height` by the Window's aspect ratio to compensate for viewport distortion.
  pub fn render_rect(&self, screen_dimensions: Dimensions) -> Rect {
    let width = self.dimensions.width as f32 / Self::DPI as f32;
    let height = self.dimensions.height as f32 * screen_dimensions.aspect / Self::DPI as f32;
    Rect::new(width, height)
  }
}

pub struct UIConfigInfoBuilder {
  dimensions: Dimensions,
  parent_rect: Option<Rect>,
  orientation: Orientation,
}

impl UIConfigInfoBuilder {
  pub fn build(&self) -> Result<UIConfigInfo> {
    //these dimensions need to be the parent dimensions
    let dimensions = self.dimensions;
    let orientation = self.orientation;

    //Calculate the element and parent rectangles
    let mut widget_rect = Rect::new(dimensions.width as f32, dimensions.height as f32);
    let parent_rect = self.parent_rect;

    match parent_rect {
      Some(parent_rect) => {
        //Calculate the left and right values of the rectangle's corners in normalized device coordinates (range [-1,1]) using the vertical orientation
        match orientation.horizontal_anchor {
          HorizontalAnchor::Center => {}
          HorizontalAnchor::Left => {
            widget_rect.min.x = parent_rect.min.x;
            widget_rect.max.x = widget_rect.min.x + dimensions.width as f32;
          }
          HorizontalAnchor::Right => {
            widget_rect.max.x = parent_rect.max.x;
            widget_rect.min.x = widget_rect.max.x - dimensions.width as f32;
          }
        }

        //Calculate the position of the widget in normalized device coordinates (range [-1,1])
        match orientation.vertical_anchor {
          VerticalAnchor::Center => {}
          VerticalAnchor::Top => {
            widget_rect.max.y = parent_rect.max.y;
            widget_rect.min.y = widget_rect.max.y - dimensions.height as f32;
          }
          VerticalAnchor::Bottom => {
            dbg!(parent_rect.min.y);
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

  pub fn parent(&mut self, parent: &impl Widget) -> &mut Self {
    self.parent_rect = Some(parent.rect());
    self
  }

  ///Prefer the `parent` method. This is really only useful for creating the `UI` struct for the whole game from the `Window`'s dimensions.
  pub fn parent_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
    self.parent_rect = Some(Rect::new(dimensions.width as f32, dimensions.height as f32));
    self
  }

  //functions to update the config info
  pub fn horizontal_anchor(&mut self, anchor: HorizontalAnchor) -> &mut Self {
    self.orientation.horizontal_anchor = anchor;
    self
  }

  pub fn vertical_anchor(&mut self, anchor: VerticalAnchor) -> &mut Self {
    self.orientation.vertical_anchor = anchor;
    self
  }

  pub fn width(&mut self, width: f32) -> &mut Self {
    self.dimensions.resize(Some(width as i32), None);
    self
  }
  pub fn height(&mut self, height: f32) -> &mut Self {
    self.dimensions.resize(None, Some(height as i32));
    self
  }
}

#[cfg(test)]
mod test {
  use super::UIConfigInfo;
  use crate::ui::{Dimensions, HorizontalAnchor, VerticalAnchor};

  #[test]
  fn rect_position() {
    //Create the screen wrapper
    let screen_dimensions = Dimensions::new(400, 400);

    //Create the parent wrapper
    let element = UIConfigInfo::new()
      .parent_dimensions(screen_dimensions)
      .width(200.0)
      .height(200.0)
      .horizontal_anchor(HorizontalAnchor::Left)
      .vertical_anchor(VerticalAnchor::Bottom)
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
}
