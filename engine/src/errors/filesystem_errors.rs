use image::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilesystemErrors {
  #[error("The system cannot load {name} from {path}.")]
  FailedToLoadImage { name: String, path: String },
  #[error("Failed to decode image:{0:?}")]
  FailedToDecodeImage(ImageError),
  #[error("Format can only be RGB or RGBA")]
  IllegalTextureFormat,
  #[error("File does not contain readable data")]
  FileContainsNil,
}
