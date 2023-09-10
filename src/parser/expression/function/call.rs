use crate::{
  lexer::{Lexer, Token},
  parser::{expression::member, operators::assignment, AstNode},
};

use super::arguments;

#[derive(Debug)]
pub struct CallExpression {
  pub callee: Box<AstNode>,
  pub arguments: Vec<AstNode>,
}

impl CallExpression {
  pub fn new(callee: AstNode, arguments: Vec<AstNode>) -> AstNode {
    return AstNode::CallExpression(Self {
      callee: Box::new(callee),
      arguments,
    });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut exp = member::parse(lexer)?;

  loop {
    match lexer.lookahead() {
      Some(Token::Punctuator(p, _)) if p.to_string() == "(" => {
        exp = CallExpression::new(exp, arguments::parse(lexer));
      }
      _ => break,
    }
  }

  return Some(exp);
}
