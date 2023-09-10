use crate::{lexer::Lexer, parser::AstNode};

use super::function;

pub mod new;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return new::parse(lexer).or(function::call::parse(lexer));
}
