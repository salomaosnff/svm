use crate::{
  lexer::Lexer,
  parser::{
    identifier,
    literal::{self, array, object},
    AstNode,
  },
};

use super::{function, parenthesis};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return identifier::parse(lexer)
    .or_else(|| literal::parse(lexer))
    .or_else(|| array::parse(lexer))
    .or_else(|| object::parse(lexer))
    .or_else(|| function::parse(lexer))
    .or_else(|| parenthesis::parse(lexer));
}
