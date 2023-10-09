use thiserror::Error;

#[derive(Debug,Error)]
pub enum CustomErrors{
  #[error("The system cannot load the requested resource from the given location.")]
  FailedToLoadImage,
  #[error("Failed to decode image")]
  FailedToDecodeImage,
  #[error("Format can only be RGB or RGB")]
  IllegalTextureFormat
}