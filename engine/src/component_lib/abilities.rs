use super::{Cooldown, Script};
use crate::event::{AbilityFour, AbilityOne, AbilityThree, AbilityTwo, AutoAttack};
use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

// Refactor:
// -This can't be indexed with type ids because they can't be easily serialized to lua
// -Ability map should probably hold the cooldown.
// -Don't call the Ability "Ability"

pub struct Ability {
  cooldown: Cooldown,
  script: Script,
}

pub struct AbilityMap {
  map: HashMap<TypeId, Script>,
}

impl AbilityMap {
  pub fn new(ability_1: Script, ability_2: Script, ability_3: Script, ability_4: Script, autoattack: Script) -> Self {
    let mut map = HashMap::new();
    map.insert(AbilityOne.type_id(), ability_1);
    map.insert(AbilityTwo.type_id(), ability_2);
    map.insert(AbilityThree.type_id(), ability_3);
    map.insert(AbilityFour.type_id(), ability_4);
    map.insert(AutoAttack.type_id(), autoattack);
    AbilityMap { map }
  }

  pub fn get(&self, id: TypeId) -> &Script {
    self.map.get(&id).unwrap()
  }
}
