use crate::lexer::{Lexer, Token};

use super::AstNode;

#[derive(Debug)]
pub struct BlockStatement {
  pub statements: Vec<AstNode>,
}

impl BlockStatement {
  pub fn new(statements: Vec<AstNode>) -> AstNode {
    return AstNode::BlockStatement(BlockStatement { statements });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.lookahead() {
    Some(Token::Punctuator(p, _)) if p == "{" => {
      lexer.consume();

      let mut statements = Vec::new();

      loop {
        let statement = super::parse(lexer);

        if statement.is_some() {
          statements.push(statement.unwrap());
        } else {
          break;
        }
      }

      lexer
        .consume_if(|t| matches!(t, Token::Punctuator(p, _) if p == "}"))
        .expect("Expected '}'");

      return Some(BlockStatement::new(statements));
    }
    _ => None,
  }
}
