use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

use super::assignment;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek() {
    Some(Token::Punctuator(p, _)) if p == "(" => {
      lexer.consume();
      let expression = assignment::parse(lexer).expect("Expected expression after '('");
      lexer
        .consume_if(|t| matches!(t, Token::Punctuator(p, _) if p == ")"))
        .expect("Expected ')' after expression");
      return Some(expression);
    }
    _ => None,
  }
}
