use super::token::{Location, Token, TokenKind};

// the letmut replace should ignore whitespace

// need to convert strings into single chunks

fn tokenize(mut source:String) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut loc = Location::new();

  merge_string_chunks(&mut source);

  // Tidy up the code
  source = source
    .replace("(", "( ")
    .replace(")", " )")
    .replace("{", "{ ")
    .replace("}", " }")
    .replace(";", " ; ")
    .replace("let mut", "letmut");

  let chunks = source
    .split(&[' ', '\t', '\r'])
    .filter(|&s| !s.is_empty() && !s.starts_with("///") && !s.starts_with("//"))
    .collect::<Vec<&str>>();

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
    tokens.push(Token::new(Some(chunk), loc));

    if loc.index == source.len() {
      break;
    }
  }

  tokens.push(Token::new(None, loc));
  second_pass(&mut tokens);
  tokens
}

///Second parsing pass.
/// - Creates identifier tokens.
fn second_pass(tokens:&mut Vec<Token>) {
  // Attempt to coerce the token following a let or letmut into and identifier
  for i in 0..tokens.len() {
    let current = &tokens[i];
    if current.kind == TokenKind::TOKEN_LET || current.kind == TokenKind::TOKEN_LET_MUT {
      // Ensure the vector is long enough or return an error
      if i + 1 > tokens.len() {
        tokens.push(Token::var_not_declared_token(None, current.loc));
      }
      else {
        let next = &mut tokens[i + 1];
        next.to_ident()
      }
    }
  }
}

/// Parse pass to merge strings into one chunk
fn merge_string_chunks(source:&mut String) {
  let mut result = String::new();
  let mut is_string = false;

  for ch in source.chars() {
    match ch {
      '"' => {
        is_string = !is_string;
        result.push('\"')
      }
      '\t' => {
        if is_string {
          result.push('_');
        }
        else {
          result.push('\t')
        }
      }
      '\r' => {
        if is_string {
          result.push('_');
        }
        else {
          result.push('\r')
        }
      }
      ' ' => {
        if is_string {
          result.push('_');
        }
        else {
          result.push(' ')
        }
      }
      _ => result.push(ch)
    }
  }
  *source = result;

  // while let Some(chunk) = chunks.into_iter().next() {
  //   if chunk.starts_with('"') {
  //     is_string = true;
  //   };
  // }
  // for i in 0..chunks.len() {
  //   let current = chunks[i];

  //   if current.starts_with('"') {
  //     is_string = true;
  //   };

  //   let next = chunks[i + 1];
  // }
}

#[cfg(test)]
mod test {
  use super::{merge_string_chunks, tokenize};
  use crate::scripting::compiler::token::TokenKind;

  #[test]
  fn generate_chunks_pass_works() {
    let mut source = r#"nonsense does not matter "test string""#.to_string();
    merge_string_chunks(&mut source);
    assert_eq!(r#"nonsense does not matter "test_string""#.to_string(), source)
  }

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
    assert_eq!(tokens[0].kind, TokenKind::TOKEN_LET_MUT);
    assert_eq!(tokens[1].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[1].value.clone().unwrap(), &"test".to_string());
    assert_eq!(tokens[2].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[3].kind, TokenKind::TOKEN_STRING);
    assert_eq!(&tokens[3].value.clone().unwrap(), &"\"test_string\"".to_string());
    assert_eq!(tokens[4].kind, TokenKind::TOKEN_SEMICOLON);
    // Test second statement
    assert_eq!(tokens[5].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[6].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[6].value.clone().unwrap(), &"test1".to_string());
    assert_eq!(tokens[7].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[8].kind, TokenKind::TOKEN_INT);
    assert_eq!(&tokens[8].value.clone().unwrap(), &"15".to_string());
    assert_eq!(tokens[9].kind, TokenKind::TOKEN_SEMICOLON);
    // // Test third statement
    assert_eq!(tokens[10].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[11].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[11].value.clone().unwrap(), &"test2".to_string());
    assert_eq!(tokens[12].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[13].kind, TokenKind::TOKEN_FLOAT);
    assert_eq!(&tokens[13].value.clone().unwrap(), &"4.2".to_string());
    assert_eq!(tokens[14].kind, TokenKind::TOKEN_SEMICOLON);
  }
}
