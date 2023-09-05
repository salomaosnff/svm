use std::fmt::Debug;

use crate::{
  lexer::{Consumer, Token},
  parser::AstNode,
};

use super::binary_expression::or_operation;

#[derive(Debug)]
pub struct TernaryExpression {
  pub condition: Box<AstNode>,
  pub then_value: Box<AstNode>,
  pub otherwise_value: Box<AstNode>,
}

pub fn parse<'a>(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  let mut expression = or_operation(lexer)?;

  match lexer.lookahead() {
    Some(Token::Punctuation(op, _)) if op == "?" => {
      lexer.consume();

      let then_value = or_operation(lexer).expect("Esperado uma expressão!");

      lexer
        .consume_if(|x| matches!(x, Token::Punctuation(op, _) if op == ":"))
        .expect("Esperado \":\"!");

      let otherwise_value = or_operation(lexer).expect("Esperado uma expressão!");

      expression = AstNode::TernaryExpression(TernaryExpression {
        condition: Box::new(expression),
        then_value: Box::new(then_value),
        otherwise_value: Box::new(otherwise_value),
      });
    }
    _ => {}
  }

  return Some(expression);
}
