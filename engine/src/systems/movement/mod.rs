mod move_attacks;
mod movement;
mod update_destination;
mod update_hitbox;
mod update_path;
mod update_position;
mod update_velocity;

pub use self::{
  movement::movement, update_destination::update_destination, update_hitbox::update_hitbox, update_path::update_path,
  update_position::update_position, update_velocity::update_velocity,
};
