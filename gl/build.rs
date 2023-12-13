extern crate gl_generator;

use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};
use std::{env, fs::File, path::Path};

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

	//what is this line doing
	let registry = Registry::new(
		Api::Gl,
		(3, 3),
		Profile::Core,
		Fallbacks::All,
		["GL_NV_command_list"],
	);

	if env::var("CARGO_FEATURE_DEBUG").is_ok() {
		registry
			.write_bindings(DebugStructGenerator, &mut file_gl)
			.unwrap();
	} else {
		registry
			.write_bindings(StructGenerator, &mut file_gl)
			.unwrap();
	}
}
