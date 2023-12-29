extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  ecs::{
    component_lib::{Controllable, Destination, SelectionRadius, Position, Speed, Velocity, PathingRadius, SkinnedMesh, StaticMesh, Target, Team, MissleSpeed, AutoAttackMeshCreator, AutoAttackCooldown, AutoAttack},
    systems::{movement, render, update_destination, update_selection, combat},
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
use gl::FRAGMENT_SHADER;
use glfw::{Action, Context, Key, MouseButton};
use glm::vec3;
use std::env;

fn main() -> Result<()> {
  env::set_var("RUST_BACKTRACE", "FULL");
  let mut world = World::new();
  let mut server_time = ServerTime::new();
  let screen_dimensions = ScreenDimensions::new(720, 1280);

  world
    .add_resource(screen_dimensions)
    .add_resource(Transforms::new(&screen_dimensions.aspect))
    .add_resource(RenderUniformLocations::new(0, 3, 2))
    .add_resource(Selected::NONE)
    //add MouseRay resource
    //add physics acceleration structure resource
    //add events resource
    .add_resource(server_time)
    .add_resource(DebugElements::new(true));

  let (mut glfw, mut window, events) = create_window(&world);
  let gl = create_gl(&mut window);

  //make an update size system to handle this?
  // let (width,heigth) = window.get_size();

  let programs = ShaderPrograms {
    normal:Program::new(&gl, "textured", "textured", FRAGMENT_SHADER).unwrap(),
    highlight:Program::new(&gl, "textured", "highlight", FRAGMENT_SHADER).unwrap()
  };
  let dbg_program = DbgShaderProgram::new(Program::new(&gl, "debug", "debug", FRAGMENT_SHADER).unwrap());

  world.add_resource(programs).add_resource(dbg_program).add_resource(gl.clone());

  world
    .register_component::<SkinnedMesh>()
    .register_component::<StaticMesh>()
    .register_component::<Position>()
    .register_component::<Destination>()
    .register_component::<Speed>()
    .register_component::<Velocity>()
    .register_component::<Controllable>()
    .register_component::<SelectionRadius>()
    .register_component::<AABB3DDebugMesh>()
    .register_component::<PathingRadius>()
    .register_component::<Target>()
    .register_component::<Team>()
    .register_component::<MissleSpeed>()
    .register_component::<AutoAttackMeshCreator>()
    .register_component::<AutoAttackCooldown>()
    .register_component::<AutoAttack>();
    // .register_component::<Team>();

  // create the ground entity
  let ground_position_vec:Vec3 = vec3(0.0, -0.5, 0.0);
  let ground_position = Position::new(ground_position_vec, ground_position_vec);
  let (ground_vertices, ground_indices) = load_object("ground")?;
  let player_mesh = StaticMesh::new(&gl,ground_vertices,ground_indices,"ground");


  world
    .create_entity()
    .with_component(player_mesh)?
    .with_component(ground_position)?;

  // create the player entity 
  let player_position_vec:Vec3 = vec3(0.0, 0.0, 0.0);
  let player_position = Position::new(player_position_vec, player_position_vec);
  let player_hitbox = SelectionRadius::new(player_position_vec, 0.7, 0.7);
  let player_hitbox_mesh = AABB3DDebugMesh::new(&gl, player_hitbox.0, player_position_vec);
  
  let (sprite_vertices, sprite_indices) = load_object("box")?;
  let player_mesh = SkinnedMesh::new(&gl,sprite_vertices,sprite_indices,"blank_texture");

  //combat info
  let team = Team::BLUE;
  let target = Target(None);
  let (auto_attack_vertices, auto_attack_indices) = load_object("ball")?;
  let auto_attack_mesh_info = AutoAttackMeshCreator::new(auto_attack_vertices, auto_attack_indices, "allied_attack".to_owned());
  let missle_speed = MissleSpeed(0.07);
  let auto_attack_cooldown = AutoAttackCooldown::new(1.0, 0.0);

  world
    .create_entity()
    .with_component(Controllable)?
    .with_component(player_mesh)?
    .with_component(player_position)?
    .with_component(Destination::new(0.0, 0.0, 0.0))?
    .with_component(Speed(0.05))?
    .with_component(Velocity::default())?
    .with_component(player_hitbox)?
    .with_component(player_hitbox_mesh)?
    .with_component(PathingRadius(0.5))?
    .with_component(team)?
    .with_component(target)?
    .with_component(auto_attack_mesh_info)?
    .with_component(missle_speed)?
    .with_component(auto_attack_cooldown)?;

  // create the dummy entity 
  let dummy_position_vec:Vec3 = vec3(3.0, 0.0, 0.0);
  let dummy_position = Position::new(dummy_position_vec, dummy_position_vec);
  let dummy_hitbox = SelectionRadius::new(dummy_position_vec, 0.7, 0.7);
  let dummy_hitbox_mesh = AABB3DDebugMesh::new(&gl, dummy_hitbox.0, dummy_position_vec);
  
  let (dummy_vertices, dummy_indices) = load_object("box")?;
  let dummy_mesh = SkinnedMesh::new(&gl,dummy_vertices,dummy_indices,"wall");

  //combat info
  let dummy_team = Team::RED;
  // let dummy_target = Target(None);
  

  world
    .create_entity()
    .with_component(dummy_mesh)?
    .with_component(dummy_position)?
    // .with_component(Destination::new(0.0, 0.0, 0.0))?
    // .with_component(Speed(0.05))?
    // .with_component(Velocity::default())?
    .with_component(dummy_hitbox)?
    .with_component(dummy_hitbox_mesh)?
    .with_component(PathingRadius(0.5))?
    .with_component(dummy_team)?;
    // .with_component(dummy_target)?;

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
