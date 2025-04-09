use crate::data_lib::{
  AbilityMap, Armor, AutoAttack, BecsId, Destination, MagicResist, MissleSpeed, Owner, Position, PreviousPosition, Target, Team, Velocity
};
use nina::{storage::Bundle, world::World};

// Refactor:
// -Will eventually need to factor in pen.
// -Need separate equation for AP damage and AD damage

///Calculate physical damage mitigation from [`Armor`].
pub fn calc_gross_physical_damage(damage:i32, armor:Armor) -> i32 {
  let resist_factor = 100.0 / (100.0 + armor.total() as f32);
  let post_resist_damage = damage as f32 * resist_factor;
  post_resist_damage as i32
}

///Calculate magic damage mitigation from [`MagicResist`].
pub fn calc_gross_magic_damage(damage:i32, magic_resist:MagicResist) -> i32 {
  let resist_factor = 100.0 / (100.0 + magic_resist.total() as f32);
  let post_resist_damage = damage as f32 * resist_factor;
  post_resist_damage as i32
}

///Checks whether the player and target are on the same team and whether the
/// player's attack cooldown is 0.0. Returns true if both conditions are
/// satisfied.
pub fn can_attack(world:&World, player:usize, target:usize) -> bool {
  let target_team = world.get_component::<Team>(target).unwrap();
  let player_team = world.get_component::<Team>(player).unwrap();
  *target_team != *player_team
}

///Returns a [`Bundle`] containing the data needed to spawn an auto attack
/// entity.
pub fn create_ranged_auto_attack(world:&World, owner:Owner, target:Target, ability_slot:u32) -> impl Bundle {
  let bundle;
  {
    //Get the owner's position
    let owner_position = world.get_component::<Position>(owner.id()).unwrap();

    //Create the projectile's position information
    let attack_position = Position(owner_position.0);
    let previous_attack_position = PreviousPosition(owner_position.0);

    //Get the target's position
    let destination = Destination::from(*world.get_component::<Position>(target.0.unwrap()).unwrap());

    //Create the projectile speed
    let speed = world.get_component::<MissleSpeed>(owner.id()).unwrap();

    //Calculate velocity
    let velocity = Velocity::new(&attack_position, &destination, &speed.total());

    //Get the mesh info
    let map = world.get_component::<AbilityMap>(owner.id()).unwrap();
    let info = map.get(ability_slot);

    bundle = (
      AutoAttack::default(),
      attack_position,
      previous_attack_position,
      *speed,
      velocity,
      info.model_id.unwrap(),
      owner,
      target
    );
    bundle
  }
}
