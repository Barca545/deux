use sdl2::{keyboard::Keycode, mouse::MouseState};
use std::{
  collections::HashMap,
  hash::Hash,
  ops::{Deref, DerefMut},
};

#[derive(Debug,)]
pub struct Keybinds(HashMap<Keycode, InputType,>,);

impl Keybinds {
  /// Create a new `Keybinds`.
  pub fn new() -> Self {
    Keybinds(HashMap::new(),)
  }

  /// Build the [`Input`] that corresponds to a given [`Keycode`], `mouse` and
  /// [`ButtonAction`]. Return `None` if no [`Input`] was registered for a
  /// `Keycode`.
  pub fn build_input(
    &self,
    key: &Keycode,
    mouse: MouseState,
    action: ButtonAction,
  ) -> Option<Input,> {
    match self.get(key,) {
      Some(&ty,) => Some(Input { ty, mouse, action, },),
      None => None,
    }
  }
}

impl Default for Keybinds {
  fn default() -> Self {
    let mut keybinds = Keybinds(HashMap::new(),);
    keybinds.insert(Keycode::from_name("W",).unwrap(), InputType::MoveUp,);
    keybinds.insert(Keycode::from_name("A",).unwrap(), InputType::MoveLeft,);
    keybinds.insert(Keycode::from_name("S",).unwrap(), InputType::MoveDown,);
    keybinds.insert(Keycode::from_name("D",).unwrap(), InputType::MoveRight,);
    keybinds
  }
}

impl Deref for Keybinds {
  type Target = HashMap<Keycode, InputType,>;

  fn deref(&self,) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Keybinds {
  fn deref_mut(&mut self,) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord,)]
pub enum InputType {
  // Movement
  MoveUp,
  MoveDown,
  MoveLeft,
  MoveRight,
  // Mouse input
  Mouse,
}

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum ButtonAction {
  Press,
  Release,
  None,
}

#[derive(Debug, Clone, Copy,)]
pub struct Input {
  /// The [type](InputType) of the `Input`.
  pub(crate) ty: InputType,
  // TODO: I am not 100% sure it is called viewspace
  /// The [state](https://wiki.libsdl.org/SDL2/SDL_GetMouseState) and [view space](https://learnopengl.com/Getting-started/Coordinate-Systems) position of the mouse during the `Input`.
  pub(crate) mouse: MouseState,
  /// Whether the `Input` is a press or release.
  pub(crate) action: ButtonAction,
  // TODO: I think inputs should also track the tick they were omitted. I am debating if there
  // should be some sort of trait for this?
}

impl Input {
  #[inline(always)]
  pub fn ty(&self,) -> &InputType {
    &self.ty
  }

  #[inline(always)]
  pub fn mouse(&self,) -> &MouseState {
    &self.mouse
  }

  #[inline(always)]
  pub fn action(&self,) -> &ButtonAction {
    &self.action
  }
}

impl Hash for Input {
  fn hash<H: std::hash::Hasher,>(&self, state: &mut H,) {
    self.ty.hash(state,);
  }
}

impl PartialEq for Input {
  fn eq(&self, other: &Self,) -> bool {
    self.ty == other.ty
  }
}

impl Eq for Input {}
