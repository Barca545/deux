use crate::{
  ecs::world_resources::ScreenDimensions,
  math::Vec3,
  view::{
    render_gl::{draw_elements, Vertex},
    Mesh,
  },
};
use gl::Gl;

//Git Message:

//
// Refactored render system and pipeline

//  Refactor:
//  -BaseWidget needs a new name
//  -Update the documentation for the Orientation struct to mention offset is measured in percent.
//  -Add functionality for calculating top and bottom anchors
//  -width and height need to be scaled by something so they keep their proportion to their parent when the parent is rescaled
//  -Base widget needs to be calculated with respect to its parent not the screen (unless its parent is the screen)
//  -Need to write a new shader for the UI

///Indicates the widget's orientation with respect to an anchor in its parent.
/// For objects with no parent, the `Top` and `Bottom` are with respect to the screen.
pub enum VerticalAnchor {
  Center,
  Top,
  Bottom,
}

///Indicates the widget's orientation with respect to its parent.
/// For objects with no parent, the `Left` and `Right` are with respect to the screen.
pub enum HorizontalAnchor {
  Center,
  Left,
  Right,
}

pub struct Orientation {
  horizontal_anchor: HorizontalAnchor,
  horizontal_offset: f32,
  vertical_anchor: VerticalAnchor,
  vertical_offset: f32,
}

impl Orientation {
  pub fn new(
    horizontal_anchor: HorizontalAnchor, horizontal_offset: f32, vertical_anchor: VerticalAnchor, vertical_offset: f32, screen: &ScreenDimensions,
  ) -> Self {
    //Convert the offsets to normalized device coordinates(range [-1,1])
    let mut horizontal_offset = horizontal_offset;
    let mut vertical_offset = vertical_offset;

    //Calculate the offset as a percentage of the screen dimensions
    horizontal_offset = (horizontal_offset / 100.0) * screen.width as f32;
    vertical_offset = (vertical_offset / 100.0) * screen.height as f32;

    Orientation {
      horizontal_anchor,
      horizontal_offset,
      vertical_anchor,
      vertical_offset,
    }
  }
}

pub trait GuiElement {
  fn new(orientation: Orientation, height: f32, width: f32, gl: Gl, texture_name: &str, screen: ScreenDimensions) -> Self;
  fn set_height(&mut self, height: f32);
  fn set_width(&mut self, width: f32);
  fn get_height(&self) -> f32;
  fn get_width(&self) -> f32;

  fn draw(&self);
  fn destroy(&self);

  fn add_child(&mut self, child: impl GuiElement);
}

pub struct BaseWidget {
  //Identification
  id: usize,
  parent: usize,

  //Rectangle info
  height: f32,
  width: f32,
  position: Vec3,

  //Render info
  gl: Gl,
  mesh: Mesh,
}

