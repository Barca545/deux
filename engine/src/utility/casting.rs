use crate::{
  component_lib::{Cooldowns, PlayerState, SpellResource},
  ecs::World,
};

// Refactor:
// -Figure out if it is possible to pass a QueryEntity between systems
// -Rework Cooldowns to take TypeId or something

///Checks whether the requested cooldown is 0.0.
pub fn off_cooldown(world: &World, entity: usize, ability_name: &str) -> bool {
  let cooldowns = world.get_component::<Cooldowns>(entity).unwrap();
  cooldowns.is_zero(ability_name)
}

///Checks whether an entity has enough [`SpellResource`] to complete an action.
pub fn has_resource(world: &World, entity: usize, cost: u32) -> bool {
  let resource = world.get_component::<SpellResource>(entity).unwrap();
  resource.0 >= cost
}

///Checks whether an entity's [`PlayerState`] is `Unoccupied`.
pub fn is_unoccupied(world: &World, entity: usize) -> bool {
  let state = world.get_component::<PlayerState>(entity).unwrap();
  match *state {
    PlayerState::Unoccupied => true,
    _ => false,
  }
}
