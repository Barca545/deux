use crate::{
  component_lib::SelectionRadius, ecs::{
    world_resources::{
      ScreenDimensions, Selected::{self, HOVERED, NONE}
    },
    World
  }, math::{MouseRay, Transforms}, physics::ray_aabb3d_collision_test
};
use eyre::Result;

//does not work consistently, unclear why. most consistently does not work at
// bottom of screen seems to think the hitbox is longer than it should be so
// probably has something to do with that
fn update_hovered(world:&mut World, x:f64, y:f64) -> Result<()> {
  let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();

  //should mouse ray be a resource? Probably not unless I find another way to get
  // the transforms to it.
  let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);

  let mut selection_state = NONE;

  let mut query = world.query();
  let entities = query.with_component::<SelectionRadius>()?.run_entity();
  
  for entity in entities {
    let hitbox = entity.immut_get_component::<SelectionRadius>()?;
    let hit_check = ray_aabb3d_collision_test(hitbox.0, mouse_ray.0);
    if hit_check == true {
      selection_state = HOVERED(entity.id)
    }
  }

  let selection = world.mut_get_resource::<Selected>().unwrap();
  *selection = selection_state;
  Ok(())
}

pub fn update_selection(world:&mut World, x:f64, y:f64) -> Result<()> {
  update_hovered(world, x, y)?;
  Ok(())
}
