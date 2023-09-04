use crate::lexer::{Consumer, Token};
use crate::parser::AstNode;

#[derive(Debug)]
pub struct NumberLiteral {
  pub raw: String,
}

pub fn parse<'a>(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  match lexer.lookahead() {
    Some(Token::NumberLiteral(raw, _)) => {
      let raw_value = raw.clone();

      lexer.consume();

      return Some(AstNode::NumberLiteral(NumberLiteral { raw: raw_value }));
    }
    _ => None,
  }
}
