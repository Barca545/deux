use crate::errors::{EcsErrors, FilesystemErrors};
use eyre::Result;
use image::{io::Reader, DynamicImage};
use std::{
  ffi::CString,
  fs::File,
  io::Read,
  path::{Path, PathBuf}
};

pub fn load_image(name:&str) -> Result<DynamicImage> {
  let path = name_to_path(name);
  let image = Reader::open(path)
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
    .decode()
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));
  Ok(image)
}

pub fn load_cstring(name:&str) -> Result<CString> {
  let mut file = File::open(name_to_path(name))?;

  let mut buffer:Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

  file.read_to_end(&mut buffer)?;

  if buffer.iter().find(|i| **i == 0).is_some() {
    return Err(EcsErrors::FileContainsNil.into());
  }
  Ok(unsafe { CString::from_vec_unchecked(buffer) })
}

fn name_to_path(name:&str) -> PathBuf {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();
  let path:PathBuf = PathBuf::from(root_dir + name);
  path
}

pub trait ImageLoader<'a> {
  ///Loads a `DynamicImage` from a user defined path
  fn image_from_path(&self, path:&'a str) -> Result<DynamicImage> {
    let path = Path::new(path);
    let image = Reader::open(path)
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
      .decode()
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));
    Ok(image)
  }
}

pub trait FileLoader<'a> {
  ///Loads a `DynamicImage` from a user defined path
  fn image_from_path(&self, path:&'a str) -> Result<DynamicImage> {
    let path = Path::new(path);
    let image = Reader::open(path)
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
      .decode()
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));
    Ok(image)
  }
}

#[cfg(test)]
mod test {
  use std::path::Path;

  use eyre::Result;
  use image::io::Reader;

  use crate::errors::FilesystemErrors;
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
  fn load_cstring() {}
}
