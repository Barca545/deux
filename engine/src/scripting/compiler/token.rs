use super::errors::ParsingError;

// TODO:
// - Debating getting rid of the difference between floats and ints and just
//   using floats only
// - The error reporting does not properly repeat the dashes

#[derive(Debug, Clone, Copy,)]
pub(super) struct Location {
  pub(super) line:u32,
  pub(super) col:u32,
  pub(super) index:usize,
}

impl Location {
  /// Create a new [`Location`].
  pub fn new() -> Self {
    Location { line:1, col:1, index:0, }
  }

  pub fn next(&mut self, newline:bool,) {
    match newline {
      true => {
        self.line += 1;
        self.index += 1;
      }
      false => {
        self.col += 1;
        self.index += 1;
      }
    }
  }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub(super) enum TokenKind {
  // Single-character tokens
  TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
  TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
  TOKEN_COMMA, TOKEN_DOT, TOKEN_SEMICOLON, TOKEN_DOUBLE_COLON,
  // Math
  TOKEN_MINUS, TOKEN_PLUS, TOKEN_SLASH, TOKEN_STAR,
  // Equality
  TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
  TOKEN_BANG, TOKEN_BANG_EQUAL,
  TOKEN_GREATER, TOKEN_GREATER_EQUAL,
  TOKEN_LESS, TOKEN_LESS_EQUAL,
  // Literals
  TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_INT, TOKEN_FLOAT,
  // Keywords
  TOKEN_AND, TOKEN_OR, 
  TOKEN_TRUE, TOKEN_FALSE,
  TOKEN_FOR, TOKEN_WHILE, TOKEN_LOOP,
  TOKEN_IF, TOKEN_ELSE, TOKEN_ELSE_IF,
  TOKEN_FN, TOKEN_RETURN, TOKEN_PRINT, 
  TOKEN_LET, TOKEN_MUT, 
  // Terminators
  TOKEN_ERROR(ParsingError), TOKEN_EOF
}

impl TokenKind {
  ///Create a new token type.
  fn new(value:&String,) -> Self {
    // Return early if the token kind is a string
    if value.starts_with('"',) {
      // Check if the string terminates, return a TOKEN_ERROR if it does not
      return match value.chars().last().unwrap() == '"' {
        true => TokenKind::TOKEN_STRING,
        false => TokenKind::TOKEN_ERROR(ParsingError::StringNotTerminated,),
      };
    }

    // Return early if the token kind is a number
    if value.chars().nth(0,).unwrap().is_numeric() {
      // Check if the token is an int or a float
      if is_float(&value,) {
        return TokenKind::TOKEN_FLOAT;
      }
      else if value.parse::<u32>().is_ok() {
        return TokenKind::TOKEN_INT;
      }
      else {
        return TokenKind::TOKEN_ERROR(ParsingError::NotValidNumber(value.to_string(),),);
      }
    }

    match value.as_str() {
      // Single-character tokens
      "(" => TokenKind::TOKEN_LEFT_PAREN,
      ")" => TokenKind::TOKEN_RIGHT_PAREN,
      "{" => TokenKind::TOKEN_LEFT_BRACE,
      "}" => TokenKind::TOKEN_RIGHT_BRACE,
      ";" => TokenKind::TOKEN_SEMICOLON,
      "::" => TokenKind::TOKEN_DOUBLE_COLON,
      "," => TokenKind::TOKEN_COMMA,
      "." => TokenKind::TOKEN_DOT,
      // Math
      "-" => TokenKind::TOKEN_MINUS,
      "+" => TokenKind::TOKEN_PLUS,
      "*" => TokenKind::TOKEN_STAR,
      "/" => TokenKind::TOKEN_SLASH,
      // Equality
      "=" => TokenKind::TOKEN_EQUAL,
      "==" => TokenKind::TOKEN_EQUAL_EQUAL,
      "!" => TokenKind::TOKEN_BANG,
      "!=" => TokenKind::TOKEN_BANG_EQUAL,
      ">" => TokenKind::TOKEN_GREATER,
      ">=" => TokenKind::TOKEN_GREATER_EQUAL,
      "<" => TokenKind::TOKEN_LESS,
      "<=" => TokenKind::TOKEN_LESS_EQUAL,
      // Keywords
      "and" => TokenKind::TOKEN_AND,
      "or" => TokenKind::TOKEN_OR,
      "true" => TokenKind::TOKEN_TRUE,
      "false" => TokenKind::TOKEN_FALSE,
      "for" => TokenKind::TOKEN_FOR,
      "while" => TokenKind::TOKEN_WHILE,
      "loop" => TokenKind::TOKEN_LOOP,
      "if" => TokenKind::TOKEN_IF,
      "else" => TokenKind::TOKEN_ELSE,
      "else if" => TokenKind::TOKEN_ELSE_IF,
      "fn" => TokenKind::TOKEN_FN,
      "return" => TokenKind::TOKEN_RETURN,
      "print" => TokenKind::TOKEN_PRINT,
      "let" => TokenKind::TOKEN_LET,
      "mut" => TokenKind::TOKEN_MUT,
      _ => TokenKind::TOKEN_ERROR(ParsingError::ChunkNotRecognized(value.to_string(),),),
    }
  }
}

fn is_float(val:&str,) -> bool {
  // Check string only has one period and otherwise only contains numbers
  let mut period_found = false;
  for ch in val.chars() {
    if !ch.is_numeric() {
      if ch == '.' {
        if period_found {
          return false;
        }
        else {
          period_found = true;
        }
      }
      else {
        return false;
      }
    }
  }

  // If the first and last chars are numeric and it has not other elements other
  // than a period, it is a float
  val.chars().nth(0,).unwrap().is_numeric() && val.chars().last().unwrap().is_numeric() && period_found
}

#[derive(Debug,)]
///Intermediary struct for storing the data needed to create a [`Token`].
pub struct Chunk {
  loc:Location,
  val:String,
  newline:bool,
}

impl Chunk {
  ///Create a new [`Chunk`].
  pub fn new() -> Self {
    Chunk {
      loc:Location::new(),
      val:String::new(),
      newline:false,
    }
  }

