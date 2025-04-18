#![feature(path_add_extension)]
pub mod render;
pub mod systems;
mod update;
pub mod utils;

pub use update::update;
