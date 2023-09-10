use crate::lexer::Lexer;

use super::AstNode;

pub mod array;
pub mod boolean;
mod literal;
pub mod null;
pub mod numeric;
pub mod object;
pub mod string;
pub use literal::Literal;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return null::parse(lexer)
    .or(boolean::parse(lexer))
    .or(numeric::parse(lexer))
    .or(string::parse(lexer));
}
