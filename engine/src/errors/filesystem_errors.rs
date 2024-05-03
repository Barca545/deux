use image::ImageError;
use std::io::Error as SysError;
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
  #[error("Shader {name} does not exist at {path}. \nSystem Error: {err:?}")]
  ShaderDoesNotExist { name: String, path: String, err: SysError },
  #[error("Champion data {name} does not exist at {path}. \nSystem Error: {err:?}")]
  ChampDataDoesNotExist { name: String, path: String, err: SysError },
}
