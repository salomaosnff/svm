use super::value::Value;
use std::{cell::RefCell, collections::HashMap, process::exit, rc::Rc};

#[derive(Debug, Clone)]
pub enum Scope {
  Global {
    locals: HashMap<String, Value>,
  },
  Block {
    locals: HashMap<String, Value>,
    parent: Rc<RefCell<Scope>>,
  },
  Function {
    locals: HashMap<String, Value>,
    parent: Rc<RefCell<Scope>>,
    return_value: Option<Value>,
  },
}

impl Scope {
  pub fn new() -> Rc<RefCell<Self>> {
    return Rc::new(RefCell::new(Scope::Global {
      locals: HashMap::new(),
    }));
  }

  pub fn create_block(parent: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
    return Rc::new(RefCell::new(Scope::Block {
      locals: HashMap::new(),
      parent: Rc::clone(parent),
    }));
  }

  pub fn create_function(parent: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
    return Rc::new(RefCell::new(Scope::Function {
      locals: HashMap::new(),
      parent: Rc::clone(parent),
      return_value: None,
    }));
  }

  pub fn get(&self, name: &str) -> Value {
    match self {
      Scope::Block { locals, parent, .. } | Scope::Function { locals, parent, .. } => {
        if locals.contains_key(name) {
          return locals.get(name).unwrap().clone();
        } else {
          return parent.as_ref().borrow().get(name);
        }
      }
      Scope::Global { locals } => {
        if locals.contains_key(name) {
          return locals.get(name).unwrap().clone();
        } else {
          println!("Undefined variable: {}", name);
          exit(1)
        }
      }
    }
  }

  pub fn set(&mut self, name: &str, value: Value) {
    match self {
      Scope::Block { locals, .. } | Scope::Function { locals, .. } | Scope::Global { locals } => {
        locals.insert(name.to_string(), value);
      }
    }
  }

  pub fn set_return_value(&mut self, value: Value) {
    match self {
      Scope::Function { return_value, .. } => {
        *return_value = Some(value);
      }
      Scope::Block { parent, .. } => {
        parent.as_ref().borrow_mut().set_return_value(value);
      }
      Scope::Global { .. } => {
        println!("return statement outside of function");
        exit(1)
      }
    }
  }

  pub fn return_value_exists(&self) -> bool {
    match self {
      Scope::Function { return_value, .. } => return_value.is_some(),
      Scope::Block { parent, .. } => parent.as_ref().borrow().return_value_exists(),
      Scope::Global { .. } => false,
    }
  }

  pub fn get_return_value(&self) -> Value {
    match self {
      Scope::Function { return_value, .. } => return_value.clone().unwrap(),
      Scope::Block { parent, .. } => parent.as_ref().borrow().get_return_value(),
      Scope::Global { .. } => {
        println!("scope.get_return_value() called outside of function");
        exit(1)
      }
    }
  }
}
