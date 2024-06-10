use super::{
  errors::ParsingError,
  token::{Location, Token, TokenKind}
};

fn tokenize(mut source:String) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut loc = Location::new();

  // Tidy up the code
  source = source.replace("(", "( ").replace(")", " )").replace("{", "{ ").replace("}", " }");

  let chunks = source
    .split(&[' ', '\t', '\r'])
    .filter(|&s| !s.is_empty() && !s.starts_with("///") && !s.starts_with("//"))
    .collect::<Vec<&str>>();

  // Flag which tracks whether the next token is an identifer
  let mut expect_ident = false;

  while loc.index < chunks.len() {
    // Get the chunk
    let chunk = chunks[loc.index];

    // Update the next location
    if chunk.starts_with("\n") {
      loc.next(true);
      continue;
    }
    else {
      loc.next(false);
    }

    // If the preceding token is a TOKEN_LET or the 2 preciding tokens are
    // TOKEN_LET, TOKEN_MUT return a TOKEN_IDENTIFIER
    if expect_ident {
      // Create a token from the chunk to the tokens vec
      let mut token = Token::new(Some(chunk), loc);

      //If the next token is not mut or an error push an error
      if token.kind != TokenKind::TOKEN_ERROR(ParsingError::CharNotRecognized(chunk.to_string())) || token.kind != TokenKind::TOKEN_MUT {
        token = Token::var_not_declared_token(Some(chunk), loc);
      }
      else {
        if token.kind != TokenKind::TOKEN_MUT {
          token = Token::new_ident(Some(chunk), loc);
          expect_ident = false
        }
      }
      tokens.push(token);
    }
    else {
      // Create a token from the chunk to the tokens vec
      let token = Token::new(Some(chunk), loc);

      // Check if the token indicates an identifier is next
      if token.kind == TokenKind::TOKEN_LET {
        expect_ident = true
      }

      tokens.push(token);
    }

    if loc.index == source.len() {
      break;
    }
  }

  tokens.push(Token::new(None, loc));
  tokens
}

#[cfg(test)]
mod test {
  use super::tokenize;
  use crate::scripting::compiler::token::TokenKind;

  #[test]
  fn test_parse_for_non_identifiers() {
    let source = "



    = (  + / * >=  \t  == , fn for != >) 
    //skdjfjdjdjdjdjdj
    "
    .to_string();
    let tokens = tokenize(source);

    assert_eq!(tokens[0].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[1].kind, TokenKind::TOKEN_LEFT_PAREN);
    assert_eq!(tokens[2].kind, TokenKind::TOKEN_PLUS);
    assert_eq!(tokens[3].kind, TokenKind::TOKEN_SLASH);
    assert_eq!(tokens[4].kind, TokenKind::TOKEN_STAR);
    assert_eq!(tokens[5].kind, TokenKind::TOKEN_GREATER_EQUAL);
    assert_eq!(tokens[6].kind, TokenKind::TOKEN_EQUAL_EQUAL);
    assert_eq!(tokens[7].kind, TokenKind::TOKEN_COMMA);
    assert_eq!(tokens[8].kind, TokenKind::TOKEN_FN);
    assert_eq!(tokens[9].kind, TokenKind::TOKEN_FOR);
    assert_eq!(tokens[10].kind, TokenKind::TOKEN_BANG_EQUAL);
    assert_eq!(tokens[11].kind, TokenKind::TOKEN_GREATER);
    assert_eq!(tokens[12].kind, TokenKind::TOKEN_RIGHT_PAREN);
  }

  #[test]
  fn parsing_literals() {
    let source = r#"
    let mut test = "test string";
    let test1 = 15;
    let test2 = 4.2;
    "#
    .to_string();

    let tokens = tokenize(source);

    // Test first statement
    assert_eq!(tokens[0].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[1].kind, TokenKind::TOKEN_MUT);
    assert_eq!(tokens[2].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[2].value.clone().unwrap(), &"test".to_string());
    assert_eq!(tokens[3].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[4].kind, TokenKind::TOKEN_STRING);
    assert_eq!(&tokens[4].value.clone().unwrap(), &"test string".to_string());
    assert_eq!(tokens[5].kind, TokenKind::TOKEN_SEMICOLON);
    // Test second statement
    assert_eq!(tokens[6].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[7].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[7].value.clone().unwrap(), &"test1".to_string());
    assert_eq!(tokens[8].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[9].kind, TokenKind::TOKEN_INT);
    assert_eq!(&tokens[9].value.clone().unwrap(), &"15".to_string());
    assert_eq!(tokens[10].kind, TokenKind::TOKEN_SEMICOLON);
    // Test third statement
    assert_eq!(tokens[11].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[12].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[12].value.clone().unwrap(), &"test2".to_string());
    assert_eq!(tokens[13].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[14].kind, TokenKind::TOKEN_INT);
    assert_eq!(&tokens[14].value.clone().unwrap(), &"4.2".to_string());
    assert_eq!(tokens[15].kind, TokenKind::TOKEN_SEMICOLON);
  }
}
