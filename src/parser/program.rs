use crate::lexer::Lexer;

use super::AstNode;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut statements = Vec::new();

  loop {
    let statement = super::statement::parse(lexer);

    if statement.is_some() {
      statements.push(statement.unwrap());
    } else {
      break;
    }
  }

  return Some(AstNode::Program(statements));
}
