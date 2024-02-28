//Player State
//these probably need to hold a duration so the can be timed
#[derive(Debug, Clone, Copy)]
pub enum MovementState {
  Dashing,
  Walking,
  Blinking
}

#[derive(Debug, Default, Clone, Copy)]
pub enum PlayerState{
  #[default]
  Idle,
  Moving(MovementState),
  Attacking
}

#[derive(Debug, Clone, Copy)]
pub enum CrowdControlState {
  Stunned,
  Slowed,
  Airborne
}

#[derive(Debug, Default, Clone)]
pub struct CrowdControlList(Vec<CrowdControlState>);