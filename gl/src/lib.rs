mod bindings {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindings::*;

use::std::{ops::Deref,rc::Rc};

#[derive(Clone)]
pub struct Gl {
  pub inner: Rc<bindings::Gl>,
}

impl Gl {
  pub fn load_with(loadfn: &mut dyn FnMut(&'static str) -> *const types::GLvoid) -> Gl{
      Gl{
        inner: Rc::new(bindings::Gl::load_with(loadfn))
      }
  }
}

impl Deref for Gl {
  type Target = bindings::Gl;

  fn deref(&self) -> &bindings::Gl {
    &self.inner
  }
}

// #[test]
// fn pring_gl_size(){
//   println!("size of Gl: {}", std::mem::size_of_val(&gl));
// }