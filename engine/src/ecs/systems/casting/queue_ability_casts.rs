use crate::{
  component_lib::{AbilityMap, BecsId, BufferedAbilityCast, CastQueue},
  ecs::World,
  event::{GameEvent, GameEventQueue},
};

///Filters the [`GameEventQueue`] for and `AbilityStart` events.
/// Creates a [`BufferedAbilityCast`] from the `AbilityStart` event and adds it to its owner's [`CastQueue`].
pub fn queue_ability_casts(world: &World) {
  //Find the AbilityStart events and convert them into AbilityCast
  let events = world.get_resource::<GameEventQueue>().unwrap();
  events.process_events(|event| {
    if let GameEvent::AbilityStart { owner, ability_slot, mouse } = event {
      //Convert the AbilityStart event into a BufferedAbilityCast
      let ability_map = world.get_component::<AbilityMap>(owner.id()).unwrap();
      if let Some(buffered_cast) = ability_map.create_ability_cast(ability_slot, owner, *mouse) {
        //Add the BufferedAbilityCast to the owner's CastQueue
        let mut cast_queue = world.get_component_mut::<CastQueue>(owner.id()).unwrap();
        cast_queue.add(buffered_cast)
      }
    }
  });
}
