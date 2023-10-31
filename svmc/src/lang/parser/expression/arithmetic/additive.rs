use crate::{
  lexer::Lexer,
  operator,
  parser::{expression::binary, AstNode},
};

use super::multiplicative;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("+" | "-"),
    multiplicative::parse,
    multiplicative::parse,
  );
}
