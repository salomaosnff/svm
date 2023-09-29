use std::{cell::RefCell, rc::Rc};

use crate::{
  lexer::Lexer,
  parser::AstNode,
  runner::{scope::Scope, value::Value, Eval},
};

pub fn parse(lexer: &mut Lexer) -> Vec<AstNode> {
  let mut statements = Vec::new();

  while lexer.peek().is_some() {
    let statement = super::parse(lexer);

    if statement.is_some() {
      statements.push(statement.unwrap());
    } else {
      break;
    }
  }

  return statements;
}

pub fn run(statements: &Vec<AstNode>, scope: &Rc<RefCell<Scope>>) -> Value {
  let mut return_value = Value::Undefined;

  for statement in statements {
    return_value = statement.eval(scope);

    if scope.as_ref().borrow().return_value_exists() {
      break;
    }
  }

  return return_value;
}
