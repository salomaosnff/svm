use crate::{
  lexer::Lexer,
  parser::AstNode, runner::{scope::Scope, run::Run, value::Value},
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

pub fn run(statements: &Vec<AstNode>, scope: &mut Scope) -> Value {
  for statement in statements {
    statement.run(scope);

    match statement {
      AstNode::ReturnStatement(_) => break,
      _ => (),
    }
  }

  return scope.return_value.clone()
}