use crate::{lexer::Lexer, operator, parser::AstNode};

use super::{binary, bitwise};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("??"),
    bitwise::or::parse,
    bitwise::or::parse,
  );
}
