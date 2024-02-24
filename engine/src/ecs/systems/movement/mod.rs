mod update_destination;
mod update_position;
mod update_velocity;
mod update_hitbox;
mod update_path;
mod movement;

pub use self::{
  update_destination::update_destination,
  update_position::update_position,
  update_hitbox::update_hitbox,
  update_path::update_path,
  update_velocity::update_velocity,
  movement::movement
};