use crate::{
  get_operator,
  lexer::{Lexer, Token},
  parser::{expression::assignment, identifier},
  runner::{run::Run, scope::Scope, value::Value},
};

use super::{expression_statement::end, AstNode};

#[derive(Debug, Clone)]
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

impl Run for VariableDeclaration {
  fn run(&self, scope: &mut Scope) -> Value {
    let value = match &self.initializer {
      Some(initializer) => initializer.run(scope),
      None => Value::Undefined,
    };

    scope.declare(&self.name, value);

    return Value::Undefined;
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek() {
    Some(Token::Keyword(l, _)) if l == "let" || l == "const" => {
      let kw: String = l.clone();

      lexer.consume();

      let identifier = match identifier::parse(lexer) {
        Some(AstNode::IdentifierName(i)) => i,
        _ => panic!("Expected identifier after let/const"),
      };

      let initializer = match get_operator!(lexer, "=") {
        Some(_) => Some(assignment::parse(lexer).expect("Expected expression after '='")),
        _ => None,
      };

      end(lexer).expect("Expected ';' after variable declaration");

      return Some(VariableDeclaration::new(
        identifier.name,
        kw == "const",
        initializer,
      ));
    }
    _ => None,
  }
}
