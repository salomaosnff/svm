use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

#[derive(Debug, Clone)]
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
  left_node: L,
  right_node: R,
) -> Option<AstNode> {
  let mut exp = left_node(lexer)?;

  loop {
    match lexer.peek() {
      Some(Token::Punctuator(p, _)) if operator(p.as_str()) => {
        let op = p.clone();
        lexer.consume();
        let right = right_node(lexer).expect("Expected right side of binary expression");
        exp = BinaryExpression::new(op, exp, right);
      }
      _ => break,
    }
  }

  return Some(exp);
}
