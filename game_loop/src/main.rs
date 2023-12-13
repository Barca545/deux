extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  ecs::{
    component_lib::{
      Controllable, Destination, GroundModel, Model, ModelUniformLocation, Position,
      ProjectionUniformLocation, Speed, Velocity, ViewUniformLocation,
    },
    systems::{render, resolve_movement, set_destination},
    world_resources::ScreenDimensions,
    World,
  },
  input::user_inputs::{FrameInputs, MousePosition},
  math::{math::Vec3, MouseRay, Renderable, Transforms},
  time::ServerTime,
  view::{
    camera::Camera,
    render::Mesh,
    render_gl::{
      buffer::{ArrayBuffer, VertexArray},
      ColorBuffer, DepthBuffer, Program, RenderableObject, Texture, Vertex, Viewport,
    },
  },
};

use eyre::Result;
use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, DEPTH_TEST};
use glfw::{fail_on_errors, Action, Context, Key, MouseButton};
use std::{any::TypeId, env};

fn main() -> Result<()> {
  env::set_var("RUST_BACKTRACE", "FULL");
  let mut world = World::new();
  let mut server_time = ServerTime::new();

  world
    .add_resource()
    .from_user_defined_data(ScreenDimensions::new(720, 1280));
  // world.add_resource().path_to_asset_folder_from_relative_exe_path("assets");

  // world.add_resource().from_user_defined_data(MousePicker::new());

  //maybe I make events a or a component and query it? Might need to slap it in an RC if I want to pass it down to other functions
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  let (mut window, events) = glfw
    .create_window(
      world
        .immut_get_resource::<ScreenDimensions>()
        .unwrap()
        .width as u32,
      world
        .immut_get_resource::<ScreenDimensions>()
        .unwrap()
        .height as u32,
      "Project: Deux",
      glfw::WindowMode::Windowed,
    )
    .expect("Failed to create GLFW window.");

  window.make_current();
  window.set_all_polling(true); //maybe just use the polling for specific keys and then poll the mouse separately

  let _gl_context = window.get_context_version();

  let gl = Gl::load_with(&mut |s| window.get_proc_address(s) as *const std::os::raw::c_void);
  unsafe { gl.Enable(DEPTH_TEST) }

  world.add_resource().from_user_defined_data(gl.clone());

  let viewport = Viewport::for_window(
    world
      .immut_get_resource::<ScreenDimensions>()
      .unwrap()
      .height,
    world
      .immut_get_resource::<ScreenDimensions>()
      .unwrap()
      .width,
  );

  world
    .register_component::<Mesh>()
    .register_component::<Position>()
    .register_component::<Destination>()
    .register_component::<Speed>()
    .register_component::<Velocity>()
    .register_component::<Controllable>();

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
    Vertex::from((1.0, -1.0, 1.0, 1.0, 1.0)),
    Vertex::from((1.0, -1.0, -1.0, 0.0, 0.0)),
    Vertex::from((-1.0, -1.0, 1.0, 0.0, 1.0)),
    Vertex::from((-1.0, -1.0, -1.0, 0.0, 0.0)),
  ];

  world
    .create_entity()
    .with_component(Mesh::new(&gl, "ground.jpg", ground_model))?
    .with_component(Position::new(3.0, 0.0, 0.0))?;

  // create the player entity
  world
    .create_entity()
    .with_component(Mesh::new(&gl, "wall.jpg", sprite_model))?
    .with_component(Position::new(0.0, 0.0, 0.0))?
    .with_component(Destination::new(0.0, 0.0, 0.0))?
    .with_component(Speed(0.5))?
    .with_component(Velocity::new(
      &Position::new(0.0, 0.0, 0.0),
      &Destination::new(0.0, 0.0, 0.0),
      &Speed(0.5),
    ))?
    .with_component(Controllable)?;

  let color_buffer = ColorBuffer::from_color(0.3, 0.3, 0.5, 0.1);
  color_buffer.set_used(&gl);
  viewport.set_used(&gl);

  //this whole camera/transform section needs to be its own system
  let aspect = world
    .immut_get_resource::<ScreenDimensions>()
    .unwrap()
    .aspect;
  let camera = Camera::new();
  let transforms = Transforms::new(&aspect, &camera);

  world
    .add_resource()
    .from_user_defined_data(Transforms::new(&aspect, &camera));
  world
    .add_resource()
    .from_user_defined_data(Program::from_shader_files(&gl, "textured"));
  world
    .add_resource()
    .from_user_defined_data(ModelUniformLocation(0));
  world
    .add_resource()
    .from_user_defined_data(ViewUniformLocation(3));
  world
    .add_resource()
    .from_user_defined_data(ProjectionUniformLocation(2));

  let mut frame_inputs = FrameInputs::new();

  //main loop
  while !window.should_close() {
    //For some reason if this is not here I get a black screen
    server_time.tick();

    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      // println!("{:?}", event);
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        // glfw::WindowEvent::CursorPos(x,y)=>{
        //eventually use for selection
        // }
        glfw::WindowEvent::MouseButton(MouseButton::Button2, Action::Press, ..) => {
          let (x, y) = window.get_cursor_pos();
          let event = engine::input::user_inputs::UserInputs::MouseClick(MousePosition { x, y });

          //this needs to go into the update section.
          //the transforms and mouse coordinates need to become queryable from world
          set_destination(&mut world, x, y, &transforms)?;
          frame_inputs.add_event(event);
        }
        _ => {}
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
    if server_time.should_update() == true {
      //add picking system
      // set_destination(&mut world,x,y,&transforms)?;
      resolve_movement(&world)?;

      //my concern is that clearing the frame inputs means it won't update properly
      //it will just lerp for one frame but not move the full amount
      //what I could do is instead of updating the position directly, I have the move command set a target position and then lerp between those two
      frame_inputs.clear();

      //I think this is where I update the delta timer
      server_time.decrimint_seconds_since_update()
    }

    //Render
    //Can I clear the buffers before binding or do they need to be cleared after binding? Binding currently happens in their own functions.
    if server_time.should_render() {
      //this is where I need to use this factor 
      //probably what I do is make an endtickposition and starttickposition in the position component
      //interpolate between those
      //end tick is the position + velocity
      //start tick is the
      let interpolation_factor = server_time.get_interpolation_factor();
      
      //Render Phase
      render(&world)?;

      // let ground_obj = RenderableObject::new(&gl, &world, "textured", ground.0.clone(),"wall.jpg")?;
      // ground_obj.render(&gl, &transforms, &ground_position);

      window.swap_buffers();
      server_time.decrimint_seconds_since_render()
    }
  }
  Ok(())
}
