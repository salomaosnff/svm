use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

#[derive(Debug)]
pub enum BooleanLiteral {
  True,
  False,
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.lookahead() {
    Some(Token::Keyword(lexeme, _)) if lexeme == "true" => {
      lexer.consume();
      return Some(AstNode::BooleanLiteral(BooleanLiteral::True));
    }
    Some(Token::Keyword(lexeme, _)) if lexeme == "false" => {
      lexer.consume();
      return Some(AstNode::BooleanLiteral(BooleanLiteral::False));
    }
    _ => None,
  };
}
