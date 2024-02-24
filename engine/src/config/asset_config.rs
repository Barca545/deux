use std::env;
use crate::filesystem::load_config;

pub fn asset_config(){
  let config = load_config().unwrap();

  env::set_var("RUST_BACKTRACE", "FULL");

  let shader_folder = config.get::<String>("shader_folder").unwrap();
  env::set_var("shader_folder", shader_folder);

  let texture_folder = config.get::<String>("texture_folder").unwrap();
  env::set_var("texture_folder", texture_folder);

  let model_folder = config.get::<String>("model_folder").unwrap();
  env::set_var("model_folder", model_folder);

  let champion_folder = config.get::<String>("champion_folder").unwrap();
  env::set_var("champion_folder", champion_folder); 

  let grid_folder = config.get::<String>("grid_folder").unwrap();
  env::set_var("grid_folder", grid_folder); 
}