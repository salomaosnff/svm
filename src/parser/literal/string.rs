use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

use super::Literal;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.lookahead() {
    Some(Token::StringLiteral(lexeme, _)) => {
      let literal = lexeme.clone();
      lexer.consume();
      return Some(Literal::new(literal));
    }
    _ => None,
  };
}
