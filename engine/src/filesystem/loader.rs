use crate::errors::FilesystemErrors;
use std::path::Path;
use image::{io::Reader, DynamicImage};
use eyre::Result;

pub trait ImageLoader<'a>{
  ///Loads a `DynamicImage` from a user defined path
  fn image_from_path(&self,path:&'a str)->Result<DynamicImage>{
    let path =  Path::new(path);
    let image = Reader::open(path)
      .unwrap_or_else(|_| panic!("{}",{FilesystemErrors::FailedToLoadImage}))
      .decode()
      .unwrap_or_else(|_| panic!("{}",{FilesystemErrors::FailedToDecodeImage}));
    Ok(image)
  }
}

pub trait FileLoader<'a>{
  ///Loads a `DynamicImage` from a user defined path
  fn image_from_path(&self,path:&'a str)->Result<DynamicImage>{
    let path =  Path::new(path);
    let image = Reader::open(path)
      .unwrap_or_else(|_| panic!("{}",{FilesystemErrors::FailedToLoadImage}))
      .decode()
      .unwrap_or_else(|_| panic!("{}",{FilesystemErrors::FailedToDecodeImage}));
    Ok(image)
  }
}