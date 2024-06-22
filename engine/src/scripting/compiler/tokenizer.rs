use super::{
  errors::ParsingError,
  token::{Chunk, Token, TokenKind},
};
// TODO:
// - Update so it takes a &str. Will probably require changes to the tokens as
//   well.
//

pub type TokenStream = Vec<Token,>;

///Second parsing pass.
/// - Creates identifier tokens.
fn second_pass(tokens:&mut TokenStream,) {
  // Attempt to coerce the token following a let or let mut into an identifier
  let mut expect_ident = false;

  // Iterate over the tokens, if a let token is encountered expect an identifier.
  for token in tokens {
    if token.kind == TokenKind::TOKEN_LET {
      expect_ident = true;
      continue;
    }
    if expect_ident {
      match token.kind {
        TokenKind::TOKEN_ERROR(ParsingError::ChunkNotRecognized(_,),) => token.kind = TokenKind::TOKEN_IDENTIFIER,
        // If mut is discovered and an identity is expected do not alter the flag
        TokenKind::TOKEN_MUT => continue,
        // If the token is anything other than a mut or a ChunkNotRecognized convert the
        // Token to a VarNotDeclared
        _ => token.kind = TokenKind::TOKEN_ERROR(ParsingError::VarNotDeclared,),
      }
      expect_ident = !expect_ident
    }
  }
}

pub fn tokenize(source:String,) -> TokenStream {
  let mut tokens = Vec::new();
  let mut chunk = Chunk::new();
  let mut is_string = false;

  for ch in source.chars() {
    match ch {
      '"' => {
        if is_string {
          chunk.push('\"',);
          tokens.push(chunk.to_token(),)
        }
        else {
          chunk.push('\"',)
        }
        is_string = !is_string;
      }
      ' ' => {
        if is_string {
          chunk.push(' ',);
        }
        else if chunk.len() > 0 {
          tokens.push(chunk.to_token(),)
        }
      }
      '\r' => {
        if is_string {
          chunk.push('\r',);
        }
        else if chunk.len() > 0 {
          tokens.push(chunk.to_token(),)
        }
      }
      '\t' => {
        if is_string {
          chunk.push('\t',);
        }
        else if chunk.len() > 0 {
          tokens.push(chunk.to_token(),)
        }
      }
      '\n' => {
        if is_string {
          chunk.push('\n',);
        }
        else if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
      }
      ';' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token(";",),);
      }
      '(' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token("(",),);
      }
      ')' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token(")",),);
      }
      '{' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token("{",),);
      }
      '}' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token("}",),);
      }
      '[' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token("[",),);
      }
      ']' => {
        if chunk.len() > 0 {
          tokens.push(chunk.to_token(),);
        }
        tokens.push(chunk.new_token("]",),);
      }
      _ => chunk.push(ch,),
    }
    chunk.next()
  }

  //Filter the comments out of source
  tokens = tokens.into_iter().filter(|token| !token.value.clone().unwrap().starts_with("//",),).collect();
  //Add the EOF token
  tokens.push(Token::new(None, chunk.loc(),),);

  second_pass(&mut tokens,);

  tokens
}

#[cfg(test)]
mod test {
  use super::tokenize;
  use crate::scripting::compiler::token::TokenKind;

  #[test]
  fn generate_chunks_pass_works() {
    let mut source = r#"()  [] {} let mut  "test string""#.to_string();
    let tokens = tokenize(source,);

    assert_eq!(tokens[0].value.clone().unwrap().as_str(), "(");
    assert_eq!(tokens[0].loc.col, 1);
    assert_eq!(tokens[1].value.clone().unwrap().as_str(), ")");
    assert_eq!(tokens[1].loc.col, 2);
    assert_eq!(tokens[2].value.clone().unwrap().as_str(), "[");
    assert_eq!(tokens[2].loc.col, 5);
    assert_eq!(tokens[3].value.clone().unwrap().as_str(), "]");
    assert_eq!(tokens[3].loc.col, 6);
    assert_eq!(tokens[4].value.clone().unwrap().as_str(), "{");
    assert_eq!(tokens[4].loc.col, 8);
    assert_eq!(tokens[5].value.clone().unwrap().as_str(), "}");
    assert_eq!(tokens[5].loc.col, 9);
    assert_eq!(tokens[6].value.clone().unwrap().as_str(), "let");
    assert_eq!(tokens[6].loc.col, 11);
    assert_eq!(tokens[7].value.clone().unwrap().as_str(), "mut");
    assert_eq!(tokens[7].loc.col, 15);
    assert_eq!(tokens[8].value.clone().unwrap().as_str(), "\"test string\"");
    assert_eq!(tokens[8].loc.col, 19);
  }

  #[test]
  fn test_parse_for_non_identifiers() {
    let source = "



    = (+ / * >=  \t  == , fn for != >) 
    ///jdjdjdjdjd
    true
    "
    .to_string();

    let tokens = tokenize(source,);

    assert_eq!(tokens.len(), 15);
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
    assert_eq!(tokens[10].kind, TokenKind::TOKEN_NOT_EQUAL);
    assert_eq!(tokens[11].kind, TokenKind::TOKEN_GREATER);
    assert_eq!(tokens[12].kind, TokenKind::TOKEN_RIGHT_PAREN);
    assert_eq!(tokens[13].kind, TokenKind::TOKEN_BOOL);
    assert_eq!(tokens[14].kind, TokenKind::TOKEN_EOF);
  }

  #[test]
  fn parsing_literals_and_catching_identifiers_works() {
    let source = r#"
    let mut test = "test string";
    let test1 = 15;
    let test2 = 4.2;
    "#
    .to_string();

    let tokens = tokenize(source,);

    // Test first statement
    assert_eq!(tokens[0].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[1].kind, TokenKind::TOKEN_MUT);
    assert_eq!(tokens[2].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[2].value.clone().unwrap(), &"test".to_string());
    assert_eq!(tokens[3].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[4].kind, TokenKind::TOKEN_STRING);
    assert_eq!(&tokens[4].value.clone().unwrap(), &"\"test string\"".to_string());
    assert_eq!(tokens[5].kind, TokenKind::TOKEN_SEMICOLON);
    // Test second statement
    assert_eq!(tokens[6].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[7].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[7].value.clone().unwrap(), &"test1".to_string());
    assert_eq!(tokens[8].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[9].kind, TokenKind::TOKEN_INT);
    assert_eq!(&tokens[9].value.clone().unwrap(), &"15".to_string());
    assert_eq!(tokens[10].kind, TokenKind::TOKEN_SEMICOLON);
    // // Test third statement
    assert_eq!(tokens[11].kind, TokenKind::TOKEN_LET);
    assert_eq!(tokens[12].kind, TokenKind::TOKEN_IDENTIFIER);
    assert_eq!(&tokens[12].value.clone().unwrap(), &"test2".to_string());
    assert_eq!(tokens[13].kind, TokenKind::TOKEN_EQUAL);
    assert_eq!(tokens[14].kind, TokenKind::TOKEN_FLOAT);
    assert_eq!(&tokens[14].value.clone().unwrap(), &"4.2".to_string());
    assert_eq!(tokens[15].kind, TokenKind::TOKEN_SEMICOLON);
  }
}
