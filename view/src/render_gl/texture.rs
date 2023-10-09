use super::{CustomErrors, ImageLoader};

use eyre::Result;
use gl::{TEXTURE_2D, Gl, UNSIGNED_BYTE, RGB, RGBA, types::{GLvoid,GLenum, GLuint, GLint, GLsizei}, RGB8, TEXTURE_BASE_LEVEL, TEXTURE_MAX_LEVEL, RGBA8, TEXTURE0,TEXTURE_WRAP_S,TEXTURE_WRAP_T,REPEAT,TEXTURE_MIN_FILTER,TEXTURE_MAG_FILTER,LINEAR};

pub struct Texture{
  gl:Gl,
  texture_obj:GLuint //is this the object actually containing all the texture data?
}

impl Texture {
  fn default(gl:&Gl)->Texture{
    Texture {
      gl: gl.clone(),
      texture_obj: 0
    }
  }
  
  pub fn rgb_from_path(path:&str) -> TextureLoadBuilder{
    TextureLoadBuilder { 
      options: TextureLoadOptions::rgb_from_path(path)
    }
  }

  pub fn rgba_from_path(path:&str) ->TextureLoadBuilder{
    TextureLoadBuilder { 
      options: TextureLoadOptions::rgba_from_path(path)
    } 
  }

  //I still feel like something is supposed to be calling this externally since Necury had it publick in his crate but idk how
  fn from_path<'a>(
    options: TextureLoadOptions<'a>,
    gl:&Gl,
  )->Result<Self>{
    let mut texture_obj:GLuint = 0;
    
    unsafe{
      gl.GenTextures(1,&mut texture_obj)
    };

    let texture = Texture::default(gl);
    texture.update(options)?;

    Ok(texture)
  }

  pub fn update<'a>(
    &self,
    options: TextureLoadOptions<'a>,
  )->Result<()>{
    let gl = &self.gl;
    
    unsafe{
      gl.BindTexture(TEXTURE_2D,self.texture_obj);
      // set the texture wrapping parameters (default wrapping method)
      gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as GLint); 
      gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as GLint);
    }
    
    match options.format{
      RGB => {
        let image = options.image_from_path(options.path)?;
        let image_pixels = image.to_rgb8().into_raw();
        unsafe{
          gl.TexImage2D(
            TEXTURE_2D, 
            0,
            RGB8 as GLint, 
            image.width() as GLsizei, 
            image.height() as GLsizei, 
            0, 
            RGB, 
            UNSIGNED_BYTE, 
            &image_pixels[0] as *const u8 as *const GLvoid
          );
        }
        if options.gen_mipmaps {
          unsafe{
            gl.GenerateMipmap(TEXTURE_2D)
          }
        }
        else{
          unsafe{   
            // set texture filtering parameters   
            gl.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as GLint);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as GLint);
          }
        }
      }
    RGBA => {
      let image = options.image_from_path(options.path)?;
      let image_pixels = image.to_rgba8().into_raw();
      unsafe{
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
      }
      if options.gen_mipmaps {
        unsafe{
          gl.GenerateMipmap(TEXTURE_2D)
        }
      }
      else{
        unsafe{
          // set texture filtering parameters
          gl.TexParameteri(TEXTURE_2D,TEXTURE_BASE_LEVEL,LINEAR as GLint);
          gl.TexParameteri(TEXTURE_2D,TEXTURE_MAX_LEVEL,LINEAR as GLint);
        }
      }
    } 
    _=> unreachable!("{}",CustomErrors::IllegalTextureFormat) 
    }

    unsafe{gl.BindTexture(TEXTURE_2D,0)}

    Ok(())
  }

  pub fn bind(&self){
    unsafe{
      self.gl.BindTexture(TEXTURE_2D,self.texture_obj)
    }
  }
  //why does unbind call bind?
  pub fn unbind(&self){
    unsafe{
      self.gl.BindTexture(TEXTURE_2D,0)
    }
  }
  
  //what does this do?
  pub fn bind_at(&self,index:u32){
    unsafe{
      self.gl.ActiveTexture(TEXTURE0+index)
    }
    self.bind()
  }
}


impl Drop for Texture{
  fn drop(&mut self){
    unsafe{self.gl.DeleteTextures(1,&mut self.texture_obj)}
  }
}

pub struct TextureLoadBuilder<'a> {
  options: TextureLoadOptions<'a>,
}

impl<'a> TextureLoadBuilder<'a>{
  pub fn load(self,gl:&Gl)->Result<Texture>{
    Texture::from_path(self.options, gl)
  }
  
  pub fn with_mipmaps(mut self)->Self{
    self.options.gen_mipmaps = true;
    self
  }
}

pub struct TextureLoadOptions<'a>{
  path: &'a str,
  format: GLenum,
  gen_mipmaps: bool //is this needed if I do not want mipmaps?
}

impl<'a> ImageLoader<'a> for TextureLoadOptions<'a>{}

impl<'a> TextureLoadOptions<'a>{
  pub fn rgb_from_path(path: &'a str)->Self{
    TextureLoadOptions { 
      path,
      format: RGB,
      gen_mipmaps: true
    }
  }
  
  pub fn rgba_from_path(path: &'a str)->Self{
    TextureLoadOptions { 
      path,
      format: RGBA,
      gen_mipmaps: true
    }
  }
}
