use std::fmt::Debug;

use crate::{
  lexer::{Consumer, Token},
  parser::AstNode,
};

use super::number_literal;

#[derive(Debug)]
pub struct UnaryOperation {
  pub operator: String,
  pub operand: Box<AstNode>,
}

impl UnaryOperation {
  pub fn parse<'a, O: Fn(&str) -> bool, P: Fn(&mut Consumer<Token>) -> Option<AstNode>>(
    lexer: &mut Consumer<Token>,
    get_operator: O,
    get_operand: P,
  ) -> Option<AstNode> {
    let mut count_operations: usize = 0;
    let mut operator = String::new();

    while let Some(Token::Operator(op, _)) = lexer.lookahead() {
      if !get_operator(op) {
        break; // Sair do loop se não for um operador válido
      }

      operator = op.clone();
      lexer.consume();

      count_operations += 1;
    }

    if count_operations <= 0 {
      return None;
    }

    let mut node = AstNode::UnaryOperation(UnaryOperation {
      operator: operator.clone(),
      operand: Box::new(get_operand(lexer).expect("Falta o operando")),
    });

    for _ in 1..count_operations {
      node = AstNode::UnaryOperation(UnaryOperation {
        operator: operator.clone(),
        operand: Box::new(node),
      })
    }

    return Some(node);
  }
}

pub fn signal_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return UnaryOperation::parse(lexer, |op| matches!(op, "+" | "-"), number_literal::parse);
}
