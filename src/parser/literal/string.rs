use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
  runner::{self, run::Run, value::Value},
};

#[derive(Debug, Clone)]
pub struct StringLiteral {
  pub literal: String,
}

impl StringLiteral {
  pub fn new(literal: String) -> AstNode {
    return AstNode::StringLiteral(Self { literal });
  }
}

impl Run for StringLiteral {
  fn run(&self, _: &mut runner::scope::Scope) -> Value {
    let value = self.literal.clone()[1..self.literal.len() - 1].to_string();
    return Value::String(value);
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.peek() {
    Some(Token::StringLiteral(lexeme, _)) => {
      let literal = lexeme.clone();
      lexer.consume();
      return Some(StringLiteral::new(literal));
    }
    _ => None,
  };
}
