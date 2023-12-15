use crate::{ecs::{World, component_lib::Hitbox, world_resources::{ScreenDimensions, Selected,Selected::{HOVERED,NONE}}}, math::{MouseRay, Transforms}, physics::check_ray_aabb3d_collision};
use eyre::Result;

//I need some way to query entity by "id" so this is actually useful
//does not work consistently, unclear why. most consistently does not work at bottom of screen
fn update_hovered(world:&mut World,x:f64,y:f64)->Result<()>{
  let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  //should mouse ray be a resource? Probably not unless I find another way to get the transforms to it.
  let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);

  let mut selection_state = NONE;
  
  let mut query = world.query();
  let entities = query.with_component::<Hitbox>()?.run_entity();
  for entity in entities {
    let hitbox = entity.immut_get_component::<Hitbox>()?;
    let hit_check = check_ray_aabb3d_collision(hitbox.outer, mouse_ray.0); 
    if hit_check == true {
      selection_state = HOVERED(entity.id)
    }
  }

  let selection = world.mut_get_resource::<Selected>().unwrap();
  *selection = selection_state;

  Ok(())
}

pub fn update_selection(world:&mut World,x:f64,y:f64)->Result<()>{
  let mut query = world.query();
  let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
  let transforms = world.immut_get_resource::<Transforms>().unwrap();
  
  let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms);

  let entities = query.with_component::<Hitbox>()?.run_entity();
  let mut id = -1;
  
  for entity in entities {
    let hitbox = entity.immut_get_component::<Hitbox>()?;
    let hit_check = check_ray_aabb3d_collision(hitbox.outer, mouse_ray.0); 
    if hit_check == true {
      id = entity.id as i32;
    }
  }
  
  if id != -1 {
    dbg!(id);
    // world.add_component_to_entity_by_id(Selected, id as usize)?;
  }

  //need a way to set selection permanently when clicked possibly have a bool inside that changes to true when clicked
  
  //this might need to be in a separate selection cleanup function
  else {
    let mut query = world.query();
    let entities = query.with_component::<Selected>()?.run_entity();
    if entities.len() > 0{
      let id = &entities[0].id;
      world.delete_component_by_entity_id::<Selected>(*id)?;
    }
  }
  Ok(())
}
