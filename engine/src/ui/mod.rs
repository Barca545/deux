mod base_widget;
mod button;
mod response;
mod style;
mod textbox;
mod ui;
mod ui_config;
mod widget;

pub use self::{base_widget::*, button::*, response::*, ui::*, widget::*};

#[cfg(test)]
mod test {
  use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, FRAGMENT_SHADER, STENCIL_BUFFER_BIT};
  use glfw::{Action, Context, Key};
  use glm::{identity, translate, Vec4};
  use nalgebra::{Orthographic3, Perspective3};

  use super::{
    base_widget::{HorizontalAnchor, VerticalAnchor},
    button::Button,
    ui::{Dimensions, UIConfigInfo, UI},
  };
  use crate::{
    config::asset_config,
    ecs::{systems::register_resources, World},
    math::{calculate_model_transform, Mat4, Transforms, Vec3},
    view::render_gl::{Program, Programs},
  };

  // Refactor:
  // -If using the uniforms, want the vertices created with respect to 0,0
  //  Then generate a model matrix from the position

  #[test]
  fn create_ui() {
    asset_config();
    let mut world = World::new();
    let (mut glfw, mut window, events) = register_resources(&mut world);

    //Create a UI
    let ctx = window.render_context();
    let sceen_dimensions = Dimensions::new(1280, 720);
    let config = UIConfigInfo::new(sceen_dimensions).build();
    let ui = UI::new(1, config, ctx);

    //Create a button
    let gl = world.get_resource::<Gl>().unwrap();
    let btn_config = UIConfigInfo::new(sceen_dimensions)
      .width(1.0)
      .height(1.0)
      .horizontal_anchor(HorizontalAnchor::Center)
      .vertical_anchor(VerticalAnchor::Center)
      .build();
    let button = Button::new("Button Name", btn_config).parent(&ui).mesh_info(&gl, "ground").build().unwrap();

    //Set up the programs
    let mut programs = Programs::new();

    //Create and register the widget program
    let program = Program::new(&gl, "widget", "textured", FRAGMENT_SHADER)
      .unwrap()
      .with_model(&gl)
      .unwrap()
      // .with_projection(&gl)
      // .unwrap()
      // .with_view(&gl)
      // .unwrap()
      .build()
      .unwrap();
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
      let btn_position = button.config.ndc_position();

      //Set the model transform's value
      let model: Mat4 = identity::<f32, 4>();
      let model_transform: Mat4 = translate(&model, &btn_position);
      program.set_model_matrix(&gl, &model_transform);

      //Set projection Transform
      // let transforms = world.get_resource::<Transforms>().unwrap();
      // let projection_transform = transforms.projection_transform.as_matrix();
      // let t = Vec4::new(0.5, 0.5, 0.0, 1.0);
      // dbg!(projection_transform * t);
      // program.set_projection_matrix(&gl, &projection_transform);
      // programs.set_vp_uniforms(3, &world);

      unsafe { gl.ClearColor(0.1, 0.1, 0.1, 1.0) };
      unsafe { gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT) }
      button.draw(&gl);
      window.swap_buffers();
    }
  }

  // #[test]
  // fn projections_work() {
  //   let proj = Orthographic3::new(1.0, 10.0, 2.0, 20.0, 0.1, 1000.0);
  // }
}
