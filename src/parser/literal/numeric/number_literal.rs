use crate::{
  parser::AstNode,
  runner::{self, run::Run, value::Value},
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
  fn run(&self, _: &mut runner::scope::Scope) -> Value {
    return Value::Number(self.literal.clone().parse::<f64>().unwrap());
  }
}
