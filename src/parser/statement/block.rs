use crate::lexer::{Lexer, Token};

use super::{expression_statement::end, AstNode};

#[derive(Debug, Clone)]
pub struct BlockStatement {
  pub statements: Vec<AstNode>,
}

impl BlockStatement {
  pub fn new(statements: Vec<AstNode>) -> BlockStatement {
    return BlockStatement { statements };
  }

  pub fn node(statements: Vec<AstNode>) -> AstNode {
    return AstNode::BlockStatement(Self::new(statements));
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<BlockStatement> {
  end(lexer);

  match lexer.peek() {
    Some(Token::Punctuator(p, _)) if p == "{" => {
      lexer.consume();

      let statements = super::list::parse(lexer);

      lexer
        .consume_if(|t| matches!(t, Token::Punctuator(p, _) if p == "}"))
        .expect("Expected '}' after block");

      end(lexer);

      return Some(BlockStatement::new(statements));
    }
    _ => None,
  }
}
