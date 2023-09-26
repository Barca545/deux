extern crate gl;
extern crate view;

use view::render_gl::{vertex::Vertex,shader::{Shader,Program}};
use gl::Gl;
use eyre::Result;
use std::time::Duration;
use ecs::{World,ScreenHeight,ScreenWidth};
use sdl2::{
  event::Event,
  keyboard::Keycode,
  // rect::{Point, Rect},
  // image::LoadTexture,
  // mouse::MouseButton::Right, video::gl_attr,
  video::GLProfile::Core
};

//move with gl
use gl::types::{GLuint,GLsizeiptr,GLvoid};
use std::mem::size_of;

fn main() -> Result<(), String>  {
  let mut world = World::new();
  
  world.add_resource().from_user_defined_data(ScreenHeight(720));
  world.add_resource().from_user_defined_data(ScreenWidth(1280));
  let screen_dimensions= [world.immut_get_resource::<ScreenWidth>().unwrap().0,world.immut_get_resource::<ScreenHeight>().unwrap().0];
  
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  //gl stuff to eventually crate
  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(Core);
  gl_attr.set_context_version(3,3);

  let window = video_subsystem
    .window("deux",screen_dimensions[0] , screen_dimensions[1])
    .opengl()
    .resizable()
    .position_centered()
    .build()
    .expect("could not initialize video subsystem");

  //gl stuff to eventually crate
  let _gl_context = window.gl_create_context().unwrap();
  let gl = Gl::load_with(&mut|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

  //gl stuff to eventually crate, possibly
  unsafe {
    gl.Viewport(0, 0, screen_dimensions[0].try_into().unwrap(), screen_dimensions[1].try_into().unwrap());
    gl.ClearColor(0.3, 0.3, 0.5, 1.0);
  }
  
  //going to error because these files do not exist yet
  world.add_resource().folder_from_relative_exe_path("assets");
  let vert = &world.load_resource_from_cstring("triangle.vert").unwrap();
  let frag = &world.load_resource_from_cstring("triangle.frag").unwrap();

  let vert_shader = Shader::from_vertex_source(&gl,vert).unwrap();
  let frag_shader = Shader::from_fragment_source(&gl,frag).unwrap();
  let shader_program = Program::from_shaders(&gl,&[vert_shader,frag_shader]).unwrap();
  shader_program.use_program();

  let mut vbo:GLuint = 0;
  unsafe{gl.GenBuffers(1,&mut vbo)}

  let vertices: Vec<Vertex> = vec![
    Vertex { pos: (0.5, -0.5, 0.0).into(),  clr: (1.0, 0.0, 0.0).into() }, // bottom right
    Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
    Vertex { pos: (0.0,  0.5, 0.0).into(),  clr: (0.0, 0.0, 1.0).into() }  // top
  ];

  unsafe{
    gl.BindBuffer(gl::ARRAY_BUFFER,vbo);
    gl.BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * size_of::<Vertex>()) as GLsizeiptr,
      vertices.as_ptr() as *const GLvoid,
      gl::STATIC_DRAW,
    );
    gl.BindBuffer(gl::ARRAY_BUFFER,0);
  }
  
  let mut vao: GLuint = 0;
  unsafe{
    gl.GenVertexArrays(1,&mut vao)
  }

  //if all these index variables are the same, should index be a variable?
  unsafe{
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER,vbo);
  }

  Vertex::vertex_attrib_pointers(&gl);
  
  let mut event_pump = sdl_context.event_pump()?;
  'game: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'game;
        },
        _ => {}
      }
    }
    //Update
    
    //Render
    
    //gl stuff to eventually crate, possibly
    unsafe {
      gl.Clear(gl::COLOR_BUFFER_BIT);
    }
    
    shader_program.use_program();
    unsafe{
      gl.BindVertexArray(vao);
      gl.DrawArrays(
        gl::TRIANGLES,
        0,
        3,
      );
    }
    window.gl_swap_window();
    
    //I am debating increasing the amount of ticks but meh LoL runs on like 30  
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  } 
  Ok(())
}
