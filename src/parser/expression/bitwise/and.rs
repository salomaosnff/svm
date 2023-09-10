use crate::{
  lexer::Lexer,
  operator,
  parser::{expression, AstNode},
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return expression::binary::parse(
    lexer,
    operator!("&"),
    expression::equality::parse,
    expression::equality::parse,
  );
}
