use crate::parser::statement::variable::VariableDeclaration;

use super::value::Value;
use std::{collections::HashMap, process::exit, rc::{Rc, Weak}, cell::RefCell};

#[derive(Debug, Clone)]
pub enum ScopeType {
  Block,
  Function,
}

#[derive(Debug, Clone)]
pub struct Scope {
  pub scope_type: ScopeType,
  pub declarations: HashMap<String, Value>,
  pub parent: Weak<Scope>,
  pub return_value: Value,
}

impl Scope {
  pub fn new() -> Self {
    return Self {
      scope_type: ScopeType::Block,
      declarations: HashMap::new(),
      parent: Weak::new(),
      return_value: Value::Undefined,
    };
  }

  pub fn fork(&mut self, scope_type: ScopeType) -> Self {
    return Self {
      scope_type,
      declarations: HashMap::new(),
      parent: Rc::downgrade(&Rc::new(self.clone())),
      return_value: Value::Undefined,
    };
  }

  pub fn destroy(&mut self) {
    self.declarations.clear();
  }

  pub fn block<F>(&mut self, block_fn: F)
  where
    F: FnOnce(&mut Self),
  {
    let mut block_scope = self.fork(ScopeType::Block);
    block_fn(&mut block_scope);
    block_scope.destroy();
  }

  pub fn get(&self, name: &str) -> &Value {
    let mut scope = self;

    loop {
      match scope.declarations.get(name) {
        Some(value) => return value,
        None => {
          if let Some(parent) = scope.parent.upgrade() {
            scope = parent.as_ref();
          } else {
            println!("Undefined variable: {}", name);
            exit(1);
          }
        }
      }
    }
  }

  pub fn declare(&mut self, name: &str, value: Value) {
    self.declarations.insert(name.to_string(), value);
  }

  pub fn set_args(&mut self, args: Vec<Value>, params: Vec<VariableDeclaration>) {
    for (i, arg) in args.into_iter().enumerate() {
      if let Some(param) = params.get(i) {
        self.declare(&param.name, arg);
      }
    }
  }

  pub fn return_value(&mut self, value: Value) {
    let mut scope = self;

    loop {
      match scope.scope_type {
        ScopeType::Function => {
          scope.return_value = value;
          return;
        }
        _ => {
          if let Some(parent) = scope.parent.upgrade() {
            scope = parent.as_ref()
          } else {
            println!("Unexpected return statement");
            exit(1);
          }
        }
      }
    }
  }
}
