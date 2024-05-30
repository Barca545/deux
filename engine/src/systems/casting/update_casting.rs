use nina::world::World;

use crate::{
  data_lib::{CastQueue, Casting, PlayerState},
  time::ServerTime
};

// Refactor:
// -Once I migrate to BECS, I will need to use the command buffer to add the
// casting component -If there is a bottleneck experiment with deleting casting
// in this subsystem, try it in the cast abilities subsystem

///Query all all entities with [`CastQueue`]s.
/// Check if the entity has a [`Casting`] component.
/// If the ability in the `Casting` component is finished channeling, move the
/// next [`BufferedAbilityCast`] out of the `CastQueue` and into the `Casting`
/// component.
pub fn update_casting(world:&mut World) {
  // let mut cmds = CommandBuffer::new();
  // let mut add = Vec::new();

  // //Remove the Casting component from all entities who have cast an ability
  // let mut query = world.query();
  // let entities = query.with_component::<Casting>().unwrap().run();
  // for entity in entities {
  //   let casting = entity.get_component_mut::<Casting>().unwrap();
  //   if casting.is_done() {
  //     //Buffer the command to delete the Casting component to the command
  // buffer     cmds.remove_component::<Casting>(entity.id);
  //   }
  // }

  // //Run the commands to remove the Casting component
  // cmds.run(world);

  // let mut query = world.query();
  // let entities =
  // query.with_component::<CastQueue>().unwrap().
  // without_component::<Casting>().unwrap().run();

  // for entity in entities {
  //   //Try to get the next buffered ability and create a new casting component
  // from   // it
  //   let mut cast_queue = entity.get_component_mut::<CastQueue>().unwrap();
  //   if let Some(buffered_cast) = cast_queue.next() {
  //     //If there is another buffered cast, create a new Casting component
  //     let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
  //     let cast = Casting::new(buffered_cast, &mut server_time);
  //     add.push((entity.id, cast));

  //     //Update the PlayerState to channeling
  //     let mut state = entity.get_component_mut::<PlayerState>().unwrap();
  //     *state = PlayerState::Channeling
  //   }
  // }

  // //Run the commands to add the casting component
  // for (entity, casting) in add {
  //   world.add_component(entity, casting).unwrap();
  // }
}
