extern crate nalgebra_glm as glm;

use crate::{
  ecs::World,
  math::{Renderable, Transforms, math::Vec3},
};

use super::{
  Program,
  buffer::{ArrayBuffer,VertexArray}, 
  UncoloredTexturedVertex,
  Texture,    
};

use eyre::Result;
use gl::{Gl,TRIANGLES, types::GLint};

//right now this only renders the square
pub struct RenderableObject{
  vertices: Vec<UncoloredTexturedVertex>, //really this should be something that impls the vertex trait
  shader_program:Program,
  model_uniform_loc:GLint,
  view_uniform_loc:GLint,
  projection_uniform_loc:GLint,
  _texture: Texture,
  // _vbo: ArrayBuffer,
  // vao:VertexArray,
  // index_count: GLsizei,
}
//add error wrapping 
impl RenderableObject{
/**
Builds a `RenderableObject` by taking in data and drawing it into the VBO, 
EBO and constructing the VAO.
*/
  pub fn new(gl:&Gl,world:&World,name:&str,vertices:Vec<UncoloredTexturedVertex>,texture_path:&str) -> Result<RenderableObject>{
    let shader_program = Program::from_shader_files(&gl,world,name);
    
    //let transform_uniform_loc = shader_program.get_uniform_location("transform");
    let model_uniform_loc = shader_program.get_uniform_location("model");
    let view_uniform_loc = shader_program.get_uniform_location("view");
    let projection_uniform_loc = shader_program.get_uniform_location("projection");
    
    //this actually loads the texture beforehand which feels like it might be faster I'm not sure I love it doing all the texture binding logic before I actually tell it to render
    //texture is being bound but not unbound (I think, so I'm only able to use one at a time)
    //issue seems to be that the from path function binds the texture 
    //I think I just want to load the texture into memory
    //generate the texture object and store those then when rendering use each as appropriate
    let path = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned() + texture_path;
    let texture = Texture::rgb_from_path(path.as_str()).with_mipmaps().load(gl)?;
    
    // let indices = [
    //   0, 1, 3,  // first Triangle
    //   1, 2, 3   // second Triangle
    // ];

    Ok(RenderableObject { 
      vertices,
      shader_program,
      _texture:texture,
      model_uniform_loc,
      view_uniform_loc,
      projection_uniform_loc,
      // _vbo: vbo, 
      // vao,
      // index_count: indices.len() as i32,
    })
  }
}

impl Renderable for RenderableObject{
  ///render should be a separate system not a method
  fn render(&self, gl:&Gl,transforms:&Transforms,position:&Vec3){
    let vbo = ArrayBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&self.vertices);
    vbo.unbind();

    //Am I using the ebo, can I get rid of this?
    // let ebo = ElementArrayBuffer::new(&gl);
    // ebo.bind();
    // ebo.static_draw_data(&indices);
    // ebo.unbind();

    let vao: VertexArray = VertexArray::new(&gl);
    vao.bind();
    vbo.bind();
    // ebo.bind();

    UncoloredTexturedVertex::vertex_attrib_pointers(&gl);
    vao.unbind();
    vbo.unbind();
    // ebo.unbind();
    
    //should all the shader stuff be wrapped into a method on the struct?
    self.shader_program.use_program();

    //bind the model transform
    self.shader_program.set_uniform_matrix4fv(
      self.model_uniform_loc, 
      &transforms.get_model_transform(position)
    );

    //bind the view transform
    self.shader_program.set_uniform_matrix4fv(
      self.view_uniform_loc, 
      &transforms.get_view_transform()
    );

    //bind the projection transform
    self.shader_program.set_uniform_matrix4fv(
      self.projection_uniform_loc, 
      transforms.get_projection_transform().as_matrix()
    );
  
    vao.bind();

    unsafe{
      gl.DrawArrays(
        TRIANGLES,
        0,
        36,
      );
    }
  }
}