use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

use super::assignment;

#[derive(Debug, Clone)]
pub struct ConditionalExpression {
  pub condition: Box<AstNode>,
  pub consequent: Box<AstNode>,
  pub alternate: Box<AstNode>,
}

impl ConditionalExpression {
  pub fn new(condition: AstNode, consequent: AstNode, alternate: AstNode) -> AstNode {
    return AstNode::ConditionalExpression(Self {
      condition: Box::new(condition),
      consequent: Box::new(consequent),
      alternate: Box::new(alternate),
    });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let condition = super::short_circuit::parse(lexer)?;

  match lexer.peek() {
    Some(Token::Punctuator(p, _)) if p == "?" => {
      lexer.consume();

      let consequent = assignment::parse(lexer)?;

      match lexer.peek() {
        Some(Token::Punctuator(p, _)) if p == ":" => {
          lexer.consume();

          let alternate = assignment::parse(lexer)?;

          return Some(ConditionalExpression::new(condition, consequent, alternate));
        }
        _ => {
          panic!("Expected ':' after conditional expression consequent");
        }
      }
    }
    _ => {}
  }

  return Some(condition);
}
