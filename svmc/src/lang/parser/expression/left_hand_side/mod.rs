use crate::{lexer::Lexer, parser::AstNode};

use super::function;

pub mod new;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return function::call::parse(lexer).or_else(|| new::parse(lexer));
}
