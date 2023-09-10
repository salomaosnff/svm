use crate::{
  lexer::Lexer,
  operator,
  parser::{
    expression::{binary, bitwise},
    AstNode,
  },
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("|"),
    bitwise::xor::parse,
    bitwise::xor::parse,
  );
}
