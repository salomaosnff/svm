use crate::parser::{expression::function::FunctionExpression, literal::Literal};

#[derive(Debug, Clone)]
pub enum Value {
  String(String),
  Number(f64),
  Boolean(bool),
  Null,
  Undefined,
  Function(FunctionExpression),
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
