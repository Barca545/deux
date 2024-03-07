// Refactor:
// -Will eventually need to factor in pen.
// -Need separate equation for AP damage and AD damage

use crate::{
  component_lib::{AutoAttack, AutoAttackMesh, Cooldowns, Destination, MissleSpeed, Owner, Position, PreviousPosition, SkinnedMesh, Target, Team, Velocity},
  ecs::{Bundle, World},
};

///Calculate damage mitigation from resistance
pub fn calc_post_mitigation_damage(damage: i32, resist: i32) -> i32 {
  let resist_factor = 100.0 / (100.0 + resist as f32);
  let post_resist_damage = damage as f32 * resist_factor;
  post_resist_damage as i32
}

///Checks whether the player and target are on the same team and whether the
/// player's attack cooldown is 0.0. Returns true if both conditions are
/// satisfied.
pub fn can_attack(world: &World, player: usize, target: usize) -> bool {
  let target_team = world.get_component::<Team>(target).unwrap();
  let player_team = world.get_component::<Team>(player).unwrap();
  let player_attack_cooldown = world.get_component::<Cooldowns>(player).unwrap().get_real_remaing("auto attack");
  *target_team != *player_team && player_attack_cooldown == 0.0
}

///Returns a [`Bundle`] containing the data needed to spawn an auto attack entity.
pub fn create_ranged_auto_attack(world: &World, owner: Owner, target: Target) -> impl Bundle {
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
    let velocity = Velocity::new(&attack_position, &destination, &speed.total());

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
