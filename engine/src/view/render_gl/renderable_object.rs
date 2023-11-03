extern crate nalgebra_glm as glm;

use crate::{
  ecs::World,
  math::math::radians, view::camera::Camera
};

use super::{
  Program,
  buffer::{ArrayBuffer,VertexArray}, 
  UncoloredTexturedVertex,
  Texture,    
};

use eyre::Result;
use gl::{Gl,TRIANGLES}; 
use glm::{vec3,TVec3,TMat4,perspective,rotate,identity,translate};

//right now this only renders the square
pub struct RenderableObject{
  vertices: Vec<UncoloredTexturedVertex>, //really this should be something that impls the vertex trait
  shader_program:Program,
  // transform_uniform_loc: i32,
  model_uniform_loc: i32,
  view_uniform_loc: i32,
  projection_uniform_loc:i32,
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
  pub fn new(gl:&Gl,world:&World,name:&str,vertices:Vec<UncoloredTexturedVertex>) -> Result<RenderableObject>{
    let shader_program = Program::from_shader_files(&gl,world,name);
    
    //let transform_uniform_loc = shader_program.get_uniform_location("transform");
    let model_uniform_loc = shader_program.get_uniform_location("model");
    let view_uniform_loc = shader_program.get_uniform_location("view");
    let projection_uniform_loc = shader_program.get_uniform_location("projection");
    
    //this actually loads the texture beforehand which feels like it might be faster I'm not sure I love it doing all the texture binding logic before I actually tell it to render
    let txt_path = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/wall.jpg";
    let texture = Texture::rgb_from_path(txt_path).with_mipmaps().load(gl)?;
    
    // let indices = [
    //   0, 1, 3,  // first Triangle
    //   1, 2, 3   // second Triangle
    // ];

    Ok(RenderableObject { 
      vertices,
      shader_program,
      _texture:texture,
      // transform_uniform_loc,
      model_uniform_loc,
      view_uniform_loc,
      projection_uniform_loc,
      // _vbo: vbo, 
      // vao,
      // index_count: indices.len() as i32,
    })
  }

  fn model_transform(&self,position_vec:&TVec3<f32>)->TMat4<f32>{
    //there is some issue with this 
    let identity =  identity::<f32,4>();
    let position = translate(&identity, position_vec);
    let axis:TVec3<f32> = vec3(1.0,0.0, 0.0);
    let model = rotate(&position, radians(0.0), &axis);
    model
  }

  fn projection_transform(&self,aspect:f32)->TMat4<f32>{
    let projection = perspective(aspect, radians(45.0), 0.1, 100.0);
    projection
  }

  pub fn render(&self, gl:&Gl,aspect:f32,camera:&Camera,position:&TVec3<f32>){
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
    
    self.shader_program.use_program();
    
    //I think I want to abstract at least the projection into the camera class?

    //bind the model transform
    self.shader_program.set_uniform_matrix4fv(
      self.model_uniform_loc, 
      &self.model_transform(position)
    );

    //bind the view transform
    self.shader_program.set_uniform_matrix4fv(
      self.view_uniform_loc, 
      &camera.camera_view()
    );

    //bind the projection transform
    self.shader_program.set_uniform_matrix4fv(
      self.projection_uniform_loc, 
      //program did not like the initial value for aspect ratio
      &self.projection_transform(aspect)
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