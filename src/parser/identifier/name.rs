use crate::lexer::{Lexer, Token};

use super::AstNode;

#[derive(Debug, Clone)]
pub struct IdentifierName {
  pub name: String,
}

impl IdentifierName {
  pub fn new(name: String) -> AstNode {
    AstNode::IdentifierName(Self { name })
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek() {
    Some(Token::IdentifierName(lexeme, _)) => {
      let name = lexeme.clone();
      lexer.consume();
      return Some(IdentifierName::new(name));
    }
    _ => None,
  }
}
