use std::process::exit;

use crate::{lexer::Lexer, runner::{run::Run, scope::Scope}};

use super::AstNode;

#[derive(Debug, Clone)]
pub struct Program {
  pub statements: Vec<AstNode>,
}

impl Program {
  pub fn new(statements: Vec<AstNode>) -> AstNode {
    return AstNode::Program(Self { statements });
  }
}

impl Run for Program {
  fn run(&self, scope: &mut Scope) -> crate::runner::value::Value {
    let mut result = crate::runner::value::Value::Undefined;

    for statement in &self.statements {
      result = statement.run(scope);
    }

    return result;
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut statements = Vec::new();

  while lexer.peek().is_some() {
    let statement = super::statement::parse(lexer);

    if statement.is_some() {
      statements.push(statement.unwrap());
    } else {
      println!("Unexpected token: {:?}", lexer.peek()?);
      exit(1)
    }
  }

  return Some(Program::new(statements));
}
