use crate::{lexer::Lexer, parser::expression::function};

use super::AstNode;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return function::parse(lexer);
}
