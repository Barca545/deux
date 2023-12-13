use std::ptr;

use crate::{
	ecs::world_resources::ScreenDimensions,
	errors::{FilesystemErrors, FramebufferErrors, RenderErrors},
	filesystem::{load_image, ImageLoader},
};

use eyre::Result;
use gl::{
	types::{GLenum, GLint, GLsizei, GLuint, GLvoid},
	Gl, COLOR_ATTACHMENT0, FRAMEBUFFER, FRAMEBUFFER_COMPLETE, LINEAR, LINEAR_MIPMAP_LINEAR, NEAREST,
	REPEAT, RG32I, RGB, RGB32I, RGB8, RGBA, RGBA8, RGB_INTEGER, TEXTURE0, TEXTURE_2D,
	TEXTURE_BASE_LEVEL, TEXTURE_MAG_FILTER, TEXTURE_MAX_LEVEL, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S,
	TEXTURE_WRAP_T, UNSIGNED_BYTE, UNSIGNED_INT,
};

#[derive(Default, Debug)]
pub struct Texture {
	pub id: u32,
}

impl Texture {
	pub fn new(gl: &Gl, name: &str) -> Result<Self> {
		let image = load_image(&name)?;
		let image_pixels = image.to_rgba8().into_raw();

		let mut id = 0;

		unsafe {
			//make the texture
			gl.GenTextures(1, &mut id);
			gl.BindTexture(TEXTURE_2D, id);

			//load data
			gl.TexImage2D(
				TEXTURE_2D,
				0,
				RGBA8 as GLint,
				image.width() as GLsizei,
				image.height() as GLsizei,
				0,
				RGBA,
				UNSIGNED_BYTE,
				&image_pixels[0] as *const u8 as *const GLvoid,
			);

			//create mipmaps
			gl.GenerateMipmap(TEXTURE_2D);

			//Configure sampler
			gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as GLint);
			gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as GLint);
			gl.TexParameteri(
				TEXTURE_2D,
				TEXTURE_MIN_FILTER,
				LINEAR_MIPMAP_LINEAR as GLint,
			);
			gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
		};

		Ok(Texture { id })
	}

	//I think the reason I can only use 1 texture is because thre
	pub fn bind(&self, gl: &Gl) {
		unsafe { gl.BindTexture(TEXTURE_2D, self.id) }
	}

	pub fn unbind(&self, gl: &Gl) {
		unsafe { gl.BindTexture(TEXTURE_2D, 0) }
	}
}

// impl Drop for Texture{
//   fn drop(&mut self){
//     unsafe{self.gl.DeleteTextures(1,&mut self.texture_obj)}
//   }
// }
