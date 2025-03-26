use crate::{
  data_lib::{AbilityMap, Controllable, Owner, Player, SelectionRadius, Target},
  event::{GameEvent, GameEventQueue},
  input::user_inputs::{FrameInputs, KeyAction, Keybind, PlayerInputs},
  math::MouseRay,
  physics::ray_aabb3d_collision_test,
  utility::can_attack,
};
use nina::world::World;

// Refactor:
// -Update target could actually be a stage in the mouse click input adding

//Update target might be extraneous, what it can maybe do is update some AA
// target component
pub fn update_target(world:&World, entity:usize, mouse:MouseRay,) {
  let target = world.get_component_mut::<Target>(entity,).unwrap();

  //Query all targetable entities.
  //If the MouseRay is hitting an entity, update the Controllable Player's
  // Target.
  let mut query_targetables = world.query();
  let targetable_entities = query_targetables
    .with_component::<SelectionRadius>()
    .unwrap()
    .run();
  for targetable_entity in targetable_entities {
    let hitbox = targetable_entity
      .get_component::<SelectionRadius>()
      .unwrap();
    //Set a target and queue an auto attack if it is an enemy
    // TODO: This is code to handle auto attacking from when I still wanted to make
    // an RTS/MOBA but now that I am leaning towards a <Vampire survivors> that is
    // not how attacks work
    if ray_aabb3d_collision_test(hitbox.0, mouse.0,) {
      *target = Target::new(targetable_entity.id,);
      if can_attack(world, entity, targetable_entity.id,) {
        let events = world.get_resource_mut::<GameEventQueue>();

        let ability_map = world.get_component::<AbilityMap>(entity,).unwrap();
        if let Some(buffered_cast,) =
          ability_map.create_ability_cast(12, Owner::new(entity,), mouse, *target,)
        {
          events.push(GameEvent::AbilityStart(buffered_cast,),);
        }
      }
      //Return early if a target is found
      return;
    }
  }
  *target = Target(None,);
}

/// Converts [`FrameInputs`] into [`GameEvent`]s.
/// Places the created `GameEvent` into the `pending` field of the
/// [`GameEventQueue`] with a wind up timer based on the event's cast time.
pub fn process_inputs(world:&World,) {
  // Get the Player's ID
  let mut query = world.query();
  let entities = query
    .with_component::<Player>()
    .unwrap()
    .with_component::<Controllable>()
    .unwrap()
    .run();
  let entity = &entities[0];
  let player_id = entity.id;

  let inputs = world.get_resource_mut::<FrameInputs>();
  // Only update if there are inputs
  let player_input_state = world.get_resource_mut::<PlayerInputs>();
  if !inputs.is_empty() {
    inputs.process_inputs(|input| match (input.keybind, input.action,) {
      // Handle key presses
      (Keybind::MouseClick, _,) => update_target(world, player_id, input.mouse,),
      (Keybind::Up, Some(KeyAction::Press,),) => player_input_state.up = true,
      (Keybind::Down, Some(KeyAction::Press,),) => player_input_state.down = true,
      (Keybind::Left, Some(KeyAction::Press,),) => player_input_state.left = true,
      (Keybind::Right, Some(KeyAction::Press,),) => player_input_state.right = true,
      // Handle key releases
      (Keybind::Up, Some(KeyAction::Release,),) => player_input_state.up = false,
      (Keybind::Down, Some(KeyAction::Release,),) => player_input_state.down = false,
      (Keybind::Left, Some(KeyAction::Release,),) => player_input_state.left = false,
      (Keybind::Right, Some(KeyAction::Release,),) => player_input_state.right = false,
      _ => todo!(),
    },);
  }
}
