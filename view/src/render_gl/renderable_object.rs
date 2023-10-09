extern crate cgmath;
use std::ptr::null;

use super::{
  Program,
  buffer::{ArrayBuffer,VertexArray, ElementArrayBuffer}, TexturedVertex, Texture,
};

use ecs::World;
use eyre::Result;
use gl::{Gl,TRIANGLES, UNSIGNED_INT, types::GLsizei}; 
use cgmath::Matrix4;

//right now this only renders the square
pub struct RenderableObject{
  shader_program:Program,
  transform: i32,
  _texture: Texture,
  _vbo: ArrayBuffer,
  vao:VertexArray,
  index_count: GLsizei
}
//add error wrapping 
impl RenderableObject{
  ///Builds a `RenderableObject` by taking in data and drawing it into the VBO, EBO and constructing the VAO.
  pub fn new(gl:&Gl,world:&World,name:&str) -> Result<RenderableObject>{
    let shader_program = Program::from_shader_files(&gl,world,name);
    
    let transform = shader_program.get_uniform_location("transform");
    
    //this actually loads the texture beforehand which feels like it might be faster I'm not sure I love it doing all the texture binding logic before I actually tell it to render
    let txt_path = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/wall.jpg";
    let texture = Texture::rgb_from_path(txt_path).with_mipmaps().load(gl)?;
    
    let vertices = [
                            // positions       // colors        // texture coords
      TexturedVertex::from((0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0)), // top right
      TexturedVertex::from((0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0)), // bottom right
      TexturedVertex::from((-0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0)), // bottom left
      TexturedVertex::from((-0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0))  // top left
    ];

    let indices = [
      0, 1, 3,  // first Triangle
      1, 2, 3   // second Triangle
    ];

    let vbo = ArrayBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    let ebo = ElementArrayBuffer::new(&gl);
    ebo.bind();
    ebo.static_draw_data(&indices);
    ebo.unbind();

    let vao: VertexArray = VertexArray::new(&gl);
    vao.bind();
    vbo.bind();
    ebo.bind();

    TexturedVertex::vertex_attrib_pointers(&gl);
    vao.unbind();
    vbo.unbind();
    ebo.unbind();
    

    Ok(RenderableObject { 
      shader_program,
      _texture:texture,
      transform,
      _vbo: vbo, 
      vao,
      index_count: indices.len() as i32
    })
  }

  pub fn render(&self, gl:&Gl, matrix: &Matrix4<f32>){
    self.shader_program.use_program();
    //should this be transformation matrix?

    self.shader_program.set_uniform_matrix4fv(
      self.transform, 
      &matrix);
   
    self.vao.bind();

    unsafe{
      gl.DrawElements(
        TRIANGLES,
        self.index_count,
        UNSIGNED_INT,
        null()
      );
    }
  }

}
#[cfg(test)]
mod tests{
    use std::path::PathBuf;

    use ecs::{World, ScreenDimensions};
    use gl::Gl;
    use sdl2::video::GLProfile::Core;
    use crate::render_gl::{Texture, ImageLoader};
    use image::io::Reader;
  
  #[test]
  fn load_text()-> eyre::Result<()>{  
    let mut world = World::new();
  
    world.add_resource().from_user_defined_data(ScreenDimensions::new(720,1280));
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
  
    //gl stuff to eventually crate
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(Core);
    gl_attr.set_context_version(3,3);
    
    let window = video_subsystem
      .window(
        "Project: Deux",
        world.immut_get_resource::<ScreenDimensions>().unwrap().width.try_into().unwrap(),
        world.immut_get_resource::<ScreenDimensions>().unwrap().height.try_into().unwrap())
      .opengl()
      .resizable()
      .position_centered()
      .build()
      .expect("could not initialize video subsystem");
  
    //gl stuff to eventually crate
    let _gl_context = window.gl_create_context().unwrap();
    let gl = Gl::load_with(&mut|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
  
    
    // struct Loader{}   
    // impl<'a> ImageLoader<'a> for Loader{}
    // impl Loader{
    //   fn new()->Self{
    //     Loader{}
    //   }
    // }
    
    // let new = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/wall.jpg";
    // let image = Reader::open(new)?;
    
    //let img = loader.image_from_path(root_path.join("wall.jpg").to_str().unwrap());
  
    //let texture = Texture::rgb_from_path(new).load(&gl)?;

    Ok(())
  }
}