impl GuiElement for BaseWidget {
  ///`width` and `height` are percentages of the parent component
  fn new(orientation: Orientation, height: f32, width: f32, gl: Gl, texture_name: &str, screen: ScreenDimensions) -> Self {
    let mut position = Vec3::default();
    let horizontal_offset = orientation.horizontal_offset;
    let vertical_offset = orientation.vertical_offset;

    //Calculate the left and right values of the rectangle's corners in normalized device coordinates (range [-1,1]) using the vertical orientation
    match orientation.horizontal_anchor {
      HorizontalAnchor::Center => {
        position.x = (screen.width as f32 / 2.0) + horizontal_offset;
      }
      HorizontalAnchor::Left => {
        position.x = horizontal_offset;
      }
      HorizontalAnchor::Right => {
        position.x = screen.width as f32 + horizontal_offset;
      }
    }

    //Calculate the position of the widget in normalized device coordinates (range [-1,1])
    match orientation.vertical_anchor {
      VerticalAnchor::Center => {
        position.y = (screen.height as f32 / 2.0) + vertical_offset;
      }
      VerticalAnchor::Top => {
        position.y = vertical_offset;
      }
      VerticalAnchor::Bottom => {
        position.y = screen.height as f32 + vertical_offset;
      }
    }

    //Calculate the x and y maxima
    let mut left = position.x - (width / 2.0);
    let mut right = position.x + (width / 2.0);
    let mut top = position.y - (height / 2.0);
    let mut bottom = position.y + (height / 2.0);
    let z = -1.0;

    //Ensure shape is within screen bounds

    //Center the widget horizontally if it is wider than its parent
    if width > screen.width as f32 {
      let x = screen.width as f32 / 2.0;
      left = x - (width / 2.0);
      right = x + (width / 2.0);
    }
    //Shift the widget right if it would exceed the left bound of its parent
    else if left < 0.0 {
      let horizontal_correction = 0.0 - left;
      left += horizontal_correction;
      right += horizontal_correction;
    }
    //Shift the widget right if it would exceed the right bound of its parent
    else if right > screen.width as f32 {
      let horizontal_correction = screen.width as f32 - right;
      left += horizontal_correction;
      right += horizontal_correction;
    }

    //Center the widget vertically if it is taller than its parent
    if height > screen.height as f32 {
      let y = screen.height as f32 / 2.0;
      top = y - (height / 2.0);
      bottom = y + (height / 2.0);
    }
    //Shift the widget up if it would exceed the top of its parent
    else if top < 0.0 {
      let vertical_correction = 0.0 - top;
      top += vertical_correction;
      bottom += vertical_correction;
    }
    //Shift the widget up if it would exceed the bottom of its parent
    else if bottom > screen.height as f32 {
      let vertical_correction = screen.height as f32 - bottom;
      top += vertical_correction;
      bottom += vertical_correction;
    }

    //Convert the maxima to ndc
    left = 2.0 * left / screen.width as f32 - 1.0;
    right = 2.0 * right / screen.width as f32 - 1.0;
    top = 1.0 - (2.0 * top as f32) / screen.height as f32;
    bottom = 1.0 - (2.0 * bottom as f32) / screen.height as f32;

    //Create the widget's vertices
    let v1 = Vertex::new([right, top, z], [1.0, 1.0]); //Top Right
    let v2 = Vertex::new([right, bottom, z], [1.0, 0.0]); // Bottom Right
    let v3 = Vertex::new([left, bottom, z], [0.0, 0.0]); // Bottom Left
    let v4 = Vertex::new([left, top, z], [0.0, 1.0]); // Top Left

    //Add the vertices to a vertices vector
    let mut vertices = Vec::new();
    vertices.push(v1);
    vertices.push(v2);
    vertices.push(v3);
    vertices.push(v4);

    //Create the indices
    let indices = vec![0, 1, 3, 1, 2, 3];

    //Create the widget's mesh
    let gl = gl.clone();
    let mesh = Mesh::new(&gl, vertices, indices, texture_name);

    //Create the widget ids
    let id = 0;
    let parent = 0;

    BaseWidget {
      id,
      parent,
      height,
      width,
      position,
      gl,
      mesh,
    }
  }

  fn set_height(&mut self, height: f32) {
    todo!()
  }

  fn set_width(&mut self, width: f32) {
    todo!()
  }

  fn get_height(&self) -> f32 {
    todo!()
  }

  fn get_width(&self) -> f32 {
    todo!()
  }

  fn draw(&self) {
    let gl = &self.gl;
    let mesh = &self.mesh;
    draw_elements(gl, mesh)
  }

  fn destroy(&self) {
    todo!()
  }

  fn add_child(&mut self, child: impl GuiElement) {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use super::{BaseWidget, GuiElement, HorizontalAnchor, Orientation, VerticalAnchor};
  use crate::{
    config::asset_config,
    ecs::{systems::register_resources, world_resources::ScreenDimensions, World},
    view::render_gl::{Program, Programs},
  };
  use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, FRAGMENT_SHADER, STENCIL_BUFFER_BIT};
  use glfw::{Action, Context, Key};

  #[test]
  fn gui_rectangle_renders() {
    asset_config();

    let mut world = World::new();
    let (mut glfw, mut window, events) = register_resources(&mut world);

    //Create the orientation
    let horizontal_anchor = HorizontalAnchor::Center;
    let horizontal_offset = 0.0;
    let vertical_anchor = VerticalAnchor::Center;
    let vertical_offset = 0.0;
    let screen_dimensions = world.get_resource::<ScreenDimensions>().unwrap();
    let orientation = Orientation::new(horizontal_anchor, horizontal_offset, vertical_anchor, vertical_offset, &screen_dimensions);

    //Create the widget
    let gl = world.get_resource::<Gl>().unwrap();
    let widget = BaseWidget::new(orientation, 400.0, 200.0, gl.clone(), "ground", *screen_dimensions);

    //Set up the programs
    let mut programs = Programs::new();
    //Create and register the widget program
    let program = Program::new(&gl, "widget", "textured", FRAGMENT_SHADER).unwrap();
    programs.register_program(3, program);

    while !window.should_close() {
      glfw.poll_events();
      for (_, event) in glfw::flush_messages(&events) {
        match event {
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
          glfw::WindowEvent::MouseButton(..) => {
            dbg!(window.get_cursor_pos());
          }
          _ => {}
        }
      }

      //Render
      programs.use_program(3, &world);
      unsafe { gl.ClearColor(0.1, 0.1, 0.1, 1.0) };
      unsafe { gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT) }

      widget.draw();
      window.swap_buffers();
    }
  }
}
