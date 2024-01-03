extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  ecs::{
    component_lib::{SelectionRadius, Position, PathingRadius, SkinnedMesh, Team, Health, GameplayRadius, Gold, KDA},
    systems::{movement, render, update_destination, update_selection, combat, spawn_player, spawn_enviroment, register_components},
    world_resources::{DbgShaderProgram, DebugElements, RenderUniformLocations, ScreenDimensions, Selected, ShaderPrograms},
    World
  },
  input::user_inputs::{FrameInputs, MousePosition, UserInputs},
  math::{Transforms, Vec3},
  time::ServerTime,
  view::{
    render_gl::Program,
    window::{create_gl, create_window},
    AABB3DDebugMesh,
  }, filesystem::load_object
};

use eyre::Result;
use gl::{FRAGMENT_SHADER, Gl};
use glfw::{Action, Context, Key, MouseButton};
use glm::vec3;
use std::env;

fn main() -> Result<()> {
  env::set_var("RUST_BACKTRACE", "FULL");
  let mut world = World::new();
  let server_time = ServerTime::new();
  //make a settings thing and load in from there
  let screen_dimensions = ScreenDimensions::new(1280, 720);

  world
    .add_resource(screen_dimensions)
    .add_resource(Transforms::new(&screen_dimensions.aspect))
    //can maybe add a function to get these automatically so I don't have to hard code it
    .add_resource(RenderUniformLocations::new(0, 3, 2))
    .add_resource(Selected::NONE)
    //add MouseRay resource
    //add physics acceleration structure resource
    //add events resource
    .add_resource(server_time)
    .add_resource(DebugElements::new(false, false));

  let (mut glfw, mut window, events) = create_window(&world);
  let gl = create_gl(&mut window);

  //add gl as a resource
  world.add_resource(gl.clone());

  //create the programs
  let programs = ShaderPrograms::new(&world);
  let dbg_program = DbgShaderProgram::new(Program::new(&gl, "debug", "debug", FRAGMENT_SHADER).unwrap());

  //add the programs as a resource
  world
    .add_resource(programs)
    .add_resource(dbg_program);

  //register the components the game uses with the world
  register_components(&mut world);

  //spawn the ground
  spawn_enviroment(&mut world, "ground")?;

  //spawn the players
  spawn_player(&mut world, "warrior", 1)?;

  // create the dummy entity 
  let dummy_position_vec:Vec3 = vec3(3.0, 0.0, 0.0);
  let dummy_position = Position::new(dummy_position_vec, dummy_position_vec);
  let dummy_hitbox = SelectionRadius::new(dummy_position_vec, 0.7, 0.7);
  let dummy_hitbox_mesh = AABB3DDebugMesh::new(&gl, dummy_hitbox.0, dummy_position_vec);
  
  let (dummy_vertices, dummy_indices) = load_object("box")?;
  let dummy_mesh = SkinnedMesh::new(&gl,dummy_vertices,dummy_indices,"wall", 1.0);

  //combat info
  let dummy_team = Team::RED;
  let dummy_health = Health::new(500);
  // let dummy_target = Target(None);

  world
    .create_entity()
    // .with_component(Player)?
    .with_component(dummy_mesh)?
    .with_component(dummy_position)?
    // .with_component(Destination::new(0.0, 0.0, 0.0))?
    // .with_component(Speed(0.05))?
    // .with_component(Velocity::default())?
    .with_component(dummy_hitbox)?
    .with_component(dummy_hitbox_mesh)?
    .with_component(PathingRadius(0.2))?
    .with_component(GameplayRadius(0.1))?
    .with_component(dummy_team)?
    .with_component(dummy_health)?
    .with_component(Gold::default())?
    .with_component(KDA::default())?;

  let mut frame_inputs = FrameInputs::new();

  //main loop
  while !window.should_close() {
    //For some reason if this is not here I get a black screen
    {
      let server_time = world.mut_get_resource::<ServerTime>().unwrap();
      server_time.tick();
    }

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
    
    let server_time = world.immut_get_resource::<ServerTime>().unwrap().clone();

    //Update
    if server_time.should_update() == true {      
      let (x, y) = window.get_cursor_pos();
      update_selection(&mut world, x, y)?;
      movement(&world)?;
      combat(&mut world)?;

      //my concern is that clearing the frame inputs means it won't update properly
      frame_inputs.clear();

      //I think this is where I update the delta timer
      let server_time = world.mut_get_resource::<ServerTime>().unwrap();
      server_time.decrimint_seconds_since_update()
    }

    //Render
    //Can I clear the buffers before binding or do they need to be cleared after
    // binding? Binding currently happens in their own functions.
    if server_time.should_render() {
      //this does not work because I also need to update the transforms?
      //the transforms are being used somewhere (probably in the shader program) without getting fed the update
      let (width,height) = window.get_size();
      {
        let dimensions = world.mut_get_resource::<ScreenDimensions>().unwrap();
        *dimensions = ScreenDimensions::new(width, height);
        // dbg!((width,height));
        // dbg!(width/height);
      }
      
      {
        let dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap().clone();
        
        let transforms = world.mut_get_resource::<Transforms>().unwrap();
        
        *transforms = Transforms::new(&dimensions.aspect);

        let gl = world.immut_get_resource::<Gl>().unwrap();
        unsafe{
          gl.Viewport(0, 0, width, height);
        }
      }

      //can maybe make the render function handle the swapbuffers
      let interpolation_factor = server_time.get_interpolation_factor();
      render(&world, interpolation_factor)?;

      window.swap_buffers();
      let server_time = world.mut_get_resource::<ServerTime>().unwrap();
      server_time.decrimint_seconds_since_render()
    }
  }
  Ok(())
}