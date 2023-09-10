use crate::{
  lexer::{Lexer, Token},
  parser::{
    identifier::{self, name::IdentifierName},
    operators::assignment,
  },
};

use super::AstNode;

#[derive(Debug)]
pub struct VariableDeclaration {
  pub name: String,
  pub constant: bool,
  pub initializer: Option<Box<AstNode>>,
}

impl VariableDeclaration {
  pub fn new(name: String, constant: bool, initializer: Option<AstNode>) -> AstNode {
    return AstNode::VariableDeclaration(Self {
      name,
      constant,
      initializer: initializer.map(Box::new),
    });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.lookahead() {
    Some(Token::Keyword(l, _)) if l == "let" || l == "const" => {
      let kw: String = l.clone();

      lexer.consume();

      let identifier = match identifier::parse(lexer) {
        Some(AstNode::IdentifierName(i)) => i,
        _ => panic!("Expected identifier after let/const"),
      };

      let initializer = match lexer.lookahead() {
        Some(Token::Punctuator(p, _)) if p == "=" => {
          lexer.consume();

          Some(assignment::parse(lexer).expect("Expected expression after '='"))
        }
        _ => None,
      };

      return Some(VariableDeclaration::new(
        identifier.name,
        kw == "const",
        initializer,
      ));
    }
    _ => None,
  }
}
