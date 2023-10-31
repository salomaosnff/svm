use crate::{
  lexer::{Lexer, Token},
  parser::expression,
};

use super::{block, AstNode};

#[derive(Debug, Clone)]
pub struct IfStatement {
  pub condition: Box<AstNode>,
  pub then_statement: Vec<AstNode>,
  pub else_statement: Vec<AstNode>,
}

impl IfStatement {
  pub fn new(
    condition: AstNode,
    then_statement: Vec<AstNode>,
    else_statement: Vec<AstNode>,
  ) -> AstNode {
    AstNode::IfStatement(Self {
      condition: Box::new(condition),
      then_statement,
      else_statement,
    })
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  if let Token::Keyword(keyword, _) = lexer.peek()? {
    if keyword == "if" {
      lexer.consume()?;

      let condition = expression::parse(lexer).expect("Expected expression after 'if'");
      let then_statement = block::parse(lexer)
        .map(|block| block.statements)
        .expect("Expected block after 'if' condition");
      let else_statement = match lexer.peek()? {
        Token::IdentifierName(keyword, _) if keyword == "else" => {
          lexer.consume()?;
          block::parse(lexer)
            .map(|s| s.statements)
            .expect("Expected block after 'else'")
        }
        _ => Vec::new(),
      };

      return Some(IfStatement::new(condition, then_statement, else_statement));
    }
  }

  None
}
