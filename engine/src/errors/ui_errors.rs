use thiserror::Error;

#[derive(Debug, Error)]
pub enum UIErrors {
  #[error("Tried to create a child element without a parent.")]
  NoParentElement,
  #[error("Tried to create a widget without any render information")]
  NoRenderInformation,
}
