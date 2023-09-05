use std::fmt::Debug;

use crate::{
  lexer::{Consumer, Token},
  parser::AstNode,
};

use super::binary_expression::{term, logical_operation};


#[derive(Debug)]
pub struct AssignExpression {
  pub operator: String,
  pub left: Box<AstNode>,
  pub right: Box<AstNode>,
}

impl AssignExpression {
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

          left = AstNode::AssignExpression(AssignExpression {
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


pub fn equality_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return AssignExpression::parse(
    lexer,
    |op| matches!(op, "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^=" | "|="),
    logical_operation,
    term,
  );
}

pub fn bitwise_and_operation(lexer: &mut Consumer<Token>) -> Option<AstNode> {
  return AssignExpression::parse(lexer, |op| op == "&", equality_operation, term);
}
