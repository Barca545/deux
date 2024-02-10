use gl::Gl;
use crate::view::{render_gl::Vertex, Mesh};
//rendering
pub struct SkinnedMesh{
  pub mesh:Mesh,
  pub scale_factor:f32
}

impl SkinnedMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str, scale_factor:f32) -> Self{
    SkinnedMesh{
      mesh:Mesh::new(gl, vertices, indices, texture_name).to_owned(),
      scale_factor
    }
  }
}

impl From<AutoAttackMesh> for SkinnedMesh{
  fn from(value: AutoAttackMesh) -> Self {
    let mesh = value.mesh;
    let scale_factor = value.scale_factor;
    SkinnedMesh{
      mesh,
      scale_factor
    }
  }
}

#[derive(Debug, Clone)]
pub struct AutoAttackMesh{
  pub mesh:Mesh,
  pub scale_factor:f32
}
impl AutoAttackMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str, scale_factor:f32) -> Self{
    AutoAttackMesh{
      mesh:Mesh::new(gl, vertices, indices, texture_name).to_owned(),
      scale_factor
    }
  }
}

pub struct StaticMesh(pub Mesh);
impl StaticMesh{
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str) -> Self{
    StaticMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}