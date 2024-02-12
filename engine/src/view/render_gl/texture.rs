use crate::filesystem::load_texture_image;

use eyre::Result;
use gl::{
  types::{GLint, GLsizei, GLvoid},
  Gl, LINEAR, LINEAR_MIPMAP_LINEAR, REPEAT, RGBA, RGBA8, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T,
  UNSIGNED_BYTE
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Texture {
  pub id:u32
}

impl Texture {
  //eventually will need to change when I settle on a texture filetype
  pub fn new(gl:&Gl, name:&str) -> Result<Self> {
    let image = load_texture_image(&name, "jpg")?;
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
        &image_pixels[0] as *const u8 as *const GLvoid
      );

      //create mipmaps
      gl.GenerateMipmap(TEXTURE_2D);

      //Configure sampler
      gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as GLint);
      gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as GLint);
      gl.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as GLint);
      gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
    };

    Ok(Texture { id })
  }

  //I think the reason I can only use 1 texture is because thre
  pub fn bind(&self, gl:&Gl) {
    unsafe { gl.BindTexture(TEXTURE_2D, self.id) }
  }

  pub fn unbind(&self, gl:&Gl) {
    unsafe { gl.BindTexture(TEXTURE_2D, 0) }
  }
}

// impl Drop for Texture{
//   fn drop(&mut self){
//     unsafe{self.gl.DeleteTextures(1,&mut self.id)}
//   }
// }
