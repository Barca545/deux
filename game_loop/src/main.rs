extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  ecs::{
    component_lib::{
      Controllable, Destination, ModelUniformLocation, Position, ProjectionUniformLocation, Speed, Velocity,
      ViewUniformLocation, Hitbox
    },
    systems::{render, resolve_movement, update_destination, update_selection},
    world_resources::{ScreenDimensions, ShaderPrograms, Selected},
    World
  },
  input::user_inputs::{FrameInputs, MousePosition, UserInputs},
  math::{Transforms, Vec3},
  time::ServerTime,
  view::{
    camera::Camera,
    render_gl::{ColorBuffer, Program, Vertex, Viewport}, SkinnedMesh, StaticMesh
  }
};

use eyre::Result;
use gl::{Gl, DEPTH_TEST, LESS, NOTEQUAL, STENCIL_TEST, KEEP, REPLACE};
use glfw::{fail_on_errors, Action, Context, Key, MouseButton,WindowHint::{ContextVersionMinor,ContextVersionMajor,OpenGlProfile}, OpenGlProfileHint};
use glm::vec3;
use std::env;

fn main() -> Result<()> {
  env::set_var("RUST_BACKTRACE", "FULL");
  let mut world = World::new();
  let mut server_time = ServerTime::new();

  world
    .add_resource(ScreenDimensions::new(720, 1280));

  //maybe I make events a or a component and query it? Might need to slap it in
  // an RC if I want to pass it down to other functions
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  glfw.window_hint(ContextVersionMajor(3));
  glfw.window_hint(ContextVersionMinor(3));
  glfw.window_hint(OpenGlProfile(OpenGlProfileHint::Core));
  let (mut window, events) = glfw
    .create_window(
      world.immut_get_resource::<ScreenDimensions>().unwrap().width as u32,
      world.immut_get_resource::<ScreenDimensions>().unwrap().height as u32,
      "Project: Deux",
      glfw::WindowMode::Windowed
    )
    .expect("Failed to create GLFW window.");

  
  window.make_current();
  //maybe just use the polling for specific keys and then poll the mouse separately
  window.set_all_polling(true); 

  let _gl_context = window.get_context_version();

  let gl = Gl::load_with(&mut |s| window.get_proc_address(s) as *const std::os::raw::c_void);
  //configure gloabl OpenGL state
  unsafe { 
    gl.Enable(DEPTH_TEST);
    gl.DepthFunc(LESS);
    gl.Enable(STENCIL_TEST);
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilOp(KEEP, KEEP, REPLACE);
  }

  world.add_resource(gl.clone());

  let viewport = Viewport::for_window(
    world.immut_get_resource::<ScreenDimensions>().unwrap().height,
    world.immut_get_resource::<ScreenDimensions>().unwrap().width
  );

  world
    .register_component::<SkinnedMesh>()
    .register_component::<StaticMesh>()
    .register_component::<Position>()
    .register_component::<Destination>()
    .register_component::<Speed>()
    .register_component::<Velocity>()
    .register_component::<Controllable>()
    .register_component::<Hitbox>();

  let sprite_model = vec![
    Vertex::from((-0.5, -0.5, -0.5, 0.0, 0.0)),
    Vertex::from((0.5, -0.5, -0.5, 1.0, 0.0)),
    Vertex::from((0.5, 0.5, -0.5, 1.0, 1.0)),
    Vertex::from((0.5, 0.5, -0.5, 1.0, 1.0)),
    Vertex::from((-0.5, 0.5, -0.5, 0.0, 1.0)),
    Vertex::from((-0.5, -0.5, -0.5, 0.0, 0.0)),
    Vertex::from((-0.5, -0.5, 0.5, 0.0, 0.0)),
    Vertex::from((0.5, -0.5, 0.5, 1.0, 0.0)),
    Vertex::from((0.5, 0.5, 0.5, 1.0, 1.0)),
    Vertex::from((0.5, 0.5, 0.5, 1.0, 1.0)),
    Vertex::from((-0.5, 0.5, 0.5, 0.0, 1.0)),
    Vertex::from((-0.5, -0.5, 0.5, 0.0, 0.0)),
    Vertex::from((-0.5, 0.5, 0.5, 1.0, 0.0)),
    Vertex::from((-0.5, 0.5, -0.5, 1.0, 1.0)),
    Vertex::from((-0.5, -0.5, -0.5, 0.0, 1.0)),
    Vertex::from((-0.5, -0.5, -0.5, 0.0, 1.0)),
    Vertex::from((-0.5, -0.5, 0.5, 0.0, 0.0)),
    Vertex::from((-0.5, 0.5, 0.5, 1.0, 0.0)),
    Vertex::from((0.5, 0.5, 0.5, 1.0, 0.0)),
    Vertex::from((0.5, 0.5, -0.5, 1.0, 1.0)),
    Vertex::from((0.5, -0.5, -0.5, 0.0, 1.0)),
    Vertex::from((0.5, -0.5, -0.5, 0.0, 1.0)),
    Vertex::from((0.5, -0.5, 0.5, 0.0, 0.0)),
    Vertex::from((0.5, 0.5, 0.5, 1.0, 0.0)),
    Vertex::from((-0.5, -0.5, -0.5, 0.0, 1.0)),
    Vertex::from((0.5, -0.5, -0.5, 1.0, 1.0)),
    Vertex::from((0.5, -0.5, 0.5, 1.0, 0.0)),
    Vertex::from((0.5, -0.5, 0.5, 1.0, 0.0)),
    Vertex::from((-0.5, -0.5, 0.5, 0.0, 0.0)),
    Vertex::from((-0.5, -0.5, -0.5, 0.0, 1.0)),
    Vertex::from((-0.5, 0.5, -0.5, 0.0, 1.0)),
    Vertex::from((0.5, 0.5, -0.5, 1.0, 1.0)),
    Vertex::from((0.5, 0.5, 0.5, 1.0, 0.0)),
    Vertex::from((0.5, 0.5, 0.5, 1.0, 0.0)),
    Vertex::from((-0.5, 0.5, 0.5, 0.0, 0.0)),
    Vertex::from((-0.5, 0.5, -0.5, 0.0, 1.0)),
  ];

  //why do they need to be at -1, why does putting them at -1 not put them on the ground
  let ground_model = vec![
    Vertex::from((5.0, -0.5,  5.0,  2.0, 0.0)),
    Vertex::from((-5.0, -0.5,  5.0,  0.0, 0.0)),
    Vertex::from((-5.0, -0.5, -5.0,  0.0, 2.0)),

    Vertex::from((5.0, -0.5,  5.0,  2.0, 0.0)),
    Vertex::from((-5.0, -0.5, -5.0,  0.0, 2.0)),
    Vertex::from((5.0, -0.5, -5.0,  2.0, 2.0))
  ];

  // create the ground entity
  let ground_position_vec:Vec3 = vec3(0.0, 0.0, 0.0);
  let ground_position = Position::new(ground_position_vec, ground_position_vec);
  
  world
    .create_entity()
    .with_component(StaticMesh::new(&gl, "ground.jpg", ground_model))?
    .with_component(ground_position)?;

  // create the player entity
  let player_position_vec:Vec3 = vec3(0.0, 0.0, 0.0);
  let player_position = Position::new(player_position_vec, player_position_vec);
  let player_hitbox = Hitbox::new(player_position_vec, 0.5,0.5, 0.5);
  
  world
    .create_entity()
    .with_component(SkinnedMesh::new(&gl, "wall.jpg", sprite_model.clone()))?
    .with_component(player_position)?
    .with_component(Destination::new(0.0, 0.0, 0.0))?
    .with_component(Speed(0.05))?
    .with_component(Velocity::new(
      &player_position,
      &Destination::new(0.0, 0.0, 0.0),
      &Speed(0.5)
    ))?
    .with_component(Controllable)?
    .with_component(player_hitbox)?;


  // create the dummy entity
  let dummy_position_vec:Vec3 = vec3(-3.0, 0.0, 0.0);
  let dummy_position = Position::new(dummy_position_vec, dummy_position_vec);
  let dummy_hitbox = Hitbox::new(dummy_position_vec, 0.5,0.5, 0.5);
  
  world
    .create_entity()
    .with_component(SkinnedMesh::new(&gl, "wall.jpg", sprite_model))?
    .with_component(dummy_position)?
    .with_component(Destination::new(-3.0, 0.0, 0.0))?
    .with_component(Speed(0.05))?
    .with_component(Velocity::new(
      &dummy_position,
      &Destination::new(-3.0, 0.0, 0.0),
      &Speed(0.5)
    ))?
    .with_component(dummy_hitbox)?;  
  viewport.set_used(&gl);

  //this whole camera/transform section needs to be its own system
  let aspect = world.immut_get_resource::<ScreenDimensions>().unwrap().aspect;
  let camera = Camera::new();

  //refactor the Program so it can reuse the "textured.vert" for the highlight Program
  let programs = ShaderPrograms{
    normal: Program::from_shader_files(&gl, "textured"),
    highlight: Program::from_shader_files(&gl, "highlight")
  };

  world
    .add_resource(Transforms::new(&aspect, &camera))  
    .add_resource(programs)
    .add_resource(ModelUniformLocation(0))
    .add_resource(ViewUniformLocation(3))
    .add_resource(ProjectionUniformLocation(2))
    .add_resource(Selected::NONE);

  let mut frame_inputs = FrameInputs::new();

  //main loop
  while !window.should_close() {
    //For some reason if this is not here I get a black screen
    server_time.tick();

    //I don't think I want to poll events, I want to put them into an event pump?
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Press, ..) => {
          let (x, y) = window.get_cursor_pos();
          let event = UserInputs::MouseClick(MousePosition { x, y });

          //this needs to go into the update section.
          //the transforms and mouse coordinates need to become queryable from world
          update_destination(&mut world, x, y)?;
          frame_inputs.add_event(event);
        }
        _ => {}
      }
    }
    // for event in event_pump.poll_iter() {
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
    //       // //I might need to break out glfw here since sdl does not track the z
    // position       //I need to figure why this part of the input capture
    // causes the program to lag out       // Event::MouseMotion { x, y,..}=>{
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
    if server_time.should_update() == true {
      let (x, y) = window.get_cursor_pos();
      update_selection(&mut world, x, y)?;

      // set_destination(&mut world,x,y,&transforms)?;
      resolve_movement(&world)?;

      //my concern is that clearing the frame inputs means it won't update properly
      //it will just lerp for one frame but not move the full amount
      //what I could do is instead of updating the position directly, I have the move
      // command set a target position and then lerp between those two
      frame_inputs.clear();

      //I think this is where I update the delta timer
      server_time.decrimint_seconds_since_update()
    }

    //Render
    //Can I clear the buffers before binding or do they need to be cleared after
    // binding? Binding currently happens in their own functions.
    if server_time.should_render() {
      //can maybe make the render function handle the swapbuffers
      //possibly find another way to get the interpolation factor
      let interpolation_factor = server_time.get_interpolation_factor();
      render(&world, interpolation_factor)?;

      window.swap_buffers();
      server_time.decrimint_seconds_since_render()
    }
  }
  Ok(())
}
