use std::any::Any;

use crate::{
  component_lib::{Controllable, Cooldowns, Owner, Player, SelectionRadius, Target},
  ecs::World,
  event::{AbilityFour, AbilityOne, AbilityThree, AbilityTwo, AutoAttack, GameEvent, GameEventQueue},
  input::user_inputs::{FrameInputs, Keybind},
  math::MouseRay,
  physics::ray_aabb3d_collision_test,
  time::ServerTime,
  utility::can_attack,
};

// Refactor:
// -Needs to select the entity that is in front, not just the first found
// -Update target could actually be a stage in the mouse click input adding
// -Sending an attack command should be a different stage than setting the target?
// -Need to switch the player state at some point so the pending stuff is actually a cast time and not a buffered ability.

//Update target might be extraneous, what it can maybe do is update some AA target component
pub fn update_target(world: &World, entity: usize, mouseray: MouseRay) {
  let mut target = world.get_component_mut::<Target>(entity).unwrap();

  //Query all targetable entities.
  //If the MouseRay is hitting an entity, update the Controllable Player's Target.
  let mut query_targetables = world.query();
  let targetable_entities = query_targetables.with_component::<SelectionRadius>().unwrap().run();
  for targetable_entity in targetable_entities {
    let hitbox = targetable_entity.get_component::<SelectionRadius>().unwrap();
    //Set a target and queue an auto attack if it is an enemy
    if ray_aabb3d_collision_test(hitbox.0, mouseray.0) {
      *target = Target::new(targetable_entity.id);
      if can_attack(world, entity, targetable_entity.id) {
        let owner = Owner(entity);
        let mut queue = world.get_resource_mut::<GameEventQueue>().unwrap();
        //Get the cast time
        let mut server_time = world.get_resource_mut::<ServerTime>().unwrap();
        let timer = 10.0;
        //Add the event to the events' pending field
        queue.push_pending(
          timer,
          &mut server_time,
          GameEvent::AbilityStart {
            mouseray,
            ability_type: AutoAttack.type_id(),
            owner,
          },
        );
      }
      //Return early if a target is found
      return;
    }
  }
  *target = Target(None);
  let owner = Owner(entity);
  let mut queue = world.get_resource_mut::<GameEventQueue>().unwrap();
  queue.push(GameEvent::UpdateDestination { mouseray, owner });
}

///Converts [`FrameInputs`] into [`GameEvent`]s.
/// Places the created `GameEvent` into the `pending` field of the [`GameEventQueue`] with a wind up timer based on the event's cast time.
pub fn process_inputs(world: &World) {
  //Get the Player's ID
  let mut query = world.query();
  let entities = query.with_component::<Player>().unwrap().with_component::<Controllable>().unwrap().run();
  let entity = &entities[0];
  let player_id = entity.id;

  let inputs = world.get_resource_mut::<FrameInputs>().unwrap();
  inputs.process_inputs(|input| match input.keybind {
    Keybind::MouseClick => update_target(world, player_id, input.mouse),
    Keybind::AbilityOne => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Get the cast time
      let cooldowns = entity.get_component::<Cooldowns>().unwrap();
      let timer = cooldowns.get_cooldown("auto cast time");
      //Add the event to the events' pending field
      events.push(
        // timer,
        GameEvent::AbilityStart {
          mouseray: input.mouse,
          ability_type: AbilityOne.type_id(),
          owner: Owner(player_id),
        },
      )
    }
    Keybind::AbilityTwo => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Create the ability 2 start event
      events.push(GameEvent::AbilityStart {
        mouseray: input.mouse,
        ability_type: AbilityTwo.type_id(),
        owner: Owner(player_id),
      })
    }
    Keybind::AbilityThree => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Create the ability 3 start event
      events.push(GameEvent::AbilityStart {
        mouseray: input.mouse,
        ability_type: AbilityThree.type_id(),
        owner: Owner(player_id),
      })
    }
    Keybind::AbilityFour => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Create the ability 4 start event
      events.push(GameEvent::AbilityStart {
        mouseray: input.mouse,
        ability_type: AbilityFour.type_id(),
        owner: Owner(player_id),
      })
    }
  });
}
