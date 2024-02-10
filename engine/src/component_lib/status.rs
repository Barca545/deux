//Player State
//these probably need to hold a duration so the can be timed
#[derive(Debug, Clone, Copy)]
pub enum MovementState {
  DASHING,
  WALKING
}

#[derive(Debug, Clone, Copy)]
pub enum CrowdControlState {
  STUNNED,
  SLOWED,
  AIRBORNE
}

#[derive(Debug, Default, Clone)]
pub struct CrowdControlList(Vec<CrowdControlState>);