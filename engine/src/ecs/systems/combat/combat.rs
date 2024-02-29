use super::{
  auto_attack_start::auto_attack_start,
  resolve_attacks::{process_hits, resolve_attacks},
};
use crate::{
  component_lib::{Gold, Player, KDA},
  ecs::{world_resources::DebugElements, World},
};
// Refactor:
// -Scripts shouldn't run each frame.
// -Scripts need some tag or something that indicate when they should run.
// -Moving the attacks should possible be part of the move system

pub fn combat(world: &mut World) {
  auto_attack_start(world);
  //Look for an attackhit event and resolve them and any onhit scripts
  //Maybe resolve events is a separate system and this goes there
  resolve_attacks(world);
  process_hits(world);
  //only run if debug attacks is enabled
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
