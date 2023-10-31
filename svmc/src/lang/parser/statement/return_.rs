use crate::{
  lexer::{Lexer, Token},
  parser::expression,
};

use super::{expression_statement::end, AstNode};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
  pub expression: Option<Box<AstNode>>,
}

impl ReturnStatement {
  pub fn new(expression: Option<AstNode>) -> AstNode {
    AstNode::ReturnStatement(Self {
      expression: expression.map(Box::new),
    })
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek()? {
    Token::Keyword(kw, _) if kw == "return" => {
      lexer.consume();
      let exp = expression::parse(lexer);
      end(lexer).expect("Expected ';' after return statement");
      return Some(ReturnStatement::new(exp));
    }
    _ => None,
  }
}
