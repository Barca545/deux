mod combat;
mod input;
mod mouse;
mod movement;
mod playerstate;
mod render;
mod selection;
mod spawn;

pub use self::{
  // selection::update_selection,
  combat::combat,
  input::{process_inputs, update_mouseray::update_mouseray, update_target},
  movement::{movement, update_destination},
  render::render,
  spawn::{register_components, spawn_dummy, spawn_enviroment, spawn_player},
};
