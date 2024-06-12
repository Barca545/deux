use super::tokenizer::TokenStream;

struct Node {
  index:usize,
  kind:ExpressionKind,
  tokens:TokenStream,
  descendants:Vec<Node,>,
}

enum ExpressionKind {}

pub struct AbstractSyntaxTree {}

impl AbstractSyntaxTree {
  pub fn new() -> Self {
    AbstractSyntaxTree {}
  }
}
