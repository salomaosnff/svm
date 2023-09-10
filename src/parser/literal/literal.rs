use crate::parser::AstNode;

#[derive(Debug)]
pub struct Literal {
  pub value: String,
}

impl Literal {
  pub fn new(value: String) -> AstNode {
    AstNode::Literal(Self { value })
  }
}
