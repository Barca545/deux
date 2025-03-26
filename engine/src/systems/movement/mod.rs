mod move_attacks;
mod movement;
mod update_hitbox;
mod update_path;
mod update_position;
mod update_velocity;

pub use self::{
  move_attacks::*, movement::*, update_hitbox::*, update_path::*, update_position::*,
  update_velocity::*,
};
