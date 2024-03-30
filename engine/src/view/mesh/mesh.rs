use super::{Material, RenderStageName};
use crate::{
  errors::MeshErrors,
  math::Vec3,
  physics::AABB3D,
  view::render_gl::{
    buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray},
    draw_indexed_primative, Texture, UntexturedVertex, Vertex,
  },
};
use eyre::Result;
use gl::{Gl, STATIC_DRAW};

// Refactor:
// -Should I be unbinding the VBO, VAO, and EBO in the init_mesh functions?
//  This example does not unbind, https://learnopengl.com/Model-Loading/Mesh.
//  This example does unbind, https://gamedev.stackexchange.com/questions/90471/should-unbind-buffers.
// -I don't think the mesh needs to hold onto the verts after buffering them
// -Debug mesh can just use a red texture + normal Vertexes then get rid of untextured vertex, tho I would need indices and text coords...
// -Refactor the draw_stage so it doesn't change the program each time/see if this is even a problem
//  Easiest way to do that is probably just to move the passes off the material and define them elsewhere
//  Like in some config step at the begining of the game then store them as a resource

#[derive(Debug, Clone)]
pub struct Mesh {
  // pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
  // pub texture: Texture,
  pub material: Material,
  pub vao: VertexArray,
  pub vbo: ArrayBuffer,
  pub ebo: ElementArrayBuffer,
}

impl Mesh {
  pub fn new<'b>(gl: &'b Gl, vertices: Vec<Vertex>, indices: Vec<u32>) -> MeshBuilder<'b> {
    MeshBuilder::new(gl, vertices, indices)
  }

  ///Draws the specified [`RenderStage`].
  pub fn draw_stage(&self, gl: &Gl, stage: RenderStageName) {
    for pass in &self.material.stages.get(&stage).unwrap().passes {
      pass.init(gl);

      draw_indexed_primative(gl, self);
    }
  }

  pub fn texture(&self) -> Texture {
    self.material.samplers[0].texture
  }
}

pub struct MeshBuilder<'b> {
  gl: &'b Gl,
  vertices: Vec<Vertex>,
  indices: Vec<u32>,
  material: Option<Material>,
}

impl<'b> MeshBuilder<'b> {
  fn new(gl: &'b Gl, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
    MeshBuilder {
      gl,
      vertices,
      indices,
      material: None,
    }
  }

  pub fn with_material(&mut self, material: Material) -> &mut Self {
    self.material = Some(material);
    self
  }

  pub fn build(&self) -> Result<Mesh> {
    let vao = VertexArray::new(self.gl);
    let vbo = ArrayBuffer::new(self.gl);
    let ebo = ElementArrayBuffer::new(self.gl);

    // vbo.bind();
    // vbo.buffer_data(&vertices, STATIC_DRAW);
    // vbo.unbind();

    // vao.bind();
    // vbo.bind();
    // Vertex::init_attrib_pointers(&gl);

    // vao.unbind();
    // vbo.unbind();
    // vao

    vao.bind(self.gl);

    vbo.bind(self.gl);
    vbo.buffer_data(self.gl, &self.vertices, STATIC_DRAW);

    ebo.bind(self.gl);
    ebo.buffer_data(self.gl, &self.indices, STATIC_DRAW);

    Vertex::init_attrib_pointers(self.gl);

    vbo.unbind(self.gl);
    ebo.unbind(self.gl);

    vao.unbind(self.gl);

    match &self.material {
      Some(material) => {
        let mesh = Mesh {
          material: material.clone(),
          indices: self.indices.clone(),
          ebo,
          vao,
          vbo,
        };
        Ok(mesh)
      }
      None => return Err(MeshErrors::NoRegisteredMaterial.into()),
    }
  }
}

pub struct AABB3DDebugMesh {
  pub vao: VertexArray,
}

impl AABB3DDebugMesh {
  pub fn new(gl: &Gl, aabb: AABB3D, position: Vec3) -> Self {
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

  fn init_mesh(gl: &Gl, vertices: &Vec<UntexturedVertex>) -> VertexArray {
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

    vao.bind(gl);
    vbo.bind(gl);
    vbo.buffer_data(gl, &vertices, STATIC_DRAW);
    UntexturedVertex::init_attrib_pointers(&gl);
    vbo.unbind(gl);
    vao.unbind(gl);

    vao
  }
}
