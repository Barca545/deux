use crate::math::Dimensions;

// Refactor:
// -Offset should really be called padding.

///Indicates the widget's orientation with respect to an align in its parent.
/// For objects with no parent, the `Top` and `Bottom` are with respect to the screen.
#[derive(Debug, Clone, Copy, Default)]
pub enum VerticalAlign {
  #[default]
  Center,
  Top,
  Bottom,
}

///Indicates the widget's orientation with respect to its parent.
/// For objects with no parent, the `Left` and `Right` are with respect to the screen.
#[derive(Debug, Clone, Copy, Default)]
pub enum HorizontalAlign {
  #[default]
  Center,
  Left,
  Right,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Orientation {
  pub horizontal_align: HorizontalAlign,
  pub horizontal_offset: f32,
  pub vertical_align: VerticalAlign,
  pub vertical_offset: f32,
}

impl Orientation {
  pub fn new(horizontal_align: HorizontalAlign, horizontal_offset: f32, vertical_align: VerticalAlign, vertical_offset: f32, screen: &Dimensions) -> Self {
    //Convert the offsets to normalized device coordinates(range [-1,1])
    let mut horizontal_offset = horizontal_offset;
    let mut vertical_offset = vertical_offset;

    //Calculate the offset as a percentage of the screen dimensions
    horizontal_offset = (horizontal_offset / 100.0) * screen.width as f32;
    vertical_offset = (vertical_offset / 100.0) * screen.height as f32;

    Orientation {
      horizontal_align,
      horizontal_offset,
      vertical_align,
      vertical_offset,
    }
  }
}
