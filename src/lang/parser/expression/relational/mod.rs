use crate::{lexer::Lexer, operator, parser::AstNode};

use super::binary;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("<" | ">" | "<=" | ">="),
    super::bitwise::shift::parse,
    super::bitwise::shift::parse,
  );
}
