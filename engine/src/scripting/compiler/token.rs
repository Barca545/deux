use super::errors::ParsingError;

// TODO:
// - Debating getting rid of the difference between floats and ints and just
//   using floats only

#[derive(Debug, Clone, Copy)]
pub(super) struct Location {
  pub(super) line:u32,
  pub(super) col:u32,
  pub(super) index:usize
}

impl Location {
  /// Create a new [`Location`].
  pub fn new() -> Self {
    Location { line:0, col:0, index:0 }
  }

  pub fn next(&mut self, newline:bool) {
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

  ///Reads through the whole source code until it locates the target line.
  /// Returns a string pointing to where in the line the error occured.
  fn dbg(&self, raw:&String, error:ParsingError) -> Result<(), String> {
    let mut line = 0;
    let mut line_string = String::new();

    // Find the whole line of original source
    for char in raw.chars() {
      if char == '\n' {
        line += 1;

        // If a linebreak was reached and the line is not empty than we have finished
        // searching the line.
        if !line_string.is_empty() {
          break;
        }
        continue;
      }

      if self.line == line {
        line_string.push(char);
      }
    }

    //Create the indicator to the error
    let indicator = "-".repeat(self.col as usize);
    Err(format!("{}\n\n{}\n{}^ Near here", error.to_string(), line_string, indicator))
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
  fn new(value:&str) -> Self {
    // Return early if the token kind is a string
    if value.starts_with('"') {
      // Check if the string terminates, return a TOKEN_ERROR if it does not
      return match value.chars().max().unwrap() == '"' {
        true => TokenKind::TOKEN_STRING,
        false => TokenKind::TOKEN_ERROR(ParsingError::StringNotTerminated)
      };
    }

    // Return early if the token kind is a number
    if value.chars().nth(0).unwrap().is_numeric() {
      // Check if the token is an int or a float
      if value.parse::<f32>().is_ok() {
        return TokenKind::TOKEN_FLOAT;
      }
      else if value.parse::<u32>().is_ok() {
        return TokenKind::TOKEN_INT;
      }
      else {
        return TokenKind::TOKEN_ERROR(ParsingError::NotValidNumber(value.to_string()));
      }
    }

    match value {
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
      _ => TokenKind::TOKEN_ERROR(ParsingError::CharNotRecognized(value.to_string()))
    }
  }
}

#[derive(Debug, Clone)]
pub(super) struct Token {
  pub value:Option<String>,
  pub kind:TokenKind,
  pub loc:Location
}

impl Token {
  /// Create a [`Token`] from a [`String`].
  pub fn new(value:Option<&str>, loc:Location) -> Token {
    match value {
      Some(str) => Token {
        value:Some(str.to_string()),
        kind:TokenKind::new(str),
        loc
      },
      None => Token {
        value:None,
        kind:TokenKind::TOKEN_EOF,
        loc
      }
    }
  }

  /// Create an Identifier [`Token`] from a [`String`].
  pub fn new_ident(value:Option<&str>, loc:Location) -> Token {
    // Confirm the token is a valid identifier
    let val = value.unwrap().to_string();
    if val.chars().nth(0).unwrap().is_alphabetic() {
      return Token {
        value:Some(val.clone()),
        kind:TokenKind::TOKEN_ERROR(ParsingError::InvalidVarName(val)),
        loc
      };
    }
    else {
      Token {
        value:Some(val),
        kind:TokenKind::TOKEN_IDENTIFIER,
        loc
      }
    }
  }

  pub fn var_not_declared_token(value:Option<&str>, loc:Location) -> Token {
    Token {
      value:Some(value.unwrap().to_string()),
      kind:TokenKind::TOKEN_ERROR(ParsingError::VarNotDeclared),
      loc
    }
  }
}
