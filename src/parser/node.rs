use super::nodes::{
  binary_operation::BinaryOperation, number_literal::NumberLiteral, unary_operation::UnaryOperation,
};

#[derive(Debug)]
pub enum AstNode {
  UnaryOperation(UnaryOperation),
  BinaryOperation(BinaryOperation),
  NumberLiteral(NumberLiteral),
}
