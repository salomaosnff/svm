use crate::{
  lexer::Lexer,
  operator,
  parser::{
    expression::{arithmetic::additive, binary},
    AstNode,
  },
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("<<" | ">>" | ">>>"),
    additive::parse,
    additive::parse,
  );
}
