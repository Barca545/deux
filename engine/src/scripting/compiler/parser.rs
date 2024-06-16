use crate::scripting::compiler::ast::{StatementKind, P};

use super::{
  ast::{AbstractSyntaxTree, Expression, ExpressionKind, Ident, Literal, LiteralKind, Local, LocalKind, Pat, PatKind, Statement, Ty},
  errors::ParsingError,
  symbol_table::Symbol,
  token::{Location, Token, TokenKind},
  tokenizer::{tokenize, TokenStream},
};
use eyre::Result;

// TO DO:
// - Use ANSI escape codes to color the errors
// - Roll the match statements in the parse function into a parse_rule() method

//I think what he calls consume is the function that emits bytecode but I don't
// want to do that yet, I just want to make an AST

// right now I'm separating the AST generation and bytecode generation. This
// might lead to duplicate code but should make adding IR steps later on a lot
// easier.

// I should finish up the parser first then do the symbol table based on that
// paper I found

///Structure used to generate an AST from a [`TokenStream`].
struct Parser {
  cursor:usize,
  source:String,
  tokens:TokenStream,
  had_err:bool,
  erroring:bool,
}

impl Parser {
  ///Load a [`TokenStream`] into the [`Parser`].
  fn new(source:String,) -> Self {
    let tokens = tokenize(source.clone(),);

    Parser {
      cursor:0,
      source,
      tokens,
      had_err:false,
      erroring:false,
    }
  }

  ///Increments the `cursor`.
  fn next(&mut self,) {
    self.cursor += 1;
  }

  ///Returns a reference to the current [`Token`].
  fn current(&self,) -> Token {
    self.tokens[self.cursor].clone()
  }

  ///Returns a reference to the next [`Token`].
  fn peek(&self,) -> &Token {
    &self.tokens[self.cursor + 1]
  }

  // ///Loop through the [`TokenStream`] and read the [`Token`]s.
  // fn advance(&mut self,) -> &Token {
  //   // self.cursor += 1;

  //   //Print an error if one is encountered
  //   // if let TokenKind::TOKEN_ERROR(err,) = &token.kind {
  //   //   self.err_detected(token.loc, err,);
  //   // }
  //   // for token in self.tokens.clone() {
  //   //   //Scan forward in the TokenStream
  //   //   self.cursor += 1;

  //   //   if let TokenKind::TOKEN_ERROR(err,) = &token.kind {
  //   //     self.err_detected(token.loc, err,);
  //   //   }
  //   // }
  // }

  ///Consumes the next [`Token`].
  ///
  ///# Panics
  /// - Panics if the `current` `Token` is not the expected [`TokenKind`].
  fn eat_token_expect(&mut self, token:TokenKind, msg:&str,) -> Result<(),> {
    self.next();
    if self.current().kind != token {
      self.had_err = true;
      return Err(ParsingError::UnexpectedToken(msg.to_string(),).into(),);
    }
    Ok((),)
  }

  /// If the next token is the given [`TokenKind`], returns true.
  fn check_keyword(&self, token:TokenKind,) -> bool {
    self.tokens[self.cursor + 1].kind == token
  }

  /// If the next token is the given keyword, eats it and returns `true`.
  /// Otherwise, returns `false`. An expectation is also added for diagnostics
  /// purposes.
  pub fn eat_token_if_match(&mut self, token:TokenKind,) -> bool {
    if self.check_keyword(token,) {
      self.next();
      true
    }
    else {
      false
    }
  }

  fn parse(&mut self,) -> Result<AbstractSyntaxTree,> {
    let mut ast = AbstractSyntaxTree::new();

    while self.tokens.len() > 0 && self.current().kind != TokenKind::TOKEN_EOF {
      ast.push(self.parse_statement().unwrap(),);
      self.next();
    }

    Ok(ast,)
  }
}

//Actual Parsing rules
impl Parser {
  ///Returns an [`Expression`].
  fn parse_expression(&mut self,) -> Expression {
    self.next();
    let token = self.current();
    match token.kind {
      TokenKind::TOKEN_INT => self.parse_integer(token,),
      _ => {
        todo!()
      }
    }
  }

  fn parse_integer(&mut self, token:Token,) -> Expression {
    let val = Literal {
      kind:LiteralKind::Integer,
      symbol:Symbol,
    };

    Expression {
      id:0,
      kind:ExpressionKind::Literal(P::new(val,),),
      loc:token.loc,
    }
  }

  ///Returns a [`Statement`].
  fn parse_statement(&mut self,) -> Result<Statement,> {
    // let lhs = self.next();

    let token = self.current();
    match token.kind {
      TokenKind::TOKEN_LET => Ok(self.parse_let(&token,),),
      _ => Err(ParsingError::NoStatementMatch.into(),),
    }
  }

