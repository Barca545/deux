use thiserror::Error;

//separate lexing and parsing errors

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq,)]
pub enum ParsingError {
  #[error("{0:?} is not recognized as a token.")]
  ChunkNotRecognized(u32,),
  #[error("Strings must be closed with \"\" ")]
  StringNotTerminated,
  #[error("{0:?} is not an integer or a float.")]
  NotValidNumber(u32,),
  #[error("Expected variable declaration.")]
  VarNotDeclared,
  #[error("{0:?} is not a valid variable name. Variable names must begin with a letter")]
  InvalidVarName(u32,),
  #[error("Unexpected {0:?} token.")]
  UnexpectedToken(u32,),
  #[error("{0:?} is not a valid type. Accepted types are int, float, usize, and str or their corresponding arrays int[], float[], usize[], and str[]")]
  InvalidTpe(u32,),
  #[error("No statement match")]
  NoStatementMatch,
}
