pub mod bin;
pub mod decimal;
pub mod hex;
pub mod oct;

use crate::{lexer::Lexer, parser::AstNode};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return decimal::parse(lexer)
    .or(bin::parse(lexer))
    .or(oct::parse(lexer))
    .or(hex::parse(lexer));
}
