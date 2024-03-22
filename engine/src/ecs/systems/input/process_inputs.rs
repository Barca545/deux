use crate::{
  component_lib::{Controllable, Owner, Player, SelectionRadius, Target},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  input::user_inputs::{FrameInputs, Keybind},
  math::MouseRay,
  physics::ray_aabb3d_collision_test,
  utility::can_attack,
};

// Refactor:
// -Needs to select the entity that is in front, not just the first found
// -Update target could actually be a stage in the mouse click input adding
// -Sending an attack command should be a different stage than setting the target?
// -Need to switch the player state at some point so the pending stuff is actually a cast time and not a buffered ability.

//Update target might be extraneous, what it can maybe do is update some AA target component
pub fn update_target(world: &World, entity: usize, mouse: MouseRay) {
  let mut target = world.get_component_mut::<Target>(entity).unwrap();

  //Query all targetable entities.
  //If the MouseRay is hitting an entity, update the Controllable Player's Target.
  let mut query_targetables = world.query();
  let targetable_entities = query_targetables.with_component::<SelectionRadius>().unwrap().run();
  for targetable_entity in targetable_entities {
    let hitbox = targetable_entity.get_component::<SelectionRadius>().unwrap();
    //Set a target and queue an auto attack if it is an enemy
    if ray_aabb3d_collision_test(hitbox.0, mouse.0) {
      *target = Target::new(targetable_entity.id);
      if can_attack(world, entity, targetable_entity.id) {
        let owner = Owner::new(entity);
        let mut queue = world.get_resource_mut::<GameEventQueue>().unwrap();
        //Add the event to the events' pending field
        queue.push(GameEvent::AbilityStart {
          owner,
          ability_slot: 12,
          mouse,
        });
      }
      //Return early if a target is found
      return;
    }
  }
  *target = Target(None);
  let owner = Owner::new(entity);
  let mut queue = world.get_resource_mut::<GameEventQueue>().unwrap();
  queue.push(GameEvent::UpdateDestination { owner, mouse });
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
      //Create the ability 1 start event
      events.push(GameEvent::AbilityStart {
        owner: Owner::new(player_id),
        ability_slot: 1,
        mouse: input.mouse,
      })
    }
    Keybind::AbilityTwo => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Create the ability 2 start event
      events.push(GameEvent::AbilityStart {
        owner: Owner::new(player_id),
        ability_slot: 2,
        mouse: input.mouse,
      })
    }
    Keybind::AbilityThree => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Create the ability 3 start event
      events.push(GameEvent::AbilityStart {
        owner: Owner::new(player_id),
        ability_slot: 3,
        mouse: input.mouse,
      })
    }
    Keybind::AbilityFour => {
      let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
      //Create the ability 4 start event
      events.push(GameEvent::AbilityStart {
        owner: Owner::new(player_id),
        ability_slot: 4,
        mouse: input.mouse,
      })
    }
  });
}
