use std::rc::Rc;

use crate::{
  component_lib::{Controllable, Destination, Owner, Path, Position, Target},
  ecs::World,
  input::user_inputs::{FrameInputs, UserInput},
};
use mlua::Lua;

//Refactor
// -The mouse intersection should be calculated in the same place where the MouseRay is set
// -This should only run if the selection test says nothing is selected
//  needs selection needs to run first and do the AABB test
// -Revisit the limit before using the pathing algorithm once the arena is added
// -Add attack move as a toggle (If there is a target, move into attack range and no further)
// -Update the run scripts function to use the new scripting set up

///Updates the [`Destination`] component for the entity marked with the [`Controllable`] component.
/// If the destination is less than 100 units away, sets the `Destination` to the location of the mouse click.
/// Otherwise calculates a [`Path`] using the pathing script.
/// Does not update entity's [`Velocity`] components.
pub fn update_destination(world: &World) {
  let frame_inputs = world.get_resource_mut::<FrameInputs>().unwrap();
  //Get the MouseClick and it's corresponding ray and convert them into a Destination
  if let Some(UserInput::MouseClick(mouse_ray)) = frame_inputs.get_input() {
    let new_destination = Destination::from(mouse_ray.ray_ground_intersection());

    //Check how far the new destination is from the mouse click
    let mut query = world.query();
    let entities = query
      .with_component::<Controllable>()
      .unwrap()
      .with_component::<Destination>()
      .unwrap()
      .run();

    for entity in entities {
      let target = entity.get_component::<Target>().unwrap();
      //Do not move if the entity has a selected target
      if target.0 == None {
        let position = entity.get_component::<Position>().unwrap();
        //Narrowing the scope of the mut borrow here otherwise trying to borrow it in the script causes errors
        {
          let mut destination = entity.get_component_mut::<Destination>().unwrap();
          *destination = new_destination;
        }

        // If the distance between the current Position and the Destination is large run the pathing script and replace the destination with the first node of the calculated Path
        if position.distance(&new_destination) > 100.0 {
          run_scripts(world, Owner(entity.id));
          let mut path = entity.get_component_mut::<Path>().unwrap();
          if let Some(first_node) = path.next() {
            let mut destination = entity.get_component_mut::<Destination>().unwrap();
            *destination = first_node;
          }
        }
      }
    }
  }
}

///Run a unit's pathing script [`MovementScript`]s.
pub fn run_scripts(world: &World, _owner: Owner) {
  let _lua = world.get_resource::<Rc<Lua>>().unwrap();
}
