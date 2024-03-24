mod base_widget;
mod button;
mod response;
mod textbox;
mod ui;
mod widget;

#[cfg(test)]
mod test {
  use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, FRAGMENT_SHADER, STENCIL_BUFFER_BIT};
  use glfw::{Action, Context, Key};

  use super::{
    button::Button,
    ui::{UIConfigInfo, UI},
  };
  use crate::{
    config::asset_config,
    ecs::{systems::register_resources, World},
    view::render_gl::{Program, Programs},
  };

  #[test]
  fn create_ui() {
    asset_config();

    let mut world = World::new();
    let (mut glfw, mut window, events) = register_resources(&mut world);

    //Create a UI
    let ctx = window.render_context();
    let config = UIConfigInfo::new().width(400.0).height(400.0).build();
    let ui = UI::new(1, config, ctx, 400.0, 400.0);

    //Create a button
    let gl = world.get_resource::<Gl>().unwrap();
    let btn_config = UIConfigInfo::new().width(200.0).height(100.0).build();
    let button = Button::new("Button Name", btn_config).parent(&ui).mesh_info(&gl, "ground").build().unwrap();

    //Set up the programs
    let mut programs = Programs::new();

    //Create and register the widget program
    let gl = world.get_resource::<Gl>().unwrap();
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
      button.draw(&gl);
      window.swap_buffers();
    }
  }
}
