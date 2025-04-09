use image::ImageError;
use std::io::Error as SysError;
use thiserror::Error;

#[derive(Debug, Error,)]
pub enum RendererErrors {
  #[error("The system cannot load {name} from {path}.")]
  FailedToLoadImage { name:String, path:String, },
  #[error("Failed to decode image:{0:?}")]
  FailedToDecodeImage(ImageError,),
  #[error("Format can only be RGB or RGBA not {0}")]
  // TODO: This is a place holder until I can figure out the actual type this should be
  IllegalTextureFormat(usize,),
  #[error("Shader {name} does not exist at {path}. \nSystem Error: {err:?}")]
  ShaderDoesNotExist {
    name:String,
    path:String,
    err:SysError,
  },
}
