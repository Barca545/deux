mod loader;
mod parser;

pub use self::{
  loader::{load_cstring, load_image, load_shader, load_object},
  parser::create_whitespace_cstring
};

//rig up the ability to load from JSONs
//rig up the ability to load obj files
