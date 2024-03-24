use super::{
  base_widget::{HorizontalAnchor, Orientation, VerticalAnchor},
  response::Response,
  widget::Widget,
};
use crate::{
  math::Vec3,
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

// render_context

// #[derive(Debug)]
pub struct UI {
  id: usize,
  pub(super) config: UIConfigInfo,
  rect: Rect,
  ctx: RenderContext,
  children: Vec<Box<dyn Widget>>,
}

impl UI {
  pub fn new(id: usize, config: UIConfigInfo, ctx: RenderContext, width: f32, height: f32) -> Self {
    let rect = Rect::new(config, width, height);
    let children = Vec::new();
    UI {
      id,
      config,
      rect,
      ctx,
      children,
    }
  }

  pub fn register(&mut self, widget: impl Widget) {
    self.children.push(Box::new(widget))
  }

  ///Iterates through the [`Ui`]'s `children` and calls their update method.
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
  pub aspect: i32,
}

impl Dimensions {
  ///Create new [`Dimensions`].
  pub fn new(width: i32, height: i32) -> Self {
    let aspect = width / height;
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

    self.aspect = self.width / self.height;
  }
}

#[derive(Debug, Clone, Copy)]
pub struct UIConfigInfo {
  //Pixels per units
  dpi: i32,
  pub(super) dimensions: Dimensions,
  orientation: Orientation,
}

impl UIConfigInfo {
  pub fn new() -> UIConfigInfoBuilder {
    let dpi = 120;
    let dimensions = Dimensions::new(1280, 720);
    let orientation = Orientation::default();
    UIConfigInfoBuilder { dpi, dimensions, orientation }
  }
}

pub struct UIConfigInfoBuilder {
  dpi: i32,
  dimensions: Dimensions,
  orientation: Orientation,
}

impl UIConfigInfoBuilder {
  pub fn build(&self) -> UIConfigInfo {
    let dpi: i32 = self.dpi;
    let dimensions = self.dimensions;
    let orientation = self.orientation;
    UIConfigInfo { dpi, dimensions, orientation }
  }

  //functions to update the config info
  // pub fn horizontal_anchor(&mut self, anchor: HorizontalAnchor) -> &mut Self {}
  // pub fn horizontal_offset(&mut self, offset: f32) {}
  // pub fn vertical_anchor(&mut self, anchor: VerticalAnchor) -> &mut Self {}
  // pub fn vertical_offset(&mut self, offset: f32) -> &mut Self {}
  pub fn width(&mut self, width: f32) -> &mut Self {
    self.dimensions.resize(Some(width as i32), None);
    self
  }
  pub fn height(&mut self, height: f32) -> &mut Self {
    self.dimensions.resize(None, Some(height as i32));
    self
  }
}

pub struct Format {
  //opacity
  //align
}

pub struct Rect {
  top: f32,
  bottom: f32,
  left: f32,
  right: f32,
}

impl Rect {
  // pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {}
  pub fn new(config: UIConfigInfo, width: f32, height: f32) -> Self {
    let orientation = config.orientation;
    let dimensions = config.dimensions;

    let mut position = Vec3::default();
    let horizontal_offset = orientation.horizontal_offset;
    let vertical_offset = orientation.vertical_offset;

    //Calculate the left and right values of the rectangle's corners in normalized device coordinates (range [-1,1]) using the vertical orientation
    match orientation.horizontal_anchor {
      HorizontalAnchor::Center => {
        position.x = (dimensions.width as f32 / 2.0) + horizontal_offset;
      }
      HorizontalAnchor::Left => {
        position.x = horizontal_offset;
      }
      HorizontalAnchor::Right => {
        position.x = dimensions.width as f32 + horizontal_offset;
      }
    }

    //Calculate the position of the widget in normalized device coordinates (range [-1,1])
    match orientation.vertical_anchor {
      VerticalAnchor::Center => {
        position.y = (dimensions.height as f32 / 2.0) + vertical_offset;
      }
      VerticalAnchor::Top => {
        position.y = vertical_offset;
      }
      VerticalAnchor::Bottom => {
        position.y = dimensions.height as f32 + vertical_offset;
      }
    }

    //Calculate the x and y maxima
    let mut left = position.x - (width / 2.0);
    let mut right = position.x + (width / 2.0);
    let mut top = position.y - (height / 2.0);
    let mut bottom = position.y + (height / 2.0);

    //Ensure shape is within dimensions bounds

    //Center the widget horizontally if it is wider than its parent
    if width > dimensions.width as f32 {
      let x = dimensions.width as f32 / 2.0;
      left = x - (width / 2.0);
      right = x + (width / 2.0);
    }
    //Shift the widget right if it would exceed the left bound of its parent
    else if left < 0.0 {
      let horizontal_correction = 0.0 - left;
      left += horizontal_correction;
      right += horizontal_correction;
    }
    //Shift the widget right if it would exceed the right bound of its parent
    else if right > dimensions.width as f32 {
      let horizontal_correction = dimensions.width as f32 - right;
      left += horizontal_correction;
      right += horizontal_correction;
    }

    //Center the widget vertically if it is taller than its parent
    if height > dimensions.height as f32 {
      let y = dimensions.height as f32 / 2.0;
      top = y - (height / 2.0);
      bottom = y + (height / 2.0);
    }
    //Shift the widget up if it would exceed the top of its parent
    else if top < 0.0 {
      let vertical_correction = 0.0 - top;
      top += vertical_correction;
      bottom += vertical_correction;
    }
    //Shift the widget up if it would exceed the bottom of its parent
    else if bottom > dimensions.height as f32 {
      let vertical_correction = dimensions.height as f32 - bottom;
      top += vertical_correction;
      bottom += vertical_correction;
    }

    //Convert the maxima to ndc
    left = 2.0 * left / dimensions.width as f32 - 1.0;
    right = 2.0 * right / dimensions.width as f32 - 1.0;
    top = 1.0 - (2.0 * top as f32) / dimensions.height as f32;
    bottom = 1.0 - (2.0 * bottom as f32) / dimensions.height as f32;

    Rect { top, bottom, left, right }
  }
}

pub struct WidgetRenderInfo<'a> {
  gl: &'a Gl,
  texture_name: &'a str,
  rect: &'a Rect,
}

