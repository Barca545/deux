use super::{
  base_widget::{HorizontalAnchor, Orientation, VerticalAnchor},
  response::Response,
  widget::Widget,
};
use crate::{
  math::{
    math::{Point2, Point3},
    Vec3,
  },
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
// -I don't think UI actually needs another height and width the config info should be enough
// -UI should have the option to be renderable in case I want to use it as a background
// -In order to use the position to render it I need a uniform in the shader that takes in the position
// -

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
  pub fn new(id: usize, config: UIConfigInfo, ctx: RenderContext) -> Self {
    let dimensions = config.dimensions;
    let rect = Rect::new(dimensions.width as f32, dimensions.height as f32);
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
  pub aspect: f32,
}

impl Dimensions {
  ///Create new [`Dimensions`].
  pub fn new(width: i32, height: i32) -> Self {
    let aspect = width as f32 / height as f32;
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

    self.aspect = self.width as f32 / self.height as f32;
  }
}

#[derive(Debug, Clone, Copy)]
pub struct UIConfigInfo {
  pub(super) screen_dimensions: Dimensions,
  pub(super) dimensions: Dimensions,
  ///Position of the element's center
  pub(super) position: Vec3,
}

impl UIConfigInfo {
  //Pixels per units
  const DPI: i32 = 160;

  pub fn new(screen_dimensions: Dimensions) -> UIConfigInfoBuilder {
    let dimensions = Dimensions::new(1280, 720);
    let orientation = Orientation::default();
    UIConfigInfoBuilder {
      screen_dimensions,
      dimensions,
      orientation,
    }
  }

  pub fn ndc_position(&self) -> Vec3 {
    let x = 2.0 * self.position.x / self.screen_dimensions.width as f32 - 1.0;
    let y = 1.0 - (2.0 * self.position.y as f32) / self.screen_dimensions.height as f32;
    let z = 0.0;

    Vec3::new(x, y, z)
  }

  ///Returns the element's bounding [`Rect`].
  /// During calculation, scales the `height` by the Window's aspect ratio to compensate for viewport distortion.
  pub fn rect(&self) -> Rect {
    let width = self.dimensions.width as f32 / Self::DPI as f32;
    let height = self.dimensions.height as f32 * self.screen_dimensions.aspect / Self::DPI as f32;
    Rect::new(width, height)
  }
}

pub struct UIConfigInfoBuilder {
  screen_dimensions: Dimensions,
  dimensions: Dimensions,
  orientation: Orientation,
}

impl UIConfigInfoBuilder {
  pub fn build(&self) -> UIConfigInfo {
    //these dimensions need to be the parent dimensions
    let dimensions = self.dimensions;
    let screen_dimensions = self.screen_dimensions;
    let orientation = self.orientation;

    //Calculate the position

    let mut position = Vec3::default();
    let horizontal_offset = orientation.horizontal_offset;
    let vertical_offset = orientation.vertical_offset;

    //Calculate the left and right values of the rectangle's corners in normalized device coordinates (range [-1,1]) using the vertical orientation
    match orientation.horizontal_anchor {
      HorizontalAnchor::Center => {
        position.x = (screen_dimensions.width as f32 / 2.0) + horizontal_offset;
      }
      HorizontalAnchor::Left => {
        position.x = horizontal_offset;
      }
      HorizontalAnchor::Right => {
        position.x = screen_dimensions.width as f32 + horizontal_offset;
      }
    }

    //Calculate the position of the widget in normalized device coordinates (range [-1,1])
    match orientation.vertical_anchor {
      VerticalAnchor::Center => {
        position.y = (screen_dimensions.height as f32 / 2.0) + vertical_offset;
      }
      VerticalAnchor::Top => {
        position.y = vertical_offset;
      }
      VerticalAnchor::Bottom => {
        position.y = screen_dimensions.height as f32 + vertical_offset;
      }
    }

    UIConfigInfo {
      screen_dimensions,
      dimensions,
      position,
    }
  }

  //functions to update the config info
  pub fn horizontal_anchor(&mut self, anchor: HorizontalAnchor) -> &mut Self {
    self.orientation.horizontal_anchor = anchor;
    self
  }
  // pub fn horizontal_offset(&mut self, offset: f32) {}
  pub fn vertical_anchor(&mut self, anchor: VerticalAnchor) -> &mut Self {
    self.orientation.vertical_anchor = anchor;
    self
  }
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
  // pub top: f32,
  // pub bottom: f32,
  // pub left: f32,
  // pub right: f32,
  min: Point2,
  max: Point2,
}

impl Rect {
  ///Create a new [`Rect`].
  pub fn new(width: f32, height: f32) -> Self {
    //Calculate the x and y maxima
    // let left = position.x - (width / 2.0);
    // let right = position.x + (width / 2.0);
    // let top = position.y - (height / 2.0);
    // let bottom = position.y + (height / 2.0);

    // let left = -(width / 2.0);
    // let right = (width / 2.0);
    // let top = -(height / 2.0);
    // let bottom = (height / 2.0);

    let x = width / 2.0;
    let y = height / 2.0;

    let min = Point2::new(-x, y);
    let max = Point2::new(x, -y);

    // Rect { top, bottom, left, right }
    Rect { min, max }
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
    // let right = value.rect.right;
    // let left = value.rect.left;
    // let top = value.rect.top;
    // let bottom = value.rect.bottom;
    let min = value.rect.min;
    let max = value.rect.max;
    let z = 0.0;

    //Create the widget's vertices
    let v1 = Vertex::new([max.x, max.y, z], [1.0, 1.0]); //Top Right
    let v2 = Vertex::new([max.x, min.y, z], [1.0, 0.0]); // Bottom Right
    let v4 = Vertex::new([min.x, max.y, z], [0.0, 1.0]); // Top Left
    let v3 = Vertex::new([min.x, min.y, z], [0.0, 0.0]); // Bottom Left

    // let v1 = Vertex::new([right, top, z], [1.0, 1.0]); //Top Right
    // let v2 = Vertex::new([right, bottom, z], [1.0, 0.0]); // Bottom Right
    // let v4 = Vertex::new([left, top, z], [0.0, 1.0]); // Top Left
    // let v3 = Vertex::new([left, bottom, z], [0.0, 0.0]); // Bottom Left

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
