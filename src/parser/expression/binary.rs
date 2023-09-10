use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

#[derive(Debug)]
pub struct BinaryExpression {
  pub operator: String,
  pub left: Box<AstNode>,
  pub right: Box<AstNode>,
}

impl BinaryExpression {
  pub fn new(operator: String, left: AstNode, right: AstNode) -> AstNode {
    return AstNode::BinaryExpression(Self {
      operator,
      left: Box::new(left),
      right: Box::new(right),
    });
  }
}

pub fn parse<
  O: Fn(&str) -> bool,
  L: Fn(&mut Lexer) -> Option<AstNode>,
  R: Fn(&mut Lexer) -> Option<AstNode>,
>(
  lexer: &mut Lexer,
  operator: O,
  left: L,
  right: R,
) -> Option<AstNode> {
  let mut exp = left(lexer)?;

  loop {
    match lexer.lookahead() {
      Some(Token::Punctuator(p, _)) if operator(p) => {
        let op = p.clone();
        lexer.consume();
        exp = BinaryExpression::new(op, exp, right(lexer)?);
      }
      _ => break,
    }
  }

  return Some(exp);
}
