use super::{run::Run, value::Value};
use std::collections::HashMap;

pub struct Scope {
  pub declarations: HashMap<String, Value>,
  parent: Option<Box<Scope>>,
}

impl Scope {
  pub fn new() -> Self {
    return Self {
      declarations: HashMap::new(),
      parent: None,
    };
  }

  pub fn new_child(parent: Self) -> Self {
    return Self {
      declarations: HashMap::new(),
      parent: Some(Box::new(parent)),
    };
  }

  pub fn get(&self, name: &str) -> &Value {
    let mut scope = self;

    loop {
      match scope.declarations.get(name) {
        Some(value) => return value,
        None => match &scope.parent {
          Some(parent) => {
            scope = parent;
          }
          None => panic!("identifier '{}' is not defined", name),
        },
      }
    }
  }

  pub fn declare(&mut self, name: &str, value: Value) {
    self.declarations.insert(name.to_string(), value);
  }

  pub fn run<T: Run>(&mut self, node: &mut T) -> Value {
    return node.run(self);
  }
}
