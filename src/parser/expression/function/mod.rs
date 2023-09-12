use crate::{
  lexer::{Lexer, Token},
  parser::{identifier, statement::{block, variable::VariableDeclaration}, AstNode},
  runner::{run::Run, scope::Scope, value::Value},
};

pub mod arguments;
pub mod call;

#[derive(Debug, Clone)]
pub struct FunctionExpression {
  pub name: String,
  pub params: Vec<VariableDeclaration>,
  pub body: Vec<AstNode>,
}

impl FunctionExpression {
  pub fn new(name: String, params: Vec<VariableDeclaration>, body: Vec<AstNode>) -> AstNode {
    return AstNode::FunctionExpression(Self {
      name,
      params,
      body,
    });
  }
}

impl Run for FunctionExpression {
  fn run(&self, mut scope: &mut Scope) -> Value {
    let value = Value::Function(self.clone());

    scope.declare(self.name.as_str(), value.clone());

    return value;
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek() {
    Some(Token::Keyword(kw, _)) if kw == "function" => {
      lexer.consume();

      let identifier = match identifier::name::parse(lexer) {
        Some(AstNode::IdentifierName(identifier)) => identifier,
        _ => panic!("Expected identifier"),
      };

      lexer
        .consume_if(|t| matches!(t, Token::Punctuator(p, _) if p == "("))
        .expect("Expected '(' after function name");

      let mut parameters:Vec<VariableDeclaration> = Vec::new();

      loop {
        match lexer.peek() {
          Some(Token::Punctuator(p, _)) if p == ")" => {
            lexer.consume();
            break;
          }

          Some(Token::Punctuator(p, _)) if p == "," => {
            lexer.consume();
          }

          _ => {
            if let AstNode::IdentifierName(param) = identifier::name::parse(lexer).expect("Expected identifier") {
              parameters.push(
                VariableDeclaration {
                  name: param.name,
                  initializer: None,
                  constant: false,
                }
              );
            } else {
              panic!("Expected identifier");
            }
          }
        }
      }

      let body = match block::parse(lexer) {
        Some(block) => block,
        _ => panic!("Expected function body"),
      };

      return Some(FunctionExpression::new(
        identifier.name,
        parameters,
        body.statements,
      ));
    }
    _ => None,
  }
}
