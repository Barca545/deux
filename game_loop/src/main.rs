extern crate gl;
extern crate view;
extern crate cgmath;

use view::render_gl::{Shader,Viewport,RenderableObject,ColorBuffer};
use gl::{Gl,types::GLchar};
use eyre::Result;
use std::{time::Duration, thread::sleep};
use ecs::{World,ScreenDimensions};
use sdl2::{
  self,
  event::{Event,WindowEvent},
  keyboard::Keycode,
  video::GLProfile::Core
};
use cgmath::{Matrix4,vec3, SquareMatrix,Rad};

fn main() -> Result<(), String>  {
  let mut world = World::new();
  
  world.add_resource().from_user_defined_data(ScreenDimensions::new(720,1280));
  
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  
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

  //gl stuff to eventually crate, possibly
  let mut viewport = Viewport::for_window(
    world.immut_get_resource::<ScreenDimensions>().unwrap().height,
    world.immut_get_resource::<ScreenDimensions>().unwrap().width
  );

  //the structure of this resource will need to be adjusted once I add more than one object
  world.add_resource().path_to_asset_folder_from_relative_exe_path("assets");
  let object = RenderableObject::new(&gl,&world,"textured").unwrap();
  
  let color_buffer = ColorBuffer::from_color(0.3,0.3,0.5,0.1);
  color_buffer.set_used(&gl);

  viewport.set_used(&gl);
  
  let mut rotate = 0.0;

  // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
  //https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/4.1.textures/textures.cpp
  // glfwSwapBuffers(window);
  // glfwPollEvents();

  //main loop
  let mut event_pump = sdl_context.event_pump()?;
  'game: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'game;
        },
        Event::Window {
          win_event: WindowEvent::Resized(w,h), ..} => {
            /*currently this does not update 
            the dimensions in world. 
            When I eventually create a camera system that will need to change*/
            viewport.update_size(w, h);
            viewport.set_used(&gl);
          }
        _ => {}
      }
    }
    //Update
    //I think this is where all the systems go

    //Render
    rotate+=0.017 as f32; //delete the rotate declaration when I finalize the render code

    let mut transform:Matrix4<f32> = Matrix4::identity();
    transform =  transform * Matrix4::<f32>::from_translation(vec3(0.3, 0.3, 0.3));
    transform = transform * Matrix4::<f32>::from_angle_z(Rad(rotate));
    
    //move to render system
    color_buffer.clear(&gl);
    object.render(&gl,&transform);
    window.gl_swap_window();
    
    //I am debating increasing the amount of ticks but meh LoL runs on like 30 so 60 might already be overkill
    sleep(Duration::new(0, 1_000_000_000u32 / 60));
  } 
  Ok(())
}