impl<'a> WidgetRenderInfo<'a> {
  pub fn new(gl: &'a Gl, texture_name: &'a str, rect: &'a Rect) -> Self {
    WidgetRenderInfo { gl, texture_name, rect }
  }
}

//move this into the mesh module
impl<'a> From<WidgetRenderInfo<'a>> for Mesh {
  fn from(value: WidgetRenderInfo) -> Self {
    //Define the gl, maxima, and texture name
    let gl = value.gl;
    let texture_name = value.texture_name;
    let right = value.rect.right;
    let left = value.rect.left;
    let top = value.rect.top;
    let bottom = value.rect.bottom;
    let z = -1.0;

    //Create the widget's vertices
    let v1 = Vertex::new([right, top, z], [1.0, 1.0]); //Top Right
    let v2 = Vertex::new([right, bottom, z], [1.0, 0.0]); // Bottom Right
    let v3 = Vertex::new([left, bottom, z], [0.0, 0.0]); // Bottom Left
    let v4 = Vertex::new([left, top, z], [0.0, 1.0]); // Top Left

    //Add the vertices to a vertices vector
    let mut vertices = Vec::new();
    vertices.push(v1);
    vertices.push(v2);
    vertices.push(v3);
    vertices.push(v4);

    //Create the indices
    let indices = vec![0, 1, 3, 1, 2, 3];

    //Create the widget's mesh
    let gl = gl.clone();
    Mesh::new(&gl, vertices, indices, texture_name)
  }
}
