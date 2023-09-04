use crate::lexer::{Consumer, Token};

use super::AstNode;

pub trait Parser {
  fn parse<'a>(lexer: &mut Consumer<Token>) -> Option<AstNode>;
}
