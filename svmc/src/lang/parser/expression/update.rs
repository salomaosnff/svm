use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

use super::{left_hand_side, unary};

#[derive(Debug, Clone)]
pub struct UpdateExpression {
  pub operator: String,
  pub prefix: bool,
  pub argument: Box<AstNode>,
}

impl UpdateExpression {
  pub fn new(operator: String, argument: AstNode, prefix: bool) -> AstNode {
    return AstNode::UpdateExpression(Self {
      operator,
      prefix,
      argument: Box::new(argument),
    });
  }

  pub fn prefix(operator: String, argument: AstNode) -> AstNode {
    return Self::new(operator, argument, true);
  }

  pub fn postfix(operator: String, argument: AstNode) -> AstNode {
    return Self::new(operator, argument, false);
  }
}

fn post_increment(lexer: &mut Lexer) -> Option<AstNode> {
  let mut left = left_hand_side::parse(lexer)?;

  match lexer.peek() {
    Some(Token::Punctuator(p, _)) if p == "++" || p == "--" => {
      let op = p.clone();
      lexer.consume();
      left = UpdateExpression::postfix(op, left);
    }
    _ => {}
  }

  return Some(left);
}

fn pre_increment(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.peek() {
    Some(Token::Punctuator(p, _)) if matches!(p.as_str(), "++" | "--") => {
      let op = p.clone();
      lexer.consume();
      return Some(UpdateExpression::prefix(op, unary::parse(lexer)?));
    }
    _ => None,
  };
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return pre_increment(lexer).or_else(|| post_increment(lexer));
}
