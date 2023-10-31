use std::fmt::Display;

use crate::parser::{expression::function::FunctionExpression, literal::Literal};

#[derive(Debug, Clone)]
pub struct NativeFunction {
  function: fn(Vec<Value>) -> Value,
}

impl NativeFunction {
  pub fn new(function: fn(Vec<Value>) -> Value) -> Self {
    return Self { function };
  }

  pub fn call(&self, args: Vec<Value>) -> Value {
    return (self.function)(args);
  }
}

#[derive(Debug, Clone)]
pub enum Value {
  String(String),
  Number(f64),
  Boolean(bool),
  Null,
  Undefined,
  Function(FunctionExpression),
  NativeFunction(NativeFunction),
}

impl Value {
  pub fn from_node(literal: Literal) -> Self {
    match literal.value {
      value if value == "true" => Self::Boolean(true),
      value if value == "false" => Self::Boolean(false),
      value if value == "null" => Self::Null,
      value if value == "undefined" => Self::Undefined,
      value
        if (value.starts_with('"') && value.ends_with('"'))
          || value.starts_with('\'') && value.ends_with('\'') =>
      {
        Self::String(value[1..value.len() - 1].to_string())
      }
      value => {
        if let Ok(number) = value.parse::<f64>() {
          return Self::Number(number);
        }

        return Self::String(value);
      }
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::String(value) => write!(f, "{}", value),
      Self::Number(value) => write!(f, "{}", value),
      Self::Boolean(value) => write!(f, "{}", value),
      Self::Null => write!(f, "null"),
      Self::Undefined => write!(f, "undefined"),
      Self::Function(_) => write!(f, "[Function]"),
      Self::NativeFunction(_) => write!(f, "[NativeFunction]"),
    }
  }
}