use std::fmt::Debug;

use crate::{
  lexer::{Consumer, Token},
  parser::AstNode,
};

use super::{number_literal, unary_operation};

#[derive(Debug)]
pub struct BinaryOperation {
  pub operator: String,
  pub left: Box<AstNode>,
  pub right: Box<AstNode>,
}

impl BinaryOperation {
  pub fn parse<
    'a,
    O: Fn(&str) -> bool,
    L: Fn(&mut Consumer<Token>) -> Option<AstNode>,
    R: Fn(&mut Consumer<Token>) -> Option<AstNode>,
  >(
    lexer: &mut Consumer<Token>,
    get_operator: O,
    get_left: L,
    get_right: R,
  ) -> Option<AstNode> {
    let mut left = get_left(lexer)?;

    loop {
      match lexer.lookahead() {
        Some(Token::Operator(op, _)) if get_operator(op) => {
          let operator = op.clone();

          lexer.consume();

          left = AstNode::BinaryOperation(BinaryOperation {
            operator,
            left: Box::from(left),
            right: Box::from(get_right(lexer).expect("Falta expressão da direita!")),
          });
        }
        _ => break,
      }
    }

    return Some(left);
  }
}

pub fn parenthesis_expression(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  match lexer.lookahead() {
    Some(Token::Delimiter(del, _)) if del == "(" => {
      lexer.consume();
      let exp = add_operation(lexer).expect("Era esperado uma expressão");

      lexer
        .consume_if(|t| matches!(t, Token::Delimiter(del, _) if del == ")"))
        .expect("Está faltando o )");

      return Some(exp);
    }
    _ => None,
  }
}

pub fn term(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  match lexer.lookahead() {
    Some(Token::Delimiter(del, _)) if del == "(" => parenthesis_expression(lexer),
    _ => number_literal::parse(lexer),
  }
}

pub fn power_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(lexer, |op| matches!(op, "**"), term, term);
}

pub fn multiply_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(
    lexer,
    |op| matches!(op, "*" | "/" | "%"),
    power_operation,
    power_operation,
  );
}

pub fn add_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(
    lexer,
    |op| matches!(op, "+" | "-"),
    multiply_operation,
    term,
  );
}

pub fn logical_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(
    lexer,
    |op| matches!(op, "<" | "<=" | ">" | ">="),
    add_operation,
    term,
  );
}

pub fn equality_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(
    lexer,
    |op| matches!(op, "==" | "!="),
    logical_operation,
    term,
  );
}

pub fn bitwise_and_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(lexer, |op| op == "&", equality_operation, term);
}
pub fn bitwise_xor_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(lexer, |op| op == "^", bitwise_and_operation, term);
}
pub fn bitwise_or_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(lexer, |op| op == "|", bitwise_xor_operation, term);
}
pub fn and_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(lexer, |op| op == "&&", bitwise_or_operation, term);
}
pub fn or_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return BinaryOperation::parse(lexer, |op| op == "||", and_operation, term);
}
