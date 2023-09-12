use crate::{
  parser::AstNode,
  runner::{run::Run, value::Value, scope::Scope},
};

#[derive(Debug, Clone)]
pub struct NumberLiteral {
  pub literal: String,
}

impl NumberLiteral {
  pub fn new(literal: String) -> AstNode {
    return AstNode::NumberLiteral(Self { literal });
  }
}

impl Run for NumberLiteral {
  fn run(&self, _: &mut Scope) -> Value {
    return Value::Number(self.literal.clone().parse::<f64>().unwrap());
  }
}
