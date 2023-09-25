use std::{borrow::BorrowMut, cell::RefCell, process::exit, rc::Rc};

use crate::parser::{
  expression::{
    binary::BinaryExpression,
    function::{call::CallExpression, FunctionExpression},
  },
  identifier::name::IdentifierName,
  literal::{numeric::NumberLiteral, string::StringLiteral},
  program::Program,
  statement::{if_::IfStatement, list, return_::ReturnStatement},
  AstNode,
};

use super::{scope::Scope, value::Value};

pub trait Eval {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value;
}

impl Eval for Program {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    return list::run(&self.statements, scope);
  }
}

impl Eval for StringLiteral {
  fn eval(&self, _: &Rc<RefCell<Scope>>) -> Value {
    return Value::String(self.literal.clone());
  }
}

impl Eval for NumberLiteral {
  fn eval(&self, _: &Rc<RefCell<Scope>>) -> Value {
    if self.literal.starts_with("0b") {
      return Value::Number(i64::from_str_radix(&self.literal[2..], 2).unwrap() as f64);
    } else if self.literal.starts_with("0x") {
      return Value::Number(i64::from_str_radix(&self.literal[2..], 16).unwrap() as f64);
    } else if self.literal.starts_with("0o") {
      return Value::Number(i64::from_str_radix(&self.literal[2..], 8).unwrap() as f64);
    } else {
      return Value::Number(self.literal.parse::<f64>().unwrap());
    }
  }
}

impl Eval for BinaryExpression {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    let left = self.left.eval(scope);
    let right = self.right.eval(scope);

    fn to_number(value: &Value) -> f64 {
      match value {
        Value::Number(number) => *number,
        _ => {
          println!("Cannot convert {:?} to number", value);
          return 0.0;
        }
      }
    }

    match self.operator.as_str() {
      "+" => Value::Number(to_number(&left) + to_number(&right)),
      "-" => Value::Number(to_number(&left) - to_number(&right)),
      "*" => Value::Number(to_number(&left) * to_number(&right)),
      "/" => Value::Number(to_number(&left) / to_number(&right)),
      "<=" => Value::Boolean(to_number(&left) <= to_number(&right)),
      ">=" => Value::Boolean(to_number(&left) >= to_number(&right)),
      "<" => Value::Boolean(to_number(&left) < to_number(&right)),
      ">" => Value::Boolean(to_number(&left) > to_number(&right)),
      _ => {
        println!("Operator {:?} not implemented", self.operator);
        exit(1)
      }
    }
  }
}

impl Eval for FunctionExpression {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    let value = Value::Function(self.clone());

    scope.as_ref().borrow_mut().set(&self.name, value.clone());

    return value;
  }
}

impl Eval for CallExpression {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    let callee = self.callee.eval(scope);

    match callee {
      Value::Function(func) => {
        let fn_scope = Scope::create_function(&scope);

        for (index, param) in func.params.iter().enumerate() {
          let value = self.arguments[index].eval(&fn_scope);
          fn_scope.as_ref().borrow_mut().set(&param.name, value);
        }

        list::run(&func.body, &fn_scope);

        return fn_scope.as_ref().borrow().get_return_value();
      }
      _ => {
        println!("Cannot call {:?}", callee);
        return Value::Undefined;
      }
    }
  }
}

impl Eval for IdentifierName {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    return scope.as_ref().borrow().get(&self.name).clone();
  }
}

impl Eval for ReturnStatement {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    if let Some(value) = &self.expression {
      let value = value.eval(scope);
      scope.as_ref().borrow_mut().set_return_value(value);
    }

    return Value::Undefined;
  }
}

impl Eval for IfStatement {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    if let Value::Boolean(condition) = self.condition.eval(&scope) {
      if condition {
        list::run(&self.then_statement, scope);
      } else if self.else_statement.len() > 0 {
        list::run(&self.else_statement, scope);
      }
    }

    return Value::Undefined;
  }
}

impl Eval for AstNode {
  fn eval(&self, scope: &Rc<RefCell<Scope>>) -> Value {
    match self {
      AstNode::Program(node) => node.eval(scope),
      AstNode::NumberLiteral(node) => node.eval(scope),
      AstNode::StringLiteral(node) => node.eval(scope),
      AstNode::BinaryExpression(node) => node.eval(scope),
      AstNode::FunctionExpression(node) => node.eval(scope),
      AstNode::CallExpression(node) => node.eval(scope),
      AstNode::IdentifierName(node) => node.eval(scope),
      AstNode::ReturnStatement(node) => node.eval(scope),
      AstNode::IfStatement(node) => node.eval(scope),
      _ => {
        println!("Eval not implemented for {:?}", self);
        exit(1)
      }
    }
  }
}
