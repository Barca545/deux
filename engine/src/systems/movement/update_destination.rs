use crate::{
  data_lib::{BecsId, Destination, Owner, Path, Position},
  event::{GameEvent, GameEventQueue}
};
use nina::world::World;

//Refactor
// -The mouse intersection should be calculated in the same place where the
// MouseRay is set -This should only run if the selection test says nothing is
// selected  needs selection needs to run first and do the AABB test
// -Revisit the limit before using the pathing algorithm once the arena is added
// -Add attack move as a toggle (If there is a target, move into attack range
// and no further) -Update the run scripts function to use the new scripting set
// up

///Updates the [`Destination`] component for the entity marked with the
/// [`Controllable`] component. If the destination is less than 100 units away,
/// sets the `Destination` to the location of the mouse click.
/// Otherwise calculates a [`Path`] using the pathing script.
/// Does not update entity's [`Velocity`] components.
pub fn update_destination(world:&World) {
  let events = world.get_resource::<GameEventQueue>();
  events.process_events(|event| {
    if let GameEvent::UpdateDestination { owner, mouse } = event {
      let destination = world.get_component_mut::<Destination>(owner.id()).unwrap();
      let position = world.get_component_mut::<Position>(owner.id()).unwrap();

      // Get the MouseClick and it's corresponding ray and convert them into a
      // Destination
      let new_destination = Destination::from(mouse.ray_ground_intersection());

      // If the distance between the current Position and the Destination is small run
      // the pathing script
      if position.distance(&new_destination) < 100.0 {
        *destination = new_destination;
      }
      // else {
      //   run_scripts(world, *owner);
      //   // let mut path = entity.get_component_mut::<Path>().unwrap();
      //   let mut path = world.get_component_mut::<Path>(owner.id()).unwrap();
      //   if let Some(first_node) = path.next() {
      //     // let mut destination =
      // entity.get_component_mut::<Destination>().unwrap();     let mut
      // destination =
      // world.get_component_mut::<Destination>(owner.id()).unwrap();
      //     *destination = first_node;
      //   }
      // }
    }
  });
}
