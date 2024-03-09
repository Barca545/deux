use crate::{
  component_lib::{Cooldowns, Dead, PlayerState, SpellResource, Team},
  ecs::World,
};

// Refactor:
// -Figure out if it is possible to pass a QueryEntity between systems
// -Rework Cooldowns to take TypeId or something
// -These should create debug GameEvents like Oom{player, ...} if they fail

///Returns true if the requested cooldown is 0.0.
pub fn off_cooldown(world: &World, entity: usize, ability_name: String) -> bool {
  let cooldowns = world.get_component::<Cooldowns>(entity).unwrap();
  cooldowns.is_zero(ability_name.as_str())
}

///Returns true if an entity has enough [`SpellResource`] to complete an action.
pub fn has_resource(world: &World, entity: usize, cost: i32) -> bool {
  let resource = world.get_component::<SpellResource>(entity).unwrap();
  resource.remaining() >= cost
}

///Returns true if an entity's [`PlayerState`] is `Unoccupied`.
pub fn is_unoccupied(world: &World, entity: usize) -> bool {
  let state = world.get_component::<PlayerState>(entity).unwrap();
  match *state {
    PlayerState::Unoccupied => true,
    _ => false,
  }
}

///Returns true if a target entity has the [`Dead`] component.
pub fn target_is_alive(world: &World, target: usize) -> bool {
  //Try to get the Dead component from a target. Returns false if the target does not have the Dead component.
  world.get_component::<Dead>(target).is_err()
}

/// Returns true if the target is not on the same [`Team`] as the entity.
pub fn is_enemy(world: &World, entity: usize, target: usize) -> bool {
  let entity_team = world.get_component::<Team>(entity).unwrap();
  let target_team = world.get_component::<Team>(target).unwrap();
  *entity_team != *target_team
}

/// Returns true if the target's [`Team`] is neutral.
pub fn is_neutral(world: &World, target: usize) -> bool {
  let target_team = world.get_component::<Team>(target).unwrap();
  *target_team == Team::Neutral
}
