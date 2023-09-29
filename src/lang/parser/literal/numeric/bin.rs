use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

use super::NumberLiteral;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let token = lexer.consume_if(|c| match c {
    Token::NumericLiteral(digits, _) => digits.as_str() == "0b",
    _ => false,
  })?;

  match token {
    Token::NumericLiteral(digits, _) => {
      return Some(NumberLiteral::new(digits.to_string()));
    }
    _ => unreachable!(),
  }
}
