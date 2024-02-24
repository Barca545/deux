extern crate engine;
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

use engine::{
  arena::Grid, component_lib::{GameplayRadius, Gold, Health, PathingRadius, Position, PreviousPosition, SelectionRadius, SkinnedMesh, Team, KDA}, config::asset_config, ecs::{
    systems::{combat, movement, register_components, render, spawn_enviroment, spawn_player, update_mouseray}, world_resources::{DbgShaderProgram, DebugElements, ScreenDimensions, Selected, ShaderPrograms}, World
  }, filesystem::{load_grid, load_object}, input::user_inputs::{FrameInputs, UserInput}, math::{MouseRay, Transforms, Vec3}, time::ServerTime, view::{
    window::{create_gl, create_window},
    AABB3DDebugMesh,
  }
};
use gl::Gl;
use glfw::{Action, Context, Key, MouseButton};
use mlua::Lua;

// added a FileType enum to the file system

// Refactor
// -Could probably replace the check for if position == new_position in the renderer once I add in some sort of movement state tracker
// -Consider moving to a slower tick rate LoL uses 30hz
// -Grid should load in from a JSON once I build the grid in the level editor
// -Grid my also need to be a resource. I'm unsure if other systems will need it

fn main() {
  //Configure the location of the asset folders
  asset_config();

  //could the thing where components are registered be part of world::default()
  let mut world = World::new();

  let server_time = ServerTime::new();
  //make a settings file and load in from there
  let screen_dimensions = ScreenDimensions::new(1280, 720);
  
  // let grid = load_grid("5v5", "json").unwrap();
  let grid = Grid::new(100, 100, 1.0).unwrap();

  let lua = Lua::new();
  lua.globals().set("grid", grid).unwrap();

  world
    .add_resource(screen_dimensions)
    .add_resource(Transforms::new(&screen_dimensions.aspect))
    .add_resource(Selected::NONE)
    .add_resource(MouseRay::default())
    .add_resource(FrameInputs::new())
    //add physics acceleration structure resource
    //add window?
    .add_resource(server_time)
    .add_resource(DebugElements::new(false, false))
    //Initialize Lua
    .add_resource(lua);

  let (mut glfw, mut window, events) = create_window(&world);
  let gl = create_gl(&mut window);

  //Add gl as a resource
  //experiment with making window a resource
  world.add_resource(gl.clone());

  //Create the shader programs
  let programs = ShaderPrograms::new(&world).unwrap();
  let dbg_program = DbgShaderProgram::new(&world);

  //add the programs as a resource
  world
    .add_resource(programs)
    .add_resource(dbg_program);

  //Register the components the game uses with the world
  register_components(&mut world);

  //Spawn the ground
  spawn_enviroment(&mut world, "ground").unwrap();

  //Spawn the players
  spawn_player(&mut world, "warrior", 1).unwrap();

  //Create the dummy entity 
  let dummy_position_vec:Vec3 = Vec3::new(-3.0, 0.0, 0.0);
  let dummy_position = Position(dummy_position_vec);
  let dummy_previous_position = PreviousPosition(dummy_position_vec);
  let dummy_hitbox = SelectionRadius::new(&dummy_position, 0.7, 0.7);
  let dummy_hitbox_mesh = AABB3DDebugMesh::new(&gl, dummy_hitbox.0, dummy_position_vec);
  
  let (dummy_vertices, dummy_indices) = load_object("box").unwrap();
  let dummy_mesh = SkinnedMesh::new(&gl,dummy_vertices,dummy_indices,"wall", 1.0);

  //combat info
  let dummy_team = Team::RED;
  let dummy_health = Health::new(500);
  // let dummy_target = Target(None);

  world
    .create_entity()
    // .with_component(Player).unwrap()
    .with_component(dummy_mesh).unwrap()
    .with_component(dummy_position).unwrap()
    .with_component(dummy_previous_position).unwrap()
    // .with_component(Destination::new(0.0, 0.0, 0.0)).unwrap()
    // .with_component(Speed(0.05)).unwrap()
    // .with_component(Velocity::default()).unwrap()
    .with_component(dummy_hitbox).unwrap()
    .with_component(dummy_hitbox_mesh).unwrap()
    .with_component(PathingRadius(0.2)).unwrap()
    .with_component(GameplayRadius(0.1)).unwrap()
    .with_component(dummy_team).unwrap()
    .with_component(dummy_health).unwrap()
    .with_component(Gold::default()).unwrap()
    .with_component(KDA::default()).unwrap();

  //Main loop
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
          let mouse_ray = update_mouseray(&mut world, x, y);
          let event = UserInput::MouseClick(mouse_ray);
          let frame_inputs = world.mut_get_resource::<FrameInputs>().unwrap();
          frame_inputs.add_event(event);
        }
        _ => {}
      }
    }
    
    let server_time = world.immut_get_resource::<ServerTime>().unwrap().clone();

    //Update
    if server_time.should_update() == true {      
      // let (x, y) = window.get_cursor_pos();
      // update_selection(&mut world, x, y);
      movement(&mut world);
      combat(&mut world);

      //my concern is that clearing the frame inputs means it won't update properly
      let frame_inputs = world.mut_get_resource::<FrameInputs>().unwrap();
      frame_inputs.clear();

      //I think this is where I update the delta timer
      let server_time = world.mut_get_resource::<ServerTime>().unwrap();
      server_time.decrement_seconds_since_update()
    }

    //Render
    //Can I clear the buffers before binding or do they need to be cleared after
    // binding? Binding currently happens in their own functions.
    if server_time.should_render() {
      //move the resize thing into its own function
      //to do this window needs to be a resource
      //have some flag so it only runs if it was resized
      let (width,height) = window.get_size();
      {
        let dimensions = world.mut_get_resource::<ScreenDimensions>().unwrap();
        *dimensions = ScreenDimensions::new(width, height);
      }
      
      {
        let dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap().clone();
        
        let transforms = world.mut_get_resource::<Transforms>().unwrap();
        
        *transforms = Transforms::new(&dimensions.aspect);

        let gl = world.immut_get_resource::<Gl>().unwrap();
        unsafe{gl.Viewport(0, 0, width, height)}
      }

      //can maybe make the render function handle the swapbuffers
      render(&world);

      window.swap_buffers();
      let server_time = world.mut_get_resource::<ServerTime>().unwrap();
      server_time.decrement_seconds_since_render()
    }
  }
}