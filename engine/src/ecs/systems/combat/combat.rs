use super::{auto_attack_start::auto_attack_start, resolve_attacks::process_hits};
use crate::{
  component_lib::{AbilityMap, Gold, Player, KDA},
  ecs::{world_resources::DebugElements, World},
  event::{GameEvent, GameEventQueue},
};

// Refactor:
// -Update to also handle Auto attack casting
// -Need functionality to add/replace abilties in the map
// -Need the ability to control the logic of abilties from scripts

pub fn ability_start(world: &mut World) {
  let events = world.get_resource_mut::<GameEventQueue>().unwrap();
  events.process_events(|event| {
    if let GameEvent::AbilityStart { ability_type, owner } = event {
      let map = world.get_component::<AbilityMap>(owner.0).unwrap();
      let ability = map.get(*ability_type);
      dbg!(ability);
    }
  });
}

pub fn combat(world: &mut World) {
  auto_attack_start(world);
  ability_start(world);
  process_hits(world);
  //Only run if debug attacks is enabled
  let debug = world.get_resource::<DebugElements>().unwrap();
  if debug.attacks {
    debug_combat(world);
  }
}

fn debug_combat(world: &World) {
  let mut query = world.query();

  let entites = query.with_component::<KDA>().unwrap().run();
  for entity in entites {
    //debugs the info of the dummy a player is hitting
    if let Err(_player) = entity.get_component::<Player>() {
      let kda = entity.get_component::<KDA>().unwrap();
      let gold = entity.get_component::<Gold>().unwrap();
      dbg!(entity.id);
      dbg!(kda);
      dbg!(gold);
    }
  }
}
