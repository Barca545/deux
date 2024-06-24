use super::{
  errors::ParsingError,
  interner::{intern, lookup},
};

// TODO:
// - Not sure I need the double colon operator
// - How do tokenizers actually handle type declarations
// - switch from passing in an interner to using the INTERNER directly

// Refactor:
// - Debating getting rid of the difference between floats and ints and just
//   using floats only

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
        self.col = 1;
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
#[derive(Debug, Eq, Clone, Copy)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub(super) enum TokenKind {
  // Single-character tokens
  TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
  TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
  TOKEN_LEFT_BRACKET, TOKEN_RIGHT_BRACKET,
  TOKEN_COMMA, TOKEN_DOT, TOKEN_SEMICOLON,TOKEN_COLON, TOKEN_DOUBLE_COLON,
  // Math
  TOKEN_MINUS, TOKEN_PLUS, TOKEN_SLASH, TOKEN_STAR,
  // Assignment
  TOKEN_EQUAL, TOKEN_MINUS_EQUAL, TOKEN_PLUS_EQUAL, TOKEN_SLASH_EQUAL, TOKEN_STAR_EQUAL,
  // Equality
  TOKEN_EQUAL_EQUAL,
  TOKEN_NOT, TOKEN_NOT_EQUAL,
  TOKEN_GREATER, TOKEN_GREATER_EQUAL,
  TOKEN_LESS, TOKEN_LESS_EQUAL, 
  // Literals
  TOKEN_IDENTIFIER(u32), TOKEN_STRING(u32), TOKEN_INT(u32), TOKEN_FLOAT(u32),
  TOKEN_TYPE(u32), TOKEN_BOOL(u32),

  // Keywords
  TOKEN_AND, TOKEN_OR, 
  TOKEN_TRUE, TOKEN_FALSE,
  TOKEN_FOR, TOKEN_WHILE, TOKEN_LOOP,
  TOKEN_IF, TOKEN_ELSE, TOKEN_ELSE_IF,
  TOKEN_FN, TOKEN_RETURN, TOKEN_PRINT, 
  TOKEN_LET, TOKEN_MUT, 
  // Terminators
  TOKEN_ERROR(u32), TOKEN_EOF,
  // Intermediary Tokens
  TOKEN_POSSIBLE_IDENT(u32),

}

impl TokenKind {
  ///Create a new token type.
  fn new(value:&str,) -> Self {
    // Return early if the token kind is a string
    if value.starts_with('\"',) {
      // Check if the string terminates, return a TOKEN_ERROR if it does not
      let idx = intern(value,);
      return match value.chars().last().unwrap() == '"' {
        true => TokenKind::TOKEN_STRING(idx,),
        false => {
          let idx_err = intern(&ParsingError::StringNotTerminated.to_string(),);
          TokenKind::TOKEN_ERROR(idx_err,)
        }
      };
    }

    // Return early if the token kind is a number
    if value.chars().nth(0,).unwrap().is_numeric() {
      let idx = intern(value,);
      // Check if the token is an int or a float
      if is_float(&value,) {
        return TokenKind::TOKEN_FLOAT(idx,);
      }
      else if value.parse::<u32>().is_ok() {
        return TokenKind::TOKEN_INT(idx,);
      }
      else {
        let idx_err = intern(&ParsingError::NotValidNumber(idx,).to_string(),);
        return TokenKind::TOKEN_ERROR(idx_err,);
      }
    }

    // If the token is a type it is a type declaration
    if value == "int"
      || value == "float"
      || value == "usize"
      || value == "char"
      || value == "string"
      || value == "bool"
      || value == "int[]"
      || value == "float[]"
      || value == "usize[]"
      || value == "string[]"
      || value == "bool[]"
    {
      let idx = intern(value,);
      return TokenKind::TOKEN_TYPE(idx,);
    }

    //If the token is "true" or "false" it is a boolean
    if value == "true" || value == "false" {
      let idx = intern(value,);
      return TokenKind::TOKEN_BOOL(idx,);
    }

    match value {
      // Single-character tokens
      "(" => TokenKind::TOKEN_LEFT_PAREN,
      ")" => TokenKind::TOKEN_RIGHT_PAREN,
      "{" => TokenKind::TOKEN_LEFT_BRACE,
      "}" => TokenKind::TOKEN_RIGHT_BRACE,
      "[" => TokenKind::TOKEN_LEFT_BRACKET,
      "]" => TokenKind::TOKEN_RIGHT_BRACKET,
      ";" => TokenKind::TOKEN_SEMICOLON,
      ":" => TokenKind::TOKEN_COLON,
      "::" => TokenKind::TOKEN_DOUBLE_COLON,
      "," => TokenKind::TOKEN_COMMA,
      "." => TokenKind::TOKEN_DOT,
      // Math
      "-" => TokenKind::TOKEN_MINUS,
      "+" => TokenKind::TOKEN_PLUS,
      "/" => TokenKind::TOKEN_SLASH,
      "*" => TokenKind::TOKEN_STAR,
      // Assignment
      "=" => TokenKind::TOKEN_EQUAL,
      "-=" => TokenKind::TOKEN_MINUS_EQUAL,
      "+=" => TokenKind::TOKEN_PLUS_EQUAL,
      "/=" => TokenKind::TOKEN_SLASH_EQUAL,
      "*=" => TokenKind::TOKEN_STAR_EQUAL,
      // Equality
      "==" => TokenKind::TOKEN_EQUAL_EQUAL,
      "!" => TokenKind::TOKEN_NOT,
      "!=" => TokenKind::TOKEN_NOT_EQUAL,
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
      _ => TokenKind::TOKEN_POSSIBLE_IDENT(intern(value,),),
    }
  }
}

