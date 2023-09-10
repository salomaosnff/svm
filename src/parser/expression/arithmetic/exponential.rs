use crate::{
  lexer::Lexer,
  operator,
  parser::{
    expression::{self, binary},
    AstNode,
  },
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return binary::parse(
    lexer,
    operator!("**"),
    expression::unary::parse,
    expression::unary::parse,
  );
}
