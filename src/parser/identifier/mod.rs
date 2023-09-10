use crate::lexer::Lexer;

use super::AstNode;

pub mod name;
pub mod this;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return this::parse(lexer).or(name::parse(lexer));
}
