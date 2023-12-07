extern crate gl;
extern crate nalgebra_glm as glm;
extern crate glfw;
extern crate engine;

use engine::{
  time::ServerTime, 
  ecs::{
    World, 
    world_resources::ScreenDimensions,
    component_lib::{Model, Position}
  }, 
  view::{
    render_gl::{Viewport, UncoloredTexturedVertex, RenderableObject, ColorBuffer, DepthBuffer}, 
    camera::Camera
  }, 
  input::user_inputs::{FrameInputs, MousePosition}, math::{Renderable, Transforms, math::Vec3, MouseRay},
};

use glfw::{Context, fail_on_errors, Key, Action, MouseButton};
use glm::vec3;
use gl::{Gl,DEPTH_TEST};
use eyre::Result;
use std::env;


fn main() -> Result<()>  {
  env::set_var("RUST_BACKTRACE", "FULL");
  let mut world = World::new();
  let mut server_time = ServerTime::new();
  
  world.add_resource().from_user_defined_data(ScreenDimensions::new(720,1280));
  // world.add_resource().from_user_defined_data(MousePicker::new());

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
  
  let _gl_context = window.get_context_version();

  let gl = Gl::load_with(&mut|s| window.get_proc_address(s) as *const std::os::raw::c_void);
  unsafe{gl.Enable(DEPTH_TEST)}

  let viewport = Viewport::for_window(
    world.immut_get_resource::<ScreenDimensions>().unwrap().height,
    world.immut_get_resource::<ScreenDimensions>().unwrap().width
  );
  world
    .register_component::<Model>()
    .register_component::<Position>();

  
  let vertices = vec![    
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 0.0,)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  1.0, 0.0,)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 0.0)),
    
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  1.0, 0.0)),

    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),

    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5, -0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5, -0.5, -0.5,  0.0, 1.0)),

    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  0.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5, -0.5,  1.0, 1.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((0.5,  0.5,  0.5,  1.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5,  0.5,  0.0, 0.0)),
    UncoloredTexturedVertex::from((-0.5,  0.5, -0.5,  0.0, 1.0))
  ];

  let model = Model(vertices);
  
  let positions:Vec<Vec3> =vec![
    vec3( 3.0,  0.0,  0.0),
    vec3( -3.0,  0.0,  0.0),
    vec3( 0.0,  0.0,  3.0),
    vec3(0.0, 0.0, -3.0)
  ];

  let mut player_position:Vec3 = vec3( 0.0,  0.0,  0.0);
  
  //the structure of this resource will need to be adjusted once I need more than one texture
  world.add_resource().path_to_asset_folder_from_relative_exe_path("assets");
  
  //instead of giving it ownership of verticies maybe just make this a "Renderer" 
  //or something and have it take in a refrence to the model component
  
  let color_buffer = ColorBuffer::from_color(0.3,0.3,0.5,0.1);
  color_buffer.set_used(&gl);
  viewport.set_used(&gl);
  
  let aspect = world.immut_get_resource::<ScreenDimensions>().unwrap().aspect;
  let camera = Camera::new();
  let transforms = Transforms::new(&aspect,&camera);
  let mut frame_inputs = FrameInputs::new();
  
  //main loop
  while !window.should_close() {
    //For some reason if this is not here I get a black screen
    server_time.tick();
    
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      // println!("{:?}", event);
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
          window.set_should_close(true)
        },
        // glfw::WindowEvent::CursorPos(x,y)=>{
            //eventually use for selection
        // }
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Press,..)=>{
          
          let (x,y) = window.get_cursor_pos();
          
          let event = engine::input::user_inputs::UserInputs::MouseClick(MousePosition{x,y});
          
          let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
         
          let mouse_ray = MouseRay::new(x,y, &screen_dimensions, &transforms).0;
          
          let intersection:Vec3 = mouse_ray.calculate_ray_ground_intersection();
    
          player_position = intersection;

          frame_inputs.add_event(event);
        }
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
    let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
    let (mouse_x,mouse_y) = window.get_cursor_pos();
    
    //my concern is that clearing the frame inputs means it won't update properly 
    //it will just lerp for one frame but not move the full amount
    //what I could do is instead of updating the position directly, I have the move command set a target position and then lerp between those two
    frame_inputs.clear();

    //I think this is where I update the delta timer
    server_time.decrimint_seconds_since_update()
    }
    
    //Render
    //Can I clear the buffers before binding or do they need to be cleared after binding? Binding currently happens in their own functions.
    if server_time.should_render(){
      //Picking Phase
      // ColorBuffer::clear(&gl);
      // DepthBuffer::clear(&gl);

      // let selectable_object = SelectableObject::new(&gl,&world,"picking",&ScreenDimensions::new(720,1280));
      
      // for position in positions.clone().into_iter(){
      //   selectable_object.render()?;
      // }

      
      //Render Phase
      ColorBuffer::clear(&gl);
      DepthBuffer::clear(&gl);

      let object = RenderableObject::new(&gl,&world,"textured",model.0.clone())?;
      
      for position in positions.clone().into_iter(){
        object.render(&gl, &transforms, &position);
      }

      object.render(&gl, &transforms, &player_position);

      window.swap_buffers();
      server_time.decrimint_seconds_since_render()
    }
  } 
  Ok(())
}
