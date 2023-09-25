use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
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
