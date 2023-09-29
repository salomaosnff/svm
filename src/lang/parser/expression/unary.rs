use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

#[derive(Debug, Clone)]
pub struct UnaryExpression {
  pub operator: String,
  pub operand: Box<AstNode>,
}

impl UnaryExpression {
  pub fn new(operator: String, operand: AstNode) -> AstNode {
    return AstNode::UnaryExpression(Self {
      operator,
      operand: Box::new(operand),
    });
  }
}

use super::update;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut exp = update::parse(lexer)?;

  match lexer.peek() {
    Some(Token::Keyword(lexeme, _)) if matches!(lexeme.as_str(), "+" | "-" | "~" | "!") => {
      let operator = lexeme.clone();

      lexer.consume();

      let operand = update::parse(lexer).expect("Expected expression after unary operator");

      exp = UnaryExpression::new(operator, operand);
    }
    _ => {}
  }

  return Some(exp);
}
