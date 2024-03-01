use std::any::Any;

use crate::{
  component_lib::{
    AutoAttack, AutoAttackMesh, Cooldowns, Destination, MissleSpeed, Owner, PlayerState, Position, PreviousPosition, SkinnedMesh, Target, Velocity,
  },
  ecs::{bundle::Bundle, World},
  event::{AutoAttack as AutoAttackId, GameEvent, GameEventQueue},
};

// Refactor:
// -This system will eventually need to check if the target is still valid since a target can be valid when the auto process is started but invalid when it's time to actually start the auto.
//  Maybe make a special function for checking a target's validity
// -Consider stealing this spawner implementation to make a more fully featured comand buffer
// -System that registers attacks needs to:
// -Check if the target is an enemy and later after this is added: alive
// -Start the auto wind up -> this actually needs to be a different event, like queue auto is distinct from start auto
// -Refactor this queries for AbilityStart now and AAs are considered abilities

///System for beginning auto attacks.
/// Queries the [`GameEventQueue`] for `AutoAttackStart` events.
/// For each `AutoAttackStart` found, this system calls the any scripts associated with the [`Owner`]'s auto attack start up process.
pub fn auto_attack_start(world: &mut World) {
  let mut spawner = Vec::new();
  {
    let events = world.get_resource_mut::<GameEventQueue>().unwrap();

    //Process any AutoAttackStart events
    events.process_events(|event| {
      if let GameEvent::AbilityStart { ability_type, owner } = event {
        if *ability_type == AutoAttackId.type_id() {
          //Get the target
          let target = world.get_component::<Target>(owner.0).unwrap();

          //Spawn the auto attack and push it into the spawner
          let auto_attack = create_auto_attack(world, *owner, *target);
          spawner.push(auto_attack);

          //Reset the auto attack cooldown
          let mut cooldowns = world.get_component_mut::<Cooldowns>(owner.0).unwrap();
          cooldowns.reset("auto attack");

          //Set the player state to attacking
          let mut player_state = world.get_component_mut::<PlayerState>(owner.0).unwrap();
          *player_state = PlayerState::Attacking;
        }
      }
    });
  }
  for bundle in spawner {
    world.create_entity().with_components(bundle).unwrap();
  }
}

///Returns a `Bundle` containing the data needed to spawn an auto attack entity.
fn create_auto_attack(world: &World, owner: Owner, target: Target) -> impl Bundle {
  let bundle;
  {
    //Get the owner's position
    let owner_position = world.get_component::<Position>(owner.0).unwrap();

    //Create the projectile's position information
    let attack_position = Position(owner_position.0);
    let previous_attack_position = PreviousPosition(owner_position.0);

    //Get the target's position
    let destination = Destination::from(*world.get_component::<Position>(target.0.unwrap()).unwrap());

    //Create the projectile speed
    let speed = world.get_component::<MissleSpeed>(owner.0).unwrap();

    //Calculate velocity
    let velocity = Velocity::new(&attack_position, &destination, &speed.0);

    //Get the mesh info
    let auto_attack_mesh = world.get_component::<AutoAttackMesh>(owner.0).unwrap();

    bundle = (
      AutoAttack::default(),
      attack_position,
      previous_attack_position,
      *speed,
      velocity,
      SkinnedMesh::from(auto_attack_mesh.clone()),
      owner,
      target,
    );
    bundle
  }
}
