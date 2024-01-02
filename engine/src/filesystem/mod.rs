mod loader;
mod parser;
mod champion;

pub use self::{
  loader::{load_cstring, load_image, load_shader, load_object, load_champion_json},
  parser::create_whitespace_cstring
};

//rig up the ability to load from JSONs
//rig up the ability to load obj files