impl PartialEq for TokenKind {
  fn eq(&self, other:&Self,) -> bool {
    match (self, other,) {
      (Self::TOKEN_IDENTIFIER(_,), Self::TOKEN_IDENTIFIER(_,),) => true,
      (Self::TOKEN_STRING(_,), Self::TOKEN_STRING(_,),) => true,
      (Self::TOKEN_INT(_,), Self::TOKEN_INT(_,),) => true,
      (Self::TOKEN_FLOAT(_,), Self::TOKEN_FLOAT(_,),) => true,
      (Self::TOKEN_BOOL(_,), Self::TOKEN_BOOL(_,),) => true,
      (Self::TOKEN_ERROR(_,), Self::TOKEN_ERROR(_,),) => true,
      (Self::TOKEN_POSSIBLE_IDENT(_,), Self::TOKEN_POSSIBLE_IDENT(_,),) => true,
      _ => core::mem::discriminant(self,) == core::mem::discriminant(other,),
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
  pub newline:bool,
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
    let token = Token::new(Some(self.val.clone().as_str(),), loc,);
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

#[derive(Debug, Clone, Copy,)]
pub(super) struct Token {
  pub kind:TokenKind,
  pub loc:Location,
}

impl Token {
  /// Create a [`Token`] from a [`String`].
  pub fn new(value:Option<&str,>, loc:Location,) -> Token {
    match value {
      Some(str,) => Token {
        kind:TokenKind::new(str,),
        loc,
      },
      None => Token {
        kind:TokenKind::TOKEN_EOF,
        loc,
      },
    }
  }

  /// Convert a [`Token`]'s `kind`into `TOKEN_IDENTIFIER`.
  pub fn to_ident(&mut self,) {
    //Get the value stored in the
    if let TokenKind::TOKEN_POSSIBLE_IDENT(idx,) = self.kind {
      let val = lookup(idx,);
      if val.chars().nth(0,).unwrap().is_alphabetic() {
        self.kind = TokenKind::TOKEN_IDENTIFIER(idx,);
      }
      else {
        let idx_err = intern(&ParsingError::InvalidVarName(idx,).to_string(),);
        self.kind = TokenKind::TOKEN_ERROR(idx_err,);
      }
    }
  }

  pub fn precedence(&self,) -> u32 {
    // Need to add Or and And?

    // pub const ASSIGNMENT: i32  = 1;
    // pub const CONDITIONAL: i32 = 2;
    // pub const SUM: i32         = 3;
    // pub const PRODUCT: i32     = 4;
    // pub const EXPONENT: i32    = 5;
    // pub const PREFIX: i32      = 6;
    // pub const POSTFIX: i32     = 7;
    // pub const CALL: i32        = 8;

    match self.kind {
      // Assign expressions
      TokenKind::TOKEN_EQUAL | TokenKind::TOKEN_MINUS_EQUAL | TokenKind::TOKEN_PLUS_EQUAL | TokenKind::TOKEN_SLASH_EQUAL | TokenKind::TOKEN_STAR_EQUAL => 10,
      // Conditional expressions
      TokenKind::TOKEN_IF | TokenKind::TOKEN_ELSE => 20,
      // Math expressions
      TokenKind::TOKEN_STAR | TokenKind::TOKEN_SLASH => 30,
      TokenKind::TOKEN_MINUS | TokenKind::TOKEN_PLUS => 40,
      // Comparison expressions
      TokenKind::TOKEN_EQUAL_EQUAL
      | TokenKind::TOKEN_NOT_EQUAL
      | TokenKind::TOKEN_GREATER
      | TokenKind::TOKEN_GREATER_EQUAL
      | TokenKind::TOKEN_LESS
      | TokenKind::TOKEN_LESS_EQUAL => 60,
      // Anything else should cause the expression parsing to terminate
      _ => 0,
    }
  }

  ///Create a`TOKEN_ERROR(ParsingError::VarNotDeclared)` [`Token`].
  pub fn var_not_declared_token(value:Option<&str,>, loc:Location,) -> Token {
    let idx_err = intern(&ParsingError::VarNotDeclared.to_string(),);
    Token {
      kind:TokenKind::TOKEN_ERROR(idx_err,),
      loc,
    }
  }
}
