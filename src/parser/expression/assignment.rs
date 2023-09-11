use crate::{
  lexer::Lexer,
  operator,
  parser::{
    expression::{self, conditional, left_hand_side},
    AstNode,
  },
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return conditional::parse(lexer).or_else(|| {
    expression::binary::parse(
      lexer,
      operator!(
        "="
          | "&&="
          | "||="
          | "??="
          | "*="
          | "/="
          | "%="
          | "+="
          | "-="
          | "<<="
          | ">>="
          | ">>>="
          | "&="
          | "^="
          | "|="
          | "**="
      ),
      left_hand_side::parse,
      conditional::parse,
    )
  });
}
