use glm::{inverse, vec4};

use crate::{
	ecs::world_resources::ScreenDimensions,
	math::{
		math::{Mat4, Vec3, Vec4},
		MouseRay, Transforms,
	},
	view::camera::Camera,
};

pub struct MousePicker {
	ray: Option<MouseRay>,
}

impl MousePicker {
	pub fn new() -> Self {
		MousePicker { ray: None }
	}

	///Updates the `MousePicker`'s stored `MouseRay`.
	pub fn update_ray(
		&mut self, x: f64, y: f64, screen_dimensions: &ScreenDimensions, transforms: &Transforms,
	) {
		let inverse_projection: Mat4 = transforms.get_projection_transform().inverse();
		let inverse_view: Mat4 = inverse(&transforms.get_view_transform());

		let ndc_x = 2.0 * x as f32 / screen_dimensions.width as f32 - 1.0; //range [-1,1]
		let ndc_y = 1.0 - (2.0 * y as f32) / screen_dimensions.height as f32; //range [-1,1]

		let ndc: Vec4 = vec4(ndc_x, ndc_y, -1.0, 1.0);

		let mut ray_viewspace_coordinates: Vec4 = inverse_projection * ndc;
		ray_viewspace_coordinates /= ray_viewspace_coordinates.w;

		//convert to worldspace
		let mut ray_worldspace_coordinates: Vec4 = inverse_view * ray_viewspace_coordinates;
		ray_worldspace_coordinates /= ray_worldspace_coordinates.w;

		self.ray = Some(MouseRay::new(x, y, screen_dimensions, transforms));
	}
}
