use std::process::exit;

use crate::lexer::Lexer;

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
