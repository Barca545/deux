use crate::view::{render_gl::Vertex, Material, Mesh, RenderPass, RenderStage, RenderStageName};
use gl::Gl;

pub struct SkinnedMesh {
  pub mesh: Mesh,
  pub scale_factor: f32,
}

impl SkinnedMesh {
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str, scale_factor: f32) -> Self {
    let pass = RenderPass::new(&gl)
      .with_vert("CharacterVertexShader")
      .unwrap()
      .with_frag("CharacterFragShader")
      .unwrap()
      .build()
      .unwrap();

    let mut stage = RenderStage::new(RenderStageName::SkinnedMesh);
    stage.add_pass(pass);

    let mut material = Material::new();
    material.add_sampler(&gl, texture_name).unwrap().add_stage(stage);

    let mesh = Mesh::new(&gl, vertices, indices).with_material(material).build();

    match mesh {
      Ok(mesh) => SkinnedMesh { mesh, scale_factor },
      Err(_) => unreachable!(),
    }
  }
}

impl From<AutoAttackMesh> for SkinnedMesh {
  fn from(value: AutoAttackMesh) -> Self {
    let mesh = value.mesh;
    let scale_factor = value.scale_factor;
    SkinnedMesh { mesh, scale_factor }
  }
}

#[derive(Debug, Clone)]
pub struct AutoAttackMesh {
  pub mesh: Mesh,
  pub scale_factor: f32,
}
impl AutoAttackMesh {
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str, scale_factor: f32) -> Self {
    let pass = RenderPass::new(&gl)
      .with_vert("CharacterVertexShader")
      .unwrap()
      .with_frag("CharacterFragShader")
      .unwrap()
      .build()
      .unwrap();

    let mut stage = RenderStage::new(RenderStageName::SkinnedMesh);
    stage.add_pass(pass);

    let mut material = Material::new();
    material.add_sampler(&gl, texture_name).unwrap().add_stage(stage);

    let mesh = Mesh::new(&gl, vertices, indices).with_material(material).build().unwrap();
    AutoAttackMesh { mesh, scale_factor }
  }
}

pub struct StaticMesh(pub Mesh);
impl StaticMesh {
  pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, texture_name: &str) -> Self {
    let pass = RenderPass::new(&gl)
      .with_vert("CharacterVertexShader")
      .unwrap()
      .with_frag("CharacterFragShader")
      .unwrap()
      .build()
      .unwrap();

    let mut technique = RenderStage::new(RenderStageName::StaticMesh);
    technique.add_pass(pass);

    let mut material = Material::new();
    material.add_sampler(&gl, texture_name).unwrap().add_stage(technique);

    let mesh = Mesh::new(&gl, vertices, indices).with_material(material).build().unwrap();
    StaticMesh(mesh)
  }
}
