use game_data::{Controllable, Position, PreviousPosition, SkinnedRenderable, StaticRenderable};
use math::interpolate;
use nina::world::World;
use renderer::{
  renderer::Renderer,
  scene::{camera::Camera, SceneBuilder},
  Instance,
};
use time::ServerTime;

/// System which creates and renders a [`Scene`](renderer::scene::Scene).
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
  let player_render_position = interpolate(
    player_previous_position.0,
    player_position.0,
    interpolation_factor as f32,
  );

  // Update the camera's position
  let mut camera = world.get_resource_mut::<Camera>();
  camera.offset_camera_relative_to_position(player_render_position,);

  // Render skinned models
  let mut query = world.query();
  let entities = query.with_component::<SkinnedRenderable>().unwrap().run();

  // Create a scene to draw to
  let mut scene = SceneBuilder::new();

  for entity in entities {
    let model = &entity.get_component::<SkinnedRenderable>().unwrap().0;
    let position = entity.get_component::<Position>().unwrap();
    let previous_position = entity.get_component::<PreviousPosition>().unwrap();

    let instance = Instance::new(interpolate(
      previous_position.0,
      position.0,
      interpolation_factor as f32,
    ),);

    // Add the new instance to the scene
    scene.insert(model, instance,);
  }

  // Render static models
  let mut query = world.query();
  let entities = query.with_component::<StaticRenderable>().unwrap().run();
  // Add every instance of a model which needs to be rendered to the frame
  for entity in entities {
    let model = &entity.get_component::<StaticRenderable>().unwrap().0;
    let position = entity.get_component::<Position>().unwrap();

    let instance = Instance::new(position.0,);

    // Add the new instance to the scene
    scene.insert(model, instance,);
  }

  // Draw
  renderer.render(camera, scene.build(),).unwrap();
}
