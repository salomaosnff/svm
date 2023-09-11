use super::{scope::Scope, value::Value};

pub trait Run {
  fn run(&self, scope: &mut Scope) -> Value;
}
