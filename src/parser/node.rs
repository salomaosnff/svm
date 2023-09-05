use super::nodes::{
  binary_expression::BinaryExpression, number_literal::NumberLiteral, unary_expression::UnaryExpression, assign_expression::AssignExpression, ternary_expression::TernaryExpression,
};

#[derive(Debug)]
pub enum AstNode {
  UnaryOperation(UnaryExpression),
  BinaryOperation(BinaryExpression),
  NumberLiteral(NumberLiteral),
  TernaryExpression(TernaryExpression),
  AssignExpression(AssignExpression)
}
