use crate::lexer::{Lexer, Token};

use super::AstNode;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek() {
    Some(Token::Keyword(lexeme, _)) if lexeme == "this" => {
      lexer.consume();
      return Some(AstNode::ThisExpression);
    }
    _ => None,
  }
}
