use crate::{
  lexer::{Lexer, Token},
  parser::{literal::Literal, AstNode},
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let token = lexer.consume_if(|c| match c {
    Token::NumericLiteral(digits, _) => digits.as_str() == "0o",
    _ => false,
  })?;

  match token {
    Token::NumericLiteral(digits, _) => {
      return Some(Literal::new(digits.to_string()));
    }
    _ => unreachable!(),
  }
}
