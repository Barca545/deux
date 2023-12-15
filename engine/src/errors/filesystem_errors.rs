use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilesystemErrors {
  #[error("The system cannot load the requested resource from the given location.")]
  FailedToLoadImage,
  #[error("Failed to decode image")]
  FailedToDecodeImage,
  #[error("Format can only be RGB or RGB")]
  IllegalTextureFormat
}
