use super::Cooldown;

//vertical knockback = knock up?
pub struct KnockBack {}

//Some can just be tags
pub struct Rooted {
  timer: Cooldown,
}

pub struct Stunned {
  timer: Cooldown,
}

pub struct Accelerated;

//have some system that loops through them and
