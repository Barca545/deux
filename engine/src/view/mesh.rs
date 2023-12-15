use crate::{view::render_gl::{
  buffer::{ArrayBuffer, VertexArray},
  Texture, Vertex
}, physics::AABB3D};
use gl::Gl;

use super::render_gl::DebugVertex;

//try making mesh as trait
//split into crates
//make mesh only public to crate and then make the components pub generally

pub struct SkinnedMesh {
  pub texture:Texture,
  pub vao:VertexArray
}

//this needs worlds because I made load resource from cstring a method on
// world, undo this and just make load from c_string a separate loader function
// verticies is the data stored in model. I do not think anything else needs the
// vertivies model may not need to be a distinct component

impl SkinnedMesh {
  pub fn new(gl:&Gl, texture_name:&str, vertices:Vec<Vertex>) -> Self {
    let texture = Texture::new(gl, texture_name).unwrap();
    let vao = Self::init_mesh(gl, &vertices);

    SkinnedMesh {
      texture,
      vao
    }
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>) -> VertexArray {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);

    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    vao.bind();
    vbo.bind();
    //this defines the vertex attribute pointers for position and color
    Vertex::init_attrib_pointers(&gl);

    vao.unbind();
    vbo.unbind();
    vao
  }
}


pub struct StaticMesh {
  pub texture:Texture,
  pub vao:VertexArray
}

//this needs worlds because I made load resource from cstring a method on
// world, undo this and just make load from c_string a separate loader function
// verticies is the data stored in model. I do not think anything else needs the
// vertivies model may not need to be a distinct component
impl StaticMesh {
  pub fn new(gl:&Gl, texture_name:&str, vertices:Vec<Vertex>) -> Self {
    let texture = Texture::new(gl, texture_name).unwrap();
    let vao = Self::init_mesh(gl, &vertices);

    StaticMesh {
      texture,
      vao
    }
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>) -> VertexArray {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);

    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    vao.bind();
    vbo.bind();
    //this defines the vertex attribute pointers for position and color
    Vertex::init_attrib_pointers(&gl);

    vao.unbind();
    vbo.unbind();
    vao
  }
}


pub struct AABB3DDebugMesh{
  pub vao:VertexArray
}

impl AABB3DDebugMesh {
  pub fn new(gl:&Gl, aabb:AABB3D) -> Self {
    //will need to refactor the vertex struct for non-textured rendering
    let vertices = vec![
      DebugVertex::from((aabb.min.x, aabb.min.y, aabb.max.z)),
      DebugVertex::from((aabb.max.x, aabb.min.y, aabb.max.z)),
      DebugVertex::from((aabb.max.x, aabb.max.y, aabb.max.z)),
      DebugVertex::from((aabb.min.x, aabb.max.y, aabb.max.z)),

      DebugVertex::from((aabb.max.x, aabb.min.y, aabb.max.z)),
      DebugVertex::from((aabb.max.x, aabb.min.y, aabb.min.z)),
      DebugVertex::from((aabb.max.x, aabb.max.y, aabb.min.z)),
      DebugVertex::from((aabb.max.x, aabb.max.y, aabb.max.z)),

      DebugVertex::from((aabb.min.x, aabb.max.y, aabb.max.z)),
      DebugVertex::from((aabb.max.x, aabb.max.y, aabb.max.z)),
      DebugVertex::from((aabb.max.x, aabb.max.y, aabb.min.z)),
      DebugVertex::from((aabb.min.x, aabb.max.y, aabb.min.z)),

      DebugVertex::from((aabb.min.x, aabb.min.y, aabb.min.z)),
      DebugVertex::from((aabb.min.x, aabb.max.y, aabb.min.z)),
      DebugVertex::from((aabb.max.x, aabb.max.y, aabb.min.z)),
      DebugVertex::from((aabb.max.x, aabb.min.y, aabb.min.z)),

      DebugVertex::from((aabb.min.x, aabb.min.y, aabb.min.z)),
      DebugVertex::from((aabb.max.x, aabb.min.y, aabb.min.z)),
      DebugVertex::from((aabb.max.x, aabb.min.y, aabb.max.z)),
      DebugVertex::from((aabb.min.x, aabb.min.y, aabb.max.z)),

      DebugVertex::from((aabb.min.x, aabb.min.y, aabb.min.z)),
      DebugVertex::from((aabb.min.x, aabb.min.y, aabb.max.z)),
      DebugVertex::from((aabb.min.x, aabb.max.y, aabb.max.z)),
      DebugVertex::from((aabb.min.x, aabb.max.y, aabb.min.z)),
    ];
    let vao = Self::init_mesh(gl, &vertices);
    AABB3DDebugMesh{vao}
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<DebugVertex>) -> VertexArray {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);

    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    vao.bind();
    vbo.bind();
    //this defines the vertex attribute pointers for position and color
    DebugVertex::init_attrib_pointers(&gl);

    vao.unbind();
    vbo.unbind();
    vao
  }
}