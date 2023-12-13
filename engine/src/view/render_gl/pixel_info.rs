use gl::{Gl, COLOR_ATTACHMENT0, NONE, READ_FRAMEBUFFER, RGB_INTEGER, UNSIGNED_INT};

#[derive(Default)]
pub struct PixelInfo {
	object_id: u128,
	draw_id: u128,
	primitive_id: u128,
}

impl PixelInfo {
	pub fn read_pixel(x: f64, y: f64, gl: &Gl) {
		unsafe {
			// gl.BindFramebuffer(READ_FRAMEBUFFER, self.fbo.get_buffer_obj());

			// gl.ReadBuffer(COLOR_ATTACHMENT0);

			// // let pixel = PixelInfo::default();
			// gl.ReadPixels(1, 1, 1, 1, RGB_INTEGER, UNSIGNED_INT, );

			// gl.ReadBuffer(NONE);

			// gl.BindFramebuffer(READ_FRAMEBUFFER, 0);

			// return Pixel;
		}
	}
}
