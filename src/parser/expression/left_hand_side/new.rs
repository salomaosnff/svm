use crate::{
  lexer::{Lexer, Token},
  parser::{expression::member, AstNode},
};

#[derive(Debug)]
pub struct NewExpression {
  pub callee: Box<AstNode>,
}

impl NewExpression {
  pub fn new(callee: AstNode) -> AstNode {
    return AstNode::NewExpression(Self {
      callee: Box::new(callee),
    });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut exp = member::parse(lexer)?;

  loop {
    match lexer.lookahead() {
      Some(Token::Keyword(lexeme, _)) if lexeme == "new" => {
        lexer.consume();

        exp = NewExpression::new(exp);
      }
      _ => break,
    }
  }

  return Some(exp);
}
