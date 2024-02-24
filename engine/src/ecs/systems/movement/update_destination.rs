use mlua::Lua;
use crate::{component_lib::{Controllable, Destination, MovementScript, Path, Velocity, Position}, ecs::World, input::user_inputs::{FrameInputs, UserInput}, scripting::LuaEntity};

//Refactor
// -The mouse intersection should be calculated in the same place where the MouseRay is set
// -This should only run if the selection test says nothing is selected
//  needs selection needs to run first and do the AABB test
// -Revisit the limit before using the pathing algorithm once the arena is added
// -Figure out why scripts sometimes error when collecting the destination try debugging

///Updates the [`Destination`] component for the entity marked with the [`Controllable`] component.
/// If the destination is less than 100 units away, sets the `Destination` to the location of the mouse click.
/// Otherwise calculates a [`Path`] using the pathing script.
/// Does not update entity's [`Velocity`] components.
pub fn update_destination(world:&mut World){
  let frame_inputs = world.mut_get_resource::<FrameInputs>().unwrap();
  //Get the MouseClick and it's corresponding ray and convert them into a Destination
  if let Some(UserInput::MouseClick(mouse_ray)) = frame_inputs.get_input(){
    let new_destination = Destination::from(mouse_ray.ray_ground_intersection());

    //Check how far the new destination is from the mouse click
    let mut query = world.query();
    let entities = query
    .with_component::<Controllable>().unwrap()
    .with_component::<Destination>().unwrap()
    .run();

    for entity in entities { 
      let position = entity.immut_get_component::<Position>().unwrap();
      //Narrowing the scope of the mut borrow here otherwise trying to borrow it in the script causes errors
      {
        let mut destination = entity.mut_get_component::<Destination>().unwrap();
        *destination = new_destination;
      }

      // If the distance between the current Position and the Destination is large run the pathing script and replace the destination with the first node of the calculated Path
      if position.distance(&new_destination) > 100.0 {
        run_scripts(world);
        let mut path = entity.mut_get_component::<Path>().unwrap();
        if let Some(first_node) = path.next() {
         let mut destination = entity.mut_get_component::<Destination>().unwrap();
         *destination = first_node;
        }
      }
    }
  }
}


///Run a unit's pathing script [`MovementScript`]s.
pub fn run_scripts(world: &World) {
  //this works but any script that creates new entities *will* need to mutate world and be structured differently
  let lua = world.immut_get_resource::<Lua>().unwrap();  
  
  let mut query = world.query();

  //Search for all entities with a MovementScript component
  let entities = query.with_component::<MovementScript>().unwrap().run();

  for entity in entities {
    let script = entity.immut_get_component::<MovementScript>().unwrap();
    
    let entity_id = LuaEntity::from(entity.id);

    lua.scope(|scope| {
    //Set the id of the entity
    lua.globals().set("entity", scope.create_userdata_ref(&entity_id)?)?;

    //Add the world 
    lua.globals().set("world", scope.create_userdata_ref(world)?)?;

    //Run the script
    lua.load(script.script()).exec()?; 
    Ok(()) 
    }).unwrap();
  }
}