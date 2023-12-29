use crate::{
  math::Vec3,
  physics::AABB3D,
  view::render_gl::{
    buffer::{ArrayBuffer, VertexArray},
    Texture, Vertex
  }
};
use gl::{Gl, STATIC_DRAW};

use super::render_gl::{UntexturedVertex, buffer::ElementArrayBuffer};

#[derive(Debug, Clone)]
pub struct MeshInfo{ 
  pub vertices:Vec<Vertex>,
  pub indices:Vec<u32>,
  pub texture_name:String
}


//let's just have the one mesh structure and then wrap it in skinned mesh/static mesh etc
pub struct Mesh {
  pub vertices:Vec<Vertex>,
  pub indices:Vec<u32>,
  pub texture:Texture,
  pub vao:VertexArray,
  pub vbo:ArrayBuffer,
  pub ebo:ElementArrayBuffer
}

impl Mesh {
  pub fn new(gl:&Gl, vertices:Vec<Vertex>, indices:Vec<u32>, texture_name:&str) -> Self {
    let texture = Texture::new(gl, texture_name).unwrap();
    let (vao, vbo, ebo) = Self::init_mesh(gl, &vertices, &indices);
  
    Mesh {vertices, indices, texture, vao, vbo, ebo }
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>, indices:&Vec<u32>,) -> (VertexArray, ArrayBuffer, ElementArrayBuffer) {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);
    let ebo = ElementArrayBuffer::new(&gl);

    // vbo.bind();
    // vbo.buffer_data(&vertices, STATIC_DRAW);
    // vbo.unbind();

    // vao.bind();
    // vbo.bind();
    // Vertex::init_attrib_pointers(&gl);

    // vao.unbind();
    // vbo.unbind();
    // vao

    vao.bind(); 
    
    vbo.bind();
    vbo.buffer_data(&vertices, STATIC_DRAW);
    
    ebo.bind();
    ebo.buffer_data(&indices, STATIC_DRAW);
    
    Vertex::init_attrib_pointers(&gl);
    
    //am I not supposed to unbind these?
    //https://learnopengl.com/Model-Loading/Mesh does not seem to unbind but I am unclear if that is an oversight
    //https://gamedev.stackexchange.com/questions/90471/should-unbind-buffers this disagrees
    vbo.unbind();
    ebo.unbind();
    
    vao.unbind();
    
    (vao,vbo,ebo)
  }
}

pub struct AABB3DDebugMesh {
  pub vao:VertexArray
}

impl AABB3DDebugMesh {
  pub fn new(gl:&Gl, aabb:AABB3D, position:Vec3) -> Self {
    //will need to refactor the vertex struct for non-textured rendering
    //need to add the position of the object

    let aabb_min_x = aabb.min.x - position.x;
    let aabb_max_x = aabb.max.x - position.x;

    let aabb_min_y = aabb.min.y - position.y;
    let aabb_max_y = aabb.max.y - position.y;

    let aabb_min_z = aabb.min.z - position.z;
    let aabb_max_z = aabb.max.z - position.z;

    let vertices = vec![
      UntexturedVertex::from((aabb_min_x, aabb_min_y, aabb_max_z)),
      UntexturedVertex::from((aabb_max_x, aabb_min_y, aabb_max_z)),
      UntexturedVertex::from((aabb_max_x, aabb_max_y, aabb_max_z)),
      UntexturedVertex::from((aabb_min_x, aabb_max_y, aabb_max_z)),
      UntexturedVertex::from((aabb_max_x, aabb_min_y, aabb_max_z)),
      UntexturedVertex::from((aabb_max_x, aabb_min_y, aabb_min_z)),
      UntexturedVertex::from((aabb_max_x, aabb_max_y, aabb_min_z)),
      UntexturedVertex::from((aabb_max_x, aabb_max_y, aabb_max_z)),
      UntexturedVertex::from((aabb_min_x, aabb_max_y, aabb_max_z)),
      UntexturedVertex::from((aabb_max_x, aabb_max_y, aabb_max_z)),
      UntexturedVertex::from((aabb_max_x, aabb_max_y, aabb_min_z)),
      UntexturedVertex::from((aabb_min_x, aabb_max_y, aabb_min_z)),
      UntexturedVertex::from((aabb_min_x, aabb_min_y, aabb_min_z)),
      UntexturedVertex::from((aabb_min_x, aabb_max_y, aabb_min_z)),
      UntexturedVertex::from((aabb_max_x, aabb_max_y, aabb_min_z)),
      UntexturedVertex::from((aabb_max_x, aabb_min_y, aabb_min_z)),
      UntexturedVertex::from((aabb_min_x, aabb_min_y, aabb_min_z)),
      UntexturedVertex::from((aabb_max_x, aabb_min_y, aabb_min_z)),
      UntexturedVertex::from((aabb_max_x, aabb_min_y, aabb_max_z)),
      UntexturedVertex::from((aabb_min_x, aabb_min_y, aabb_max_z)),
      UntexturedVertex::from((aabb_min_x, aabb_min_y, aabb_min_z)),
      UntexturedVertex::from((aabb_min_x, aabb_min_y, aabb_max_z)),
      UntexturedVertex::from((aabb_min_x, aabb_max_y, aabb_max_z)),
      UntexturedVertex::from((aabb_min_x, aabb_max_y, aabb_min_z)),
    ];
    let vao = Self::init_mesh(gl, &vertices);
    AABB3DDebugMesh { vao }
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<UntexturedVertex>) -> VertexArray {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);

    // vbo.bind();
    // vbo.buffer_data(&vertices, STATIC_DRAW);
    // vbo.unbind();

    // vao.bind();
    // vbo.bind();
    // UntexturedVertex::init_attrib_pointers(&gl);

    // vao.unbind();
    // vbo.unbind();
    // vao

    vao.bind(); 
    vbo.bind();
    vbo.buffer_data(&vertices, STATIC_DRAW);
    UntexturedVertex::init_attrib_pointers(&gl);
    vbo.unbind();
    vao.unbind();
    
    vao
  }
}
