// use super::{Dimensions, HorizontalAnchor, Orientation, Rect, VerticalAnchor};
// use crate::math::Vec3;

// pub struct UIConfigInfo {
//   pub(super) screen_dimensions: Dimensions,
//   pub(super) dimensions: Dimensions,
//   ///Position of the element's center
//   pub(super) position: Vec3,
// }

// impl UIConfigInfo {
//   //Pixels per units
//   const DPI: i32 = 160;

//   pub fn new(screen_dimensions: Dimensions, parent_dimensions: Dimensions) -> UIConfigInfoBuilder {
//     let dimensions = Dimensions::new(1280, 720);
//     let orientation = Orientation::default();
//     UIConfigInfoBuilder {
//       dimensions,
//       parent_dimensions,
//       screen_dimensions,
//       orientation,
//     }
//   }

//   pub fn ndc_position(&self) -> Vec3 {
//     let x = 2.0 * self.position.x / self.screen_dimensions.width as f32 - 1.0;
//     let y = 1.0 - (2.0 * self.position.y as f32) / self.screen_dimensions.height as f32;
//     let z = 0.0;

//     Vec3::new(x, y, z)
//   }

//   // ///Returns the element's bounding [`Rect`].
//   // /// During calculation, scales the `height` by the Window's aspect ratio to compensate for viewport distortion.
//   // pub fn rect(&self) -> Rect {
//   //   let width = self.dimensions.width as f32 / Self::DPI as f32;
//   //   let height = self.dimensions.height as f32 * self.screen_dimensions.aspect / Self::DPI as f32;
//   //   Rect::new(width, height)
//   // }

//   ///Returns the element's bounding [`Rect`].
//   /// During calculation, scales the `height` by the Window's aspect ratio to compensate for viewport distortion.
//   pub fn render_rect(&self) -> Rect {
//     let width = self.dimensions.width as f32 / Self::DPI as f32;
//     let height = self.dimensions.height as f32 * self.screen_dimensions.aspect / Self::DPI as f32;
//     Rect::new(width, height)
//   }
// }

// pub struct UIConfigInfoBuilder {
//   dimensions: Dimensions,
//   parent_dimensions: Dimensions,
//   screen_dimensions: Dimensions,
//   orientation: Orientation,
// }

// impl UIConfigInfoBuilder {
//   pub fn build(&self) -> UIConfigInfo {
//     //these dimensions need to be the parent dimensions
//     let dimensions = self.dimensions;
//     let screen_dimensions = self.screen_dimensions;
//     let orientation = self.orientation;

//     //Calculate the element and parent rectangles

//     //Calculate the left and right values of the rectangle's corners in normalized device coordinates (range [-1,1]) using the vertical orientation
//     match orientation.horizontal_anchor {
//       HorizontalAnchor::Center => {}
//       HorizontalAnchor::Left => {}
//       HorizontalAnchor::Right => {}
//     }

//     //Calculate the position of the widget in normalized device coordinates (range [-1,1])
//     match orientation.vertical_anchor {
//       VerticalAnchor::Center => {}
//       VerticalAnchor::Top => {}
//       VerticalAnchor::Bottom => {}
//     }

//     UIConfigInfo {
//       screen_dimensions,
//       dimensions,
//       position,
//     }
//   }

//   //functions to update the config info
//   pub fn horizontal_anchor(&mut self, anchor: HorizontalAnchor) -> &mut Self {
//     self.orientation.horizontal_anchor = anchor;
//     self
//   }
//   // pub fn horizontal_offset(&mut self, offset: f32) {}
//   pub fn vertical_anchor(&mut self, anchor: VerticalAnchor) -> &mut Self {
//     self.orientation.vertical_anchor = anchor;
//     self
//   }
//   // pub fn vertical_offset(&mut self, offset: f32) -> &mut Self {}
//   pub fn width(&mut self, width: f32) -> &mut Self {
//     self.dimensions.resize(Some(width as i32), None);
//     self
//   }
//   pub fn height(&mut self, height: f32) -> &mut Self {
//     self.dimensions.resize(None, Some(height as i32));
//     self
//   }
// }
