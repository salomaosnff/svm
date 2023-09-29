use crate::{lexer::Lexer, parser::AstNode};

use super::{coalesce, logical::or};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return or::parse(lexer).or_else(|| coalesce::parse(lexer));
}
