use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
  pub value: bool,
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.peek() {
    Some(Token::Keyword(lexeme, _)) if lexeme == "true" => {
      lexer.consume();
      return Some(AstNode::BooleanLiteral(BooleanLiteral { value: true }));
    }
    Some(Token::Keyword(lexeme, _)) if lexeme == "false" => {
      lexer.consume();
      return Some(AstNode::BooleanLiteral(BooleanLiteral { value: false }));
    }
    _ => None,
  };
}
