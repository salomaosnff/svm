use crate::parser::AstNode;

#[derive(Debug, Clone)]
pub struct Literal {
  pub value: String,
}

impl Literal {
  pub fn new(value: String) -> AstNode {
    AstNode::Literal(Self { value })
  }
}
