use thiserror::Error;

#[derive(Debug, Error)]
pub enum VMError {
  #[error("{0:?} is not an OpCode!")]
  UnrecognizedOpCode(u8)
}
