use crate::{
  lexer::{Lexer, Token},
  parser::identifier,
};

use super::AstNode;

#[derive(Debug, Clone)]
pub struct MemberExpression {
  pub property: Box<AstNode>,
  pub object: Box<AstNode>,
}

impl MemberExpression {
  pub fn new(property: AstNode, object: AstNode) -> AstNode {
    return AstNode::MemberExpression(Self {
      property: Box::new(property),
      object: Box::new(object),
    });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut object = super::primary::parse(lexer)?;

  loop {
    match lexer.peek() {
      Some(Token::Punctuator(lexeme, _)) if lexeme == "." => {
        lexer.consume();

        let property = identifier::name::parse(lexer)?;

        object = MemberExpression::new(property, object);
      }
      Some(Token::Punctuator(lexeme, _)) if lexeme == "[" => {
        lexer.consume();

        let property = super::parse(lexer)?;

        object = MemberExpression::new(property, object);
      }
      _ => break,
    }
  }

  return Some(object);
}
