use crate::{lexer::Lexer, parser::expression};

use super::AstNode;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return expression::parse(lexer);
}
