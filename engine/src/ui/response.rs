// use crate::view::Mesh;
use glfw::RenderContext;

pub struct Response {
  id: usize,
  // mesh: Mesh,
  state: WidgetState,
  ctx: RenderContext,
}

pub struct WidgetState {
  pub clicked: bool,
  pub hovered: bool,
  pub focused: bool,
  //field for any string in the widget?
}

impl WidgetState {
  ///Create a new [`WidgetState`] with all fields set to `false`.
  pub fn new() -> Self {
    WidgetState {
      clicked: false,
      hovered: false,
      focused: false,
    }
  }
}

//Response is how the UI communicates with the rest of the engine
//widget state can be used to comunicate

// systems need to be linked to the widget they update

//Do I have to make a new mesh every time?
// Is it possible to just draw the data directly?
