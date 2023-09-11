use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.peek() {
    Some(Token::Keyword(lexeme, _)) if lexeme == "null" => {
      lexer.consume();
      return Some(AstNode::NullLiteral);
    }
    _ => None,
  };
}
