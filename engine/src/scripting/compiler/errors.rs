use thiserror::Error;

use super::token::TokenKind;

//separate lexing and parsing errors

#[derive(Debug, Clone, Error, PartialEq, Eq,)]
pub enum ParsingError {
  #[error("{0:?} is not recognized as a token.")]
  ChunkNotRecognized(String,),
  #[error("Strings must be closed with \"\" ")]
  StringNotTerminated,
  #[error("{0:?} is not an integer or a float.")]
  NotValidNumber(String,),
  #[error("Expected variable declaration.")]
  VarNotDeclared,
  #[error("{0:?} is not a valid variable name. Variable names must begin with a letter")]
  InvalidVarName(String,),
  #[error("Unexpected token. {expected:?} is not {recieved:?}")]
  UnexpectedToken { expected:TokenKind, recieved:TokenKind, },
  #[error("{0:?} is not a valid type. Accepted types are int, float, usize, and str or their corresponding arrays int[], float[], usize[], and str[]")]
  InvalidType(String,),
  #[error("No statement match")]
  NoStatementMatch,
}
