use std::ptr;

use gl::{
	types::{GLenum, GLint, GLuint},
	Gl, COLOR_ATTACHMENT0, DEPTH_COMPONENT, FLOAT, FRAMEBUFFER, NEAREST, RGB32I, RGB32UI,
	RGB_INTEGER, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, UNSIGNED_INT,
};

use crate::ecs::world_resources::ScreenDimensions;

//might be a better name for this
pub struct TextureAttachment {
	gl: Gl,
	texture_obj: GLuint,
}

impl TextureAttachment {
	pub fn new(gl: &Gl, texture_obj: GLuint) -> Self {
		let gl = gl.clone();
		let mut texture_obj = 0;

		unsafe {
			gl.GenTextures(1, &mut texture_obj);
		}

		TextureAttachment { gl, texture_obj }
	}

	pub fn get_texture_obj(&self) -> GLuint {
		self.texture_obj
	}

	pub fn generate_texture_attachment(
		&self, screen_dimensions: &ScreenDimensions, internal_format: GLenum, format: GLenum,
		texture_type: GLenum,
	) {
		unsafe {
			self.gl.BindTexture(TEXTURE_2D, self.texture_obj);
			self.gl.TexImage2D(
				TEXTURE_2D,
				0,
				internal_format as GLint,
				screen_dimensions.width,
				screen_dimensions.height,
				0,
				format,
				texture_type,
				ptr::null(),
			);

			if format == RGB_INTEGER {
				self
					.gl
					.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as GLint);
				self
					.gl
					.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as GLint);
			}
		}
	}
}
