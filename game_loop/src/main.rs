extern crate gl;
extern crate nalgebra_glm as glm;
extern crate glfw;
extern crate engine;

use engine::{
  time::ServerTime, 
  ecs::{World, world_resources::ScreenDimensions}, 
  view::{
    render_gl::{Viewport, UncoloredTexturedVertex, RenderableObject, ColorBuffer, DepthBuffer}, 
    camera::Camera
  }, 
  input::user_inputs::FrameInputs,
};

use glfw::{Context, fail_on_errors, Key, Action};
use glm::vec3;
use gl::{Gl,DEPTH_TEST};
use eyre::Result;
use std::env;


fn main() -> Result<(), String>  {
  env::set_var("RUST_BACKTRACE", "FULL");
  let mut world = World::new();
  let mut server_time = ServerTime::new();
  
  world.add_resource().from_user_defined_data(ScreenDimensions::new(720,1280));
  
  //maybe I make events a or a component and query it? Might need to slap it in an RC if I want to pass it down to other functions
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  let (mut window,events) = glfw.create_window(
    world.immut_get_resource::<ScreenDimensions>().unwrap().width as u32, 
    world.immut_get_resource::<ScreenDimensions>().unwrap().height as u32, 
    "Project: Deux", 
    glfw::WindowMode::Windowed
  ).expect("Failed to create GLFW window.");

  window.make_current();
  window.set_all_polling(true);  //maybe just use the polling for specific keys and then poll the mouse separately

  // let sdl_context = sdl2::init()?;
  // let video_subsystem = sdl_context.video()?;
  
  //gl stuff to eventually crate
  // let gl_attr = video_subsystem.gl_attr();
  // gl_attr.set_context_profile(Core);
  // gl_attr.set_context_version(3,3);
  
  // let window = video_subsystem
  //   .window(
  //     "Project: Deux",
  //     world.immut_get_resource::<ScreenDimensions>().unwrap().width as u32,
  //     world.immut_get_resource::<ScreenDimensions>().unwrap().height as u32)
  //   .opengl()
  //   .resizable()
  //   .position_centered()
  //   .build()
  //   .expect("could not initialize video subsystem");

  // let _gl_context = window.gl_create_context().unwrap();
  
  let _gl_context = window.get_context_version();

  let gl = Gl::load_with(&mut|s| window.get_proc_address(s) as *const std::os::raw::c_void);
  unsafe{gl.Enable(DEPTH_TEST)}

  let mut viewport = Viewport::for_window(
    world.immut_get_resource::<ScreenDimensions>().unwrap().height,
    world.immut_get_resource::<ScreenDimensions>().unwrap().width
  );

  let vertices = vec![    
    //this plane is not rendering
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 0.0,)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  1.0, 0.0,)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 0.0)),
    
    //this plane is causing an error
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    
    ////this plane is fine
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  1.0, 0.0)),

    //// this plane is fine
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),

    //this plane is causing an error
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),

    //this plane is causing an error
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  0.0, 1.0))
  ];

  let cube_positions = vec![
      vec3( 0.0,  0.0,  0.0), 
      vec3( 1.5,  0.0,  0.0), 
      vec3( -1.5,  0.0,  0.0), 
      //Z positions      
      vec3( 0.0,  0.0,  1.5), 
      vec3( 0.0,  0.0,  -1.5), 
      vec3( 0.0,  0.0,  5.0), 
      vec3( 0.0,  0.0,  -5.0),
      // vec3( 2.0,  0.0, -15.0), 
      // vec3(-1.5, 0.0, -2.5),  
      // vec3(-3.8, 0.0, -12.3),  
      //// Vector3::from((2.4, -0.4, -3.5)),  
      //// Vector3::from((-1.7,  3.0, -7.5)),  
      //// Vector3::from((1.3, -2.0, -2.5)),  
      //// Vector3::from((1.5,  2.0, -2.5)), 
      //// Vector3::from((1.5,  0.2, -1.5)), 
      //// Vector3::from((-1.3,  1.0, -1.5))  
  ];
  
  //the structure of this resource will need to be adjusted once I need more than one texture
  world.add_resource().path_to_asset_folder_from_relative_exe_path("assets");
  
  let object = RenderableObject::new(&gl,&world,"textured",vertices).unwrap();
  
  let color_buffer = ColorBuffer::from_color(0.3,0.3,0.5,0.1);
  color_buffer.set_used(&gl);
  viewport.set_used(&gl);
  let aspect = world.immut_get_resource::<ScreenDimensions>().unwrap().aspect;

  let mut camera = Camera::new(aspect);
  
  world.add_resource().from_user_defined_data(FrameInputs::new());
  let frame_inputs = world.mut_get_resource::<FrameInputs>().unwrap();
  
  //main loop
  // let mut event_pump = sdl_context.event_pump()?;
  while !window.should_close() {
    server_time.tick();
    window.swap_buffers(); //this seems to fix the not responding problem but causes a weird flickering
    
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      println!("{:?}", event);
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
          window.set_should_close(true)
        },
        _ => {},
      }
    }
    // for event in event_pump.poll_iter() {
    //   match event {
    //     Event::Quit {..} |
    //     Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //       break 'game;
    //     },
    //     Event::Window {
    //       win_event: WindowEvent::Resized(w,h), ..} => {
    //         /*currently this does not update 
    //         the dimensions in world. 
    //         When I eventually create a camera system that will need to change*/
    //         viewport.update_size(w, h);
    //         viewport.set_used(&gl);
    //       },
    //       Event::KeyDown {keycode:Some(Keycode::Right), ..} =>{
    //         let event = UserInputs::MoveCameraRight;
    //         frame_inputs.add_event(event);
    //       },
    //       Event::KeyDown {keycode:Some(Keycode::Left), ..} =>{
    //         let event = UserInputs::MoveCameraLeft;
    //         frame_inputs.add_event(event)
    //       },
    //       //add the ability to use the scroll wheel to move
    //       Event::KeyDown {keycode:Some(Keycode::Up), ..} =>{
    //         let event = UserInputs::MoveCameraUp;
    //         frame_inputs.add_event(event)
    //       },
    //       Event::KeyDown {keycode:Some(Keycode::Down), ..} =>{
    //         let event = UserInputs::MoveCameraDown;
    //         frame_inputs.add_event(event)
    //       },
    //       Event::KeyDown{keycode:Some(Keycode::Space), ..} =>{
    //         let event = UserInputs::CenterCamera;
    //         frame_inputs.add_event(event)
    //       },
    //       Event::MouseWheel {y, ..} =>{
    //         if y > 0{
    //           let event = UserInputs::ZoomInCamera;
    //           frame_inputs.add_event(event)
    //         }
    //         else{
    //           let event = UserInputs::ZoomOutCamera;
    //           frame_inputs.add_event(event)
    //         }
    //       },
    //       // //need to add an event to track the mouse location 
    //       // //and update the camera based on it 
    //       // //I might need to break out glfw here since sdl does not track the z position  
    //       //I need to figure why this part of the input capture causes the program to lag out
    //       // Event::MouseMotion { x, y,..}=>{
    //       //   // while x <5{
    //       //   //   let event = UserInputs::MoveCameraLeft;
    //       //   //   frame_inputs.add_event(event);
    //       //   // }
    //       //   // while x > viewport.w-5{
    //       //   //   let event = UserInputs::MoveCameraRight;
    //       //   //   frame_inputs.add_event(event);
    //       //   // }
    //       //   // // else if  y < 5{
    //       //   //   let event = UserInputs::MoveCameraUp;
    //       //   //   frame_inputs.add_event(event);
    //       //   // }
    //       //   // else if y > viewport.h-5{
    //       //   //   let event = UserInputs::MoveCameraDown;
    //       //   //   frame_inputs.add_event(event);
    //       //   // }
    //       // }
    //       _ => {}
    //   }
    // }

    //Update
    if server_time.should_update()==true{
    let interpolation_factor = server_time.get_interpolation_factor();
    
    camera.new_position(frame_inputs);
    
    //my concern is that clearing the frame inputs means it won't update properly 
    //it will just lerp for one frame but not move the full amount
    //what I could do is instead of updating the position directly, I have the move command set a target position and then lerp between those two
    frame_inputs.clear();

    //I think this is where I update the delta timer
    server_time.decrimint_seconds_since_update()
    }
    
    //Render
    if server_time.should_render(){
      color_buffer.clear(&gl);
      DepthBuffer::clear(&gl);
    
      for position in &cube_positions{
        object.render(&gl, aspect, &camera, &position);
      }
      window.swap_buffers();
      server_time.decrimint_seconds_since_render()
    }
  } 
  Ok(())
}