  ///Parse a `let` statement (`let <pat>::<ty> = <expr>`)
  fn parse_let(&mut self, token:&Token,) -> Statement {
    //Location of the first element in the statement
    let id = 0;
    let loc = token.loc;

    //Check for mutability
    let mutable = self.parse_mutability();

    //Check for an ident and error if none
    let ident = self.parse_ident();

    //Check for type
    let ty = self.parse_type();

    //The pattern cannot be found until the full expression is parsed
    let pat = Pat {
      id,
      loc,
      kind:PatKind::Ident { mutable, ident, },
    };

    let local = Local {
      id,
      loc,
      ty,
      pat:P::new(pat,),
      kind:self.parse_local_kind(),
    };

    let stmt_kind = StatementKind::Let(P::new(local,),);

    Statement::new(loc, stmt_kind,)
  }

  ///Checks if the next [`Token`] is mutable and consumes it if so.
  fn parse_mutability(&mut self,) -> bool {
    self.eat_token_if_match(TokenKind::TOKEN_MUT,)
  }

  ///Checks if the next [`Token`]s are `:<ty>` and consumes them if so.
  fn parse_type(&mut self,) -> Option<P<Ty,>,> {
    //Check for a colon if a colon is found continue otherwise return false
    if self.eat_token_if_match(TokenKind::TOKEN_COLON,) {
      //Check for a type if no type is found print an error
      self.eat_token_expect(TokenKind::TOKEN_TYPE, "Expected type declaration",).unwrap();

      //If the type is not a valid type, error
      if let Some(ty,) = Ty::new(self.current().value.clone().unwrap(),) {
        return Some(P::new(ty,),);
      };

      self.err_detected(self.current().loc, &ParsingError::InvalidTpe(self.current().value.clone().unwrap(),),)
    }
    None
  }

  fn parse_ident(&mut self,) -> Ident {
    // Eat the token expecting an ident

    self
      .eat_token_expect(TokenKind::TOKEN_IDENTIFIER, &ParsingError::VarNotDeclared.to_string(),)
      .unwrap();
    Ident {
      name:self.current().value.clone().unwrap(),
      loc:self.current().loc,
    }
  }

  fn parse_local_kind(&mut self,) -> LocalKind {
    // If equals sign, parse the output of the next statement, expect an Expression.
    if self.eat_token_if_match(TokenKind::TOKEN_EQUAL,) {
      let expression = self.parse_expression();
      //Line must end after the expression so expect a semicolon
      self
        .eat_token_expect(TokenKind::TOKEN_SEMICOLON, "Must end Let statement with a semicolon",)
        .unwrap();
      return LocalKind::Init(P::new(expression,),);
    }
    // If equals no sign error it's a delayed assignment so expect a semicolon
    self
      .eat_token_expect(TokenKind::TOKEN_SEMICOLON, "Must end Let statement with a semicolon",)
      .unwrap();

    LocalKind::Decl
  }
}

// fn infix_precedence(infix:TokenKind,) -> u8 {
//   match infix {
//     TokenKind::TOKEN_EQUAL =>
//   }
// }

//Error reporting
impl Parser {
  ///Print an error message indicating where in the source file the error
  /// occurred.
  fn err_detected(&mut self, loc:Location, err:&ParsingError,) {
    //Surpress errors if the parser is in an error state because they will likely
    // be unhelpful
    if !self.erroring {
      self.had_err = true;
      //Print the error
      Parser::print_error(loc, &self.source, err,);

      //Enter panic mode and keep consuming tokens until a synchronization point is
      // reached
      self.erroring = true;
    }
    todo!()
  }

  ///Reads through the whole source code until it locates the target line.
  /// Print a string pointing to where in the line the error occured.
  fn print_error(loc:Location, raw:&String, err:&ParsingError,) {
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

      if loc.line == line {
        line_string.push(char,);
      }
    }

    //Create the indicator to the error
    let indicator = "_".repeat(loc.col as usize,);
    print!(
      "{}\n\n{}\n{}^ Panicked near ln:{} col:{} \n",
      err.to_string(),
      line_string,
      indicator,
      loc.line,
      loc.col
    );
  }
}

#[cfg(test)]
mod tests {
  use super::Parser;

  #[test]
  fn parse_works() {
    let source = String::from(
      r#"
      let mut value_test = 5;
      let value = 3;
    "#,
    );

    let mut parser = Parser::new(source,);
    let ast = parser.parse().unwrap();
  }
}
