use gl::Gl;

#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
	pub height: i32,
	pub width: i32,
	pub aspect: f32,
}
impl ScreenDimensions {
	pub fn new(height: i32, width: i32) -> Self {
		let aspect = width as f32 / height as f32;
		ScreenDimensions {
			height,
			width,
			aspect,
		}
	}
}