  pub fn push(&mut self, ch:char,) {
    self.val.push(ch,);
    if ch == '\n' {
      self.newline = true
    }
    else {
      self.newline = false
    }
  }

  pub fn len(&self,) -> usize {
    self.val.len()
  }

  ///Emit a [`Token`] and ready a new [`Chunk`].
  pub fn to_token(&mut self,) -> Token {
    //Ensure the pointer is to the front of the token.
    let mut loc = self.loc;
    loc.col -= self.val.chars().count() as u32;
    loc.index -= self.val.chars().count();
    let token = Token::new(Some(&self.val,), loc,);
    self.val = String::new();
    token
  }

  ///Create a new [`Token`] from a [`String`].
  pub fn new_token(&mut self, val:&str,) -> Token {
    //Ensure the pointer is to the front of the token.
    let mut loc = self.loc;
    loc.col -= self.val.chars().count() as u32;
    loc.index -= self.val.chars().count();
    let token = Token::new(Some(&String::from(val,),), loc,);
    token
  }

  ///Increment the [`Chunk`]'s [`Location`]
  pub fn next(&mut self,) {
    self.loc.next(self.newline,);
    self.newline = false;
  }

  pub fn loc(&self,) -> Location {
    self.loc
  }
}

#[derive(Debug, Clone,)]
pub(super) struct Token {
  pub value:Option<String,>,
  pub kind:TokenKind,
  pub loc:Location,
}

impl Token {
  /// Create a [`Token`] from a [`String`].
  pub fn new(value:Option<&String,>, loc:Location,) -> Token {
    match value {
      Some(str,) => Token {
        value:Some(str.to_string(),),
        kind:TokenKind::new(str,),
        loc,
      },
      None => Token {
        value:None,
        kind:TokenKind::TOKEN_EOF,
        loc,
      },
    }
  }

  /// Convert a [`Token`]'s `kind`into `TOKEN_IDENTIFIER`.
  pub fn to_ident(&mut self,) {
    // Confirm the token is a valid identifier
    if let Some(val,) = &self.value {
      if val.chars().nth(0,).unwrap().is_alphabetic() {
        self.kind = TokenKind::TOKEN_IDENTIFIER;
      }
      else {
        self.kind = TokenKind::TOKEN_ERROR(ParsingError::InvalidVarName(self.value.clone().unwrap().to_string(),),);
      }
    }
  }

  ///Create a`TOKEN_ERROR(ParsingError::VarNotDeclared)` [`Token`].
  pub fn var_not_declared_token(value:Option<&str,>, loc:Location,) -> Token {
    Token {
      value:Some(value.unwrap().to_string(),),
      kind:TokenKind::TOKEN_ERROR(ParsingError::VarNotDeclared,),
      loc,
    }
  }
}
