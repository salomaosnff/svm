use crate::{
  lexer::Lexer,
  parser::{expression::assignment, AstNode},
};

pub fn parse(lexer: &mut Lexer) -> Vec<AstNode> {
  let mut statements = Vec::new();

  while let Some(statement) = super::parse(lexer) {
    statements.push(statement);
    println!("statement: {:?}", lexer.peek());
  }

  return statements;
}
