use crate::{
  data_lib::{AbilityMap, Dead, PlayerState, SpellResource, Team},
  time::Timer
};
use nina::world::World;

// Refactor:
// -Figure out if it is possible to pass a QueryEntity between systems
// -Rework Cooldowns to take TypeId or something
// -These should create debug GameEvents like Oom{player, ...} if they fail
// -Off cooldown might not need to be a utility function since the ability cast
// holds the cooldown so it's easier to check that way

///Returns true if the requested slot's `Cooldown` is 0.0.
pub fn off_cooldown(world:&World, entity:usize, ability_slot:u32) -> bool {
  let ability_map = world.get_component::<AbilityMap>(entity).unwrap();
  let cooldown = ability_map.get_cooldown(ability_slot);
  cooldown.is_zero()
}

///Returns true if an entity has enough [`SpellResource`] to complete an
/// action.
pub fn has_resource(world:&World, entity:usize, cost:i32) -> bool {
  let resource = world.get_component::<SpellResource>(entity).unwrap();
  resource.remaining() >= cost
}

///Returns true if an entity's [`PlayerState`] is `Unoccupied`.
pub fn is_unoccupied(world:&World, entity:usize) -> bool {
  let state = world.get_component::<PlayerState>(entity).unwrap();
  match *state {
    PlayerState::Unoccupied => true,
    _ => false
  }
}

///Returns true if a target entity has the [`Dead`] component.
pub fn target_is_alive(world:&World, target:usize) -> bool {
  //Try to get the Dead component from a target. Returns false if the target does
  // not have the Dead component.
  world.get_component::<Dead>(target).is_err()
}

/// Returns true if the target is not on the same [`Team`] as the entity.
pub fn is_enemy(world:&World, entity:usize, target:usize) -> bool {
  let entity_team = world.get_component::<Team>(entity).unwrap();
  let target_team = world.get_component::<Team>(target).unwrap();
  *entity_team != *target_team
}

/// Returns true if the target's [`Team`] is neutral.
pub fn is_neutral(world:&World, target:usize) -> bool {
  let target_team = world.get_component::<Team>(target).unwrap();
  *target_team == Team::Neutral
}
