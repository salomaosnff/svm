use crate::parser::AstNode;

#[derive(Debug, Clone)]
pub struct NumberLiteral {
  pub literal: String,
}

impl NumberLiteral {
  pub fn new(literal: String) -> AstNode {
    return AstNode::NumberLiteral(Self { literal });
  }
}
