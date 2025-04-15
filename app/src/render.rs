use nina::world::World;
use renderer::{renderer::Renderer, scene::camera::Camera};
use time::ServerTime;

// query the world and create draw calls
// The render loop can take the draw calls as input + maybe the camera position?

// TODO: All of this should create a draw call and pass it to the renderer
pub fn render(world: &World, renderer: &mut Renderer,) {
  // Call it once up here so each object has the same interpolation factor instead
  // of slightly different ones
  let interpolation_factor = world
    .get_resource::<ServerTime>()
    .get_interpolation_factor();

  // Update the camera

  let mut query = world.query();
  let player = &query.with_component::<Controllable>().unwrap().run()[0];
  let player_position = player.get_component::<Position>().unwrap();
  let player_previous_position = player.get_component::<PreviousPosition>().unwrap();
  let player_render_position = calculate_render_position(
    *player_previous_position,
    *player_position,
    interpolation_factor,
  )
  .0;

  let mut camera = world.get_resource_mut::<Camera>();

  camera.offset_camera_relative_to_position(player_render_position,);

  // Render skinned models
  let mut query = world.query();
  let entities = query.with_component::<SkinnedRenderable>().unwrap().run();

  // Create a scene to draw to
  let mut scene = Vec::new();

  for entity in entities {
    let model_id = &entity.get_component::<SkinnedRenderable>().unwrap().0;
    let position = entity.get_component::<Position>().unwrap();
    let previous_position = entity.get_component::<PreviousPosition>().unwrap();

    let instance = Instance::new(
      // TODO: I think calculate_render_position could be an associated function on the position
      // struct or something
      calculate_render_position(*previous_position, *position, interpolation_factor,).0,
    );

    // Group the instances for drawing
    // frame.record_instance(&model_id, instance,);

    // Create draw calls
    renderer.render(camera, scene,);
  }

  // // Render static models
  // let mut query = world.query();
  // let entities = query.with_component::<StaticRenderable>().unwrap().run();
  // // Add every instance of a model which needs to be rendered to the frame
  // for entity in entities {
  //   let model_id = entity.get_component::<StaticRenderable>().unwrap();
  //   let position = entity.get_component::<Position>().unwrap();
  // }
}
