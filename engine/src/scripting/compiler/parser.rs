use super::{
  ast::{AbstractSyntaxTree, BinOpKind, Expression, ExpressionKind, Ident, Literal, LiteralKind, Local, LocalKind, Pat, PatKind, Statement, Ty},
  errors::ParsingError,
  symbol_table::Symbol,
  token::{Location, Token, TokenKind},
  tokenizer::{tokenize, TokenStream},
};
use crate::scripting::compiler::ast::{StatementKind, P};
use eyre::Result;

// Refactor:
// - Switch to using lifetimes for tokens and the parser (must be done before
//   switching to peekable)
// - Switch to iter().peekable() instead of just a vec with methods. I think
//   then all `current()`s become `next()`s
// - next has higher precedence might need a less than sign
// - BinOps should error if both sides are not the same type
// - Panics need to be turned into actual errors
// - I don't think I need an else if token since else takes an expression, the
//   next expression can just be another if
// - Should if statements error if they reach an EOF without another brace?

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

  /// Checks whether the current [`Token`] has a higher precedence than the
  /// following `Token`.
  fn next_is_less_precedence(&self, rbp:u32,) -> bool {
    self.peek().precedence() > rbp
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

  ///Consume a [`TokenStream`] and return an [`AbstractSyntaxTree`].
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
  fn parse_expression(&mut self, rbp:u32,) -> Expression {
    self.next();
    // Get the left hand expression
    let mut left = self.parse_expression_null_detonation();

    // Keep eating tokens as long as the next token's precedence is greater than
    // the current token -- lower precedence will cause the loop to terminate
    while self.next_is_less_precedence(rbp,) {
      left = self.parse_expression_left_denotation(left,)
    }
    left
  }

  ///Returns an [`Expression`] consisting of the current [`Token`].
  fn parse_expression_null_detonation(&mut self,) -> Expression {
    // Should be some kind of literal
    let token = self.current();

    match token.kind {
      TokenKind::TOKEN_INT => self.parse_integer(token,),
      TokenKind::TOKEN_FLOAT => self.parse_float(token,),
      TokenKind::TOKEN_BOOL => self.parse_bool(token,),
      //If statement
      TokenKind::TOKEN_IF => {
        dbg!("reached 5");
        self.parse_if_expression(&token,)
      }

      // This should probably enter an error state
      // I think it should error that it was expecting some kind of literal
      _ => {
        self.err_detected(token.loc, &ParsingError::UnexpectedToken(token.value.unwrap(),),);
        panic!();
      }
    }
  }

  ///Returns an [`Expression`] consisting of the currently parsed [`Token`]s
  /// and the next `Token`.
  fn parse_expression_left_denotation(&mut self, left:Expression,) -> Expression {
    self.next();
    //Should be some kind of operator
    let token = self.current();
    match token.kind {
      //BinOps
      TokenKind::TOKEN_MINUS
      | TokenKind::TOKEN_PLUS
      | TokenKind::TOKEN_STAR
      | TokenKind::TOKEN_SLASH
      | TokenKind::TOKEN_EQUAL_EQUAL
      | TokenKind::TOKEN_NOT_EQUAL
      | TokenKind::TOKEN_GREATER
      | TokenKind::TOKEN_GREATER_EQUAL
      | TokenKind::TOKEN_LESS
      | TokenKind::TOKEN_LESS_EQUAL => {
        let loc = left.loc;
        let binop_kind = BinOpKind::from(&token,);
        let right = self.parse_expression(token.precedence(),);
        let kind = ExpressionKind::BinOp(P::new(left,), binop_kind, P::new(right,),);
        Expression { id:0, kind, loc, }
      }

      _ => panic!("{:?} {}", token.kind, token.value.unwrap()),
    }
  }

  ///Return an expression containing an integer [`Literal`].
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

  ///Return an expression containing a float [`Literal`].
  fn parse_float(&mut self, token:Token,) -> Expression {
    let val = Literal {
      kind:LiteralKind::Float,
      symbol:Symbol,
    };

    Expression {
      id:0,
      kind:ExpressionKind::Literal(P::new(val,),),
      loc:token.loc,
    }
  }

  ///Return an expression containing a boolean [`Literal`].
  fn parse_bool(&mut self, token:Token,) -> Expression {
    let val = Literal {
      kind:LiteralKind::Bool,
      symbol:Symbol,
    };

    Expression {
      id:0,
      kind:ExpressionKind::Literal(P::new(val,),),
      loc:token.loc,
    }
  }

  fn parse_if_expression(&mut self, token:&Token,) -> Expression {
    dbg!("reached here 1");
    // Parse the conditional expression
    let condition = self.parse_expression(0,);
    let loc = token.loc;

    dbg!("reached here 2");

    // Parse the statement making up its body
    self
      .eat_token_expect(TokenKind::TOKEN_LEFT_BRACE, "Must follow if statement with a brace `{`",)
      .unwrap();

    dbg!("reached here 3");

    let mut if_body = Vec::new();

    while self.current().kind != TokenKind::TOKEN_RIGHT_BRACE {
      if_body.push(self.parse_statement().unwrap(),);
    }

    // Check for an else statment and parse it if so
    let else_body = if self.eat_token_if_match(TokenKind::TOKEN_ELSE,) {
      let else_expr = self.parse_expression(0,);
      Some(P::new(else_expr,),)
    }
    else {
      None
    };

    Expression {
      id:0,
      kind:ExpressionKind::If(P::new(condition,), P::new(if_body,), else_body,),
      loc,
    }
  }

  ///Returns a [`Statement`].
  fn parse_statement(&mut self,) -> Result<Statement,> {
    let token = self.current();

    match token.kind {
      TokenKind::TOKEN_LET => Ok(self.parse_let(&token,),),
      _ => Ok(self.parse_expression_statement(&token,),),
      // Err(ParsingError::NoStatementMatch.into(),)
    }
  }

  ///Parse a `let` [`Statement`] (`let <pat>::<ty> = <expr>`).
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

  ///Parse an [`Expression`] as a [`Statement`].
  fn parse_expression_statement(&mut self, token:&Token,) -> Statement {
    let loc = token.loc;
    let expression = self.parse_expression(0,);
    dbg!(token.value.clone());
    dbg!(expression.clone());

    Statement {
      id:0,
      loc,
      kind:StatementKind::Expression(expression,),
    }
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
      let expression = self.parse_expression(0,);
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
  use crate::scripting::compiler::ast::{BinOpKind, ExpressionKind, LiteralKind, LocalKind, StatementKind};

  #[test]
  fn parse_works() {
    let source = String::from(
      // erroring in the loop a few problems
      // - Not ignoring comments -> ensure the tokenizer skips over everything between `//` and a linebreak
      // -Seems to be expecting the inside of an if expression to be an expression and not a vec of statements
      //  -> it tries to go into the rest of the expression parsing so it expects let to be an expression because it's treating it
      // life the lefthand side

      //Look into what the null detonation is supposed to be, because currently the if has to be there instead of the left

      //might also just never be reaching the if branch for some reason
      //seems like it thinks the { begins a new statement

      //it skips past the if because of the next in the parse expressions function
      // the problem is for *just* parsing expressions it needs the next()
      // but for parsing  expression statements it can't have the next

      //Still have a precedence problem with the if's inner

      // let mut value_test = 5 + 10;
      // let value = true != false == true;
      r#"

      if true {
        let a = 90;
      }
    "#,
    );

    let mut parser = Parser::new(source,);
    let ast = parser.parse().unwrap();

    //Check the AST is correct
    match ast.statements.clone()[0].kind.clone() {
      StatementKind::Let(local,) => {
        match &local.kind {
          LocalKind::Init(expr,) => match &expr.kind {
            ExpressionKind::BinOp(lhs, op, rhs,) => {
              //Check the value of the lhs
              match &lhs.kind {
                ExpressionKind::Literal(lit,) => assert_eq!(&lit.kind, &LiteralKind::Integer),
                _ => panic!(),
              }

              //Check the value of the rhs
              match &lhs.kind {
                ExpressionKind::Literal(lit,) => assert_eq!(&lit.kind, &LiteralKind::Integer),
                _ => panic!(),
              }

              //Check the value of the op
              assert_eq!(*op, BinOpKind::PLUS);
            }
            _ => panic!(),
          },
          _ => panic!(),
        };
      }
      _ => panic!(),
    }

    dbg!(ast.statements.clone()[1].kind.clone());
  }
}
