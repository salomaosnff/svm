use crate::{
  lexer::{Lexer, Token},
  parser::{operators::assignment, AstNode},
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.lookahead() {
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
