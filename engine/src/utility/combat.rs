// Refactor:
// -Will eventually need to factor in pen I suppose.
// -Need separate equation for AP damage and AD damage

///Calculate damage mitigation from resistance
pub fn calc_post_mitigation_damage(damage: u32, resist: u32) -> u32 {
  let resist_factor = 100 / (100 + resist);
  let post_resist_damage = damage * resist_factor;
  post_resist_damage
}
