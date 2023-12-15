use crate::errors::{EcsErrors, FilesystemErrors};
use eyre::Result;
use gl::{types::GLenum, VERTEX_SHADER, FRAGMENT_SHADER};
use image::{io::Reader, DynamicImage};
use std::{
  ffi::CString,
  fs::{File, FileType},
  io::Read,
  path::PathBuf
};

pub fn load_image(name:&str, extension:&str) -> Result<DynamicImage> {
  let path = name_to_pathbuff(name, extension);
  let image = Reader::open(path)
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
    .decode()
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));
  Ok(image)
}

pub fn load_cstring(name:&str, extension:&str) -> Result<CString> {
  let mut file = File::open(name_to_pathbuff(name, extension))?;
  
  let mut buffer:Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

  file.read_to_end(&mut buffer)?;

  if buffer.iter().find(|i| **i == 0).is_some() {
    return Err(EcsErrors::FileContainsNil.into());
  }
  Ok(unsafe { CString::from_vec_unchecked(buffer) })
}

pub fn load_shader(name:&str, extension:&str) ->Result<CString> {
  let shader = load_cstring(name, extension)?;
  
  Ok(shader)
}

fn name_to_pathbuff(name:&str, extension:&str) -> PathBuf {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();  
  let path:PathBuf = PathBuf::from(root_dir + name + "." + extension);
  path
}

#[cfg(test)]
mod test {
  use std::path::Path;
  use eyre::Result;
  use image::io::Reader;
  use crate::errors::FilesystemErrors;

use super::load_shader;
  #[test]
  fn load_image() -> Result<()> {
    let name = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/wall.jpg";

    let path = Path::new(name);
    let _image = Reader::open(path)
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
      .decode()
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));

    Ok(())
  }

  #[test]
  fn load_test_shader() -> Result<()>{
    let shader = load_shader("textured","vert")?;
    dbg!(shader);
    Ok(())
  }
}
