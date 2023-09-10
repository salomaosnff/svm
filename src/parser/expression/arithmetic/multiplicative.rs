use crate::{
  lexer::Lexer,
  operator,
  parser::{expression::binary, AstNode},
};

use super::exponential;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("*" | "/" | "%"),
    exponential::parse,
    exponential::parse,
  );
}
