use super::and;
use crate::{
  lexer::Lexer,
  operator,
  parser::{expression::binary, AstNode},
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(lexer, operator!("||"), and::parse, and::parse);
}
