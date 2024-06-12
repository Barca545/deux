use super::{
  ast::AbstractSyntaxTree,
  errors::ParsingError,
  token::{Location, Token, TokenKind},
  tokenizer::{tokenize, TokenStream},
};
use eyre::Result;

// TO DO:
// - Use ANSI escape codes to color the errors

//I think what he calls consume is the function that emits bytecode but I don't
// want to do that yet, I just want to make an AST

///Structure used to generate an AST from a [`TokenStream`].
struct Parser {
  current:Token,
  previous:Option<Token,>,
  source:String,
  tokens:TokenStream,
  had_err:bool,
  erroring:bool,
}

impl Parser {
  ///Load a [`TokenStream`] into the [`Parser`].
  fn new(tokens:TokenStream, source:String,) -> Self {
    Parser {
      current:tokens[0].clone(),
      previous:None,
      source,
      tokens,
      had_err:false,
      erroring:false,
    }
  }

  ///Returns the next [`Token`].
  fn peek(&self,) -> Token {
    todo!()
  }

  ///Loop through the [`TokenStream`] and read the [`Token`]s.
  fn advance(&mut self,) {
    self.previous = Some(self.current.clone(),);

    //Print an error if one is encountered
    for token in self.tokens.clone() {
      if let TokenKind::TOKEN_ERROR(err,) = &token.kind {
        self.err_detected(token.loc, err,);
      }
    }
  }

  ///Read the next [`Token`].
  ///
  ///# Panics
  ///Prints an error if a `Token` cannot be parsed.
  fn consume(&mut self,) -> Result<(),> {
    if self.had_err {
      return Err(ParsingError::UnableToParse.into(),);
    }
    todo!()
  }

  ///Print an error message indicating where in the source file the error
  /// occurred.
  fn err_detected(&mut self, loc:Location, err:&ParsingError,) {
    self.had_err = true;
    //Print the error
    Parser::print_error(loc, &self.source, err,);

    //Enter panic mode and keep consuming tokens until a synchronization point is
    // reached
    self.erroring = true;
    todo!()
  }

  fn parse(source:String,) -> Result<AbstractSyntaxTree,> {
    //Create the TokenStream
    let tokens = tokenize(source.clone(),);

    //Create the parser
    let mut parser = Parser::new(tokens, source,);

    parser.advance();
    parser.consume()?;

    let ast = AbstractSyntaxTree::new();

    Ok(ast,)
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
      r#"let mut value_test = value;
    "#,
    );

    let ast = Parser::parse(source,).unwrap();
  }
}
