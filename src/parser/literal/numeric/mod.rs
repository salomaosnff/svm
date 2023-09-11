pub mod bin;
pub mod decimal;
pub mod hex;
mod number_literal;
pub mod oct;

pub use number_literal::NumberLiteral;

use crate::{lexer::Lexer, parser::AstNode};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return decimal::parse(lexer)
    .or(bin::parse(lexer))
    .or(oct::parse(lexer))
    .or(hex::parse(lexer));
}
