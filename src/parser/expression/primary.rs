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
    .or(literal::parse(lexer))
    .or(array::parse(lexer))
    .or(object::parse(lexer))
    .or(function::parse(lexer))
    .or(parenthesis::parse(lexer));
}
