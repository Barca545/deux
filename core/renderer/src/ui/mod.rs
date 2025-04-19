use yakui::Yakui;
use yakui_wgpu::YakuiWgpu;

use crate::core::gpu_context::GpuContext;

pub struct Ui {}

impl Ui {
  pub fn new(ctx: &GpuContext,) -> Self {
    let mut ui = Yakui::new();
    ui.start();

    yakui::center(|| {
      yakui::text(32.0, "Hello, world!",);
    },);

    ui.finish();

    ui.paint();

    todo!()
  }
}
