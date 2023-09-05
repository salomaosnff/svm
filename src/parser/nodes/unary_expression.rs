use std::fmt::Debug;

use crate::{
  lexer::{Consumer, Token},
  parser::AstNode,
};

use super::number_literal;

#[derive(Debug)]
pub struct UnaryExpression {
  pub operator: String,
  pub operand: Box<AstNode>,
}

impl UnaryExpression {
  pub fn parse<'a, O: Fn(&str) -> bool, P: Fn(&mut Consumer<Token>) -> Option<AstNode>>(
    lexer: &mut Consumer<Token>,
    get_operator: O,
    get_operand: P,
  ) -> Option<AstNode> {
    let mut count_operations: usize = 0;
    let mut operator = String::new();
    let mut operators = Vec::new();

    while let Some(Token::Operator(op, _)) = lexer.lookahead() {
      if !get_operator(op) {
        break;
      }

      operators.insert(0, op.clone());

      lexer.consume();

      count_operations += 1;
    }
    
    if count_operations <= 0 {
      return None;
    }
    
    let mut node = AstNode::UnaryOperation(UnaryExpression {
      operator: operators.remove(0),
      operand: Box::new(get_operand(lexer).expect("Falta o operando")),
    });

    for operator in operators {
      node = AstNode::UnaryOperation(UnaryExpression {
        operator,
        operand: Box::new(node),
      })
    }

    operator.clear();

    return Some(node);
  }
}

pub fn signal_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return UnaryExpression::parse(
    lexer,
    |op| matches!(op, "+" | "-"), 
    number_literal::parse
  );
}
