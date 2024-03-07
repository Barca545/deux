use std::rc::Rc;

use crate::{
  component_lib::{Destination, Owner, Path, Position},
  ecs::World,
  event::{GameEvent, GameEventQueue},
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
  let events = world.get_resource::<GameEventQueue>().unwrap();
  events.process_events(|event| {
    if let GameEvent::UpdateDestination { owner, mouseray } = event {
      let mut destination = world.get_component_mut::<Destination>(owner.0).unwrap();
      let position = world.get_component_mut::<Position>(owner.0).unwrap();

      //Get the MouseClick and it's corresponding ray and convert them into a Destination
      let new_destination = Destination::from(mouseray.ray_ground_intersection());

      // If the distance between the current Position and the Destination is small run the pathing script
      if position.distance(&new_destination) < 100.0 {
        *destination = new_destination;
      } else {
        run_scripts(world, *owner);
        // let mut path = entity.get_component_mut::<Path>().unwrap();
        let mut path = world.get_component_mut::<Path>(owner.0).unwrap();
        if let Some(first_node) = path.next() {
          // let mut destination = entity.get_component_mut::<Destination>().unwrap();
          let mut destination = world.get_component_mut::<Destination>(owner.0).unwrap();
          *destination = first_node;
        }
      }
    }
  });
}

///Run a unit's pathing script [`MovementScript`]s.
pub fn run_scripts(world: &World, _owner: Owner) {
  let _lua = world.get_resource::<Rc<Lua>>().unwrap();
}
