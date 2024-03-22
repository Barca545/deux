use super::{cast_abilites::cast_abilites, queue_ability_casts::queue_ability_casts, update_casting::update_casting};
use crate::ecs::World;

pub fn casting(world: &mut World) {
  queue_ability_casts(world);
  cast_abilites(world);
  update_casting(world);
}
