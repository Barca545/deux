// Refactor:
// -Will eventually need to factor in pen I suppose.
// -Need separate equation for AP damage and AD damage

use crate::{
  component_lib::{AutoAttack, AutoAttackMesh, Destination, MissleSpeed, Owner, Position, PreviousPosition, SkinnedMesh, Target, Velocity},
  ecs::{Bundle, World},
};

///Calculate damage mitigation from resistance
pub fn calc_post_mitigation_damage(damage: u32, resist: u32) -> u32 {
  let resist_factor = 100 / (100 + resist);
  let post_resist_damage = damage * resist_factor;
  post_resist_damage
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
