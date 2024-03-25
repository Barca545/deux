use crate::{
  math::Rect,
  view::{render_gl::Vertex, Mesh},
};
use gl::Gl;

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
    let min = value.rect.min;
    let max = value.rect.max;
    let z = 0.0;

    //Create the widget's vertices
    let v1 = Vertex::new([max.x, max.y, z], [1.0, 1.0]); //Top Right
    let v2 = Vertex::new([max.x, min.y, z], [1.0, 0.0]); // Bottom Right
    let v4 = Vertex::new([min.x, max.y, z], [0.0, 1.0]); // Top Left
    let v3 = Vertex::new([min.x, min.y, z], [0.0, 0.0]); // Bottom Left

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
