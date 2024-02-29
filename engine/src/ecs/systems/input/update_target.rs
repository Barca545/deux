use crate::{
  component_lib::{Controllable, Cooldowns, Owner, Player, SelectionRadius, Target, Team},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  input::user_inputs::{FrameInputs, UserInput},
  physics::ray_aabb3d_collision_test
};

// Refactor
// -Merge and remove the selection logic. Just update the target.
//  Set to some if there is something under the MouseRay, otherwise set to none
// -Don't need to handle checking the target team here, handle it in lower level
// logic -Needs to select the entity that is in front, not just the first found
// -Needs to not overwrite a true
// -See if adding a states reduces the need for shennanigans with not moving if
// there is a target
// -Update target should also create a move event that runs if the target is
// none.  The movement system can then process the events and search for the
// move events and updat the destination accordingly -Can attack should go into
// some more general module with helper functions -Should be renamed to like
// process_mouseclick or something

pub fn update_target(world:&World) {
  let frame_inputs = world.get_resource_mut::<FrameInputs>().unwrap();
  if let Some(UserInput::MouseClick(mouse_ray)) = frame_inputs.get_input() {
    //Get the target of the controllable Player
    let mut query = world.query();
    let entities = query.with_component::<Player>().unwrap().with_component::<Controllable>().unwrap().run();
    let entity = &entities[0];
    let mut target = entity.get_component_mut::<Target>().unwrap();
    let mut new_target = Target(None);

    let mut query_targetables = world.query();
    let targetable_entities = query_targetables.with_component::<SelectionRadius>().unwrap().run();
    //Query all targetable entities.
    //If the MouseRay is hitting an entity, update the Controllable Player's
    //Target.
    for targetable_entity in targetable_entities {
      let hitbox = targetable_entity.get_component::<SelectionRadius>().unwrap();
      let hit_check = ray_aabb3d_collision_test(hitbox.0, mouse_ray.0);
      if hit_check {
        new_target = Target(Some(targetable_entity.id));
        *target = new_target;
        if can_attack(world, entity.id, targetable_entity.id) {
          let owner = Owner(entity.id);
          let mut queue = world.get_resource_mut::<GameEventQueue>().unwrap();
          queue.push(GameEvent::AutoAttackStart { owner });
        }
      }
      else if new_target.0 == None {
        *target = Target(None);
        let owner = Owner(entity.id);
        let mut queue = world.get_resource_mut::<GameEventQueue>().unwrap();
        queue.push(GameEvent::UpdateDestination { owner });
      }
    }
  }
}

///Checks whether the player and target are on the same team and whether the
/// player's attack cooldown is 0.0. Returns true if both conditions are
/// satisfied.
fn can_attack(world:&World, player:usize, target:usize) -> bool {
  let target_team = world.get_component::<Team>(target).unwrap();
  let player_team = world.get_component::<Team>(player).unwrap();
  let player_attack_cooldown = world.get_component::<Cooldowns>(player).unwrap().get_real_remaing("auto attack");
  *target_team != *player_team && player_attack_cooldown == 0.0
}
