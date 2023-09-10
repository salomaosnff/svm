use crate::lexer::Lexer;

use super::{operators, AstNode};

pub mod arithmetic;
pub mod binary;
pub mod bitwise;
pub mod coalesce;
pub mod conditional;
pub mod equality;
pub mod function;
pub mod left_hand_side;
pub mod logical;
pub mod member;
pub mod parenthesis;
pub mod primary;
pub mod relational;
pub mod short_circuit;
pub mod unary;
pub mod update;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  // TODO: (expression, expression*)
  return operators::assignment::parse(lexer);
}
