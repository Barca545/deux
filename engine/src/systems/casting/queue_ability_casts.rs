use crate::{
  data_lib::{BecsId, CastQueue},
  event::{GameEvent, GameEventQueue}
};
use nina::world::World;

///Filters the [`GameEventQueue`] for and `AbilityStart` events.
/// Creates a `BufferedAbilityCast` from the `AbilityStart` event and adds it to
/// its owner's [`CastQueue`].
pub fn queue_ability_casts(world:&World) {
  //Find the AbilityStart events and convert them into AbilityCast
  let events = world.get_resource::<GameEventQueue>();
  events.process_events(|event| match event {
    GameEvent::AbilityStart(cast) => {
      //Add the BufferedAbilityCast to the owner's CastQueue
      let cast_queue = world.get_component_mut::<CastQueue>(cast.ability.owner.id()).unwrap();
      cast_queue.add(cast.clone())
    }
    _ => {}
  });
}
