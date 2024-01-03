mod spawn_player;
mod spawn_enviroment;
mod register_components;

pub use self::{
  spawn_player::spawn_player,
  spawn_enviroment::spawn_enviroment,
  register_components::register_components
};