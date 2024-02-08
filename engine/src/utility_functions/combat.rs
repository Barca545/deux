//Damage calculation function
pub fn calc_post_mitigation_damage(damage:i32, resist:i32)->i32{
  let resist_factor = 100/(100+resist);
  let post_resist_damage = damage * resist_factor;
  post_resist_damage
}