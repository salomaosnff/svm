use crate::{
  lexer::{Lexer, Token},
  parser::{identifier, statement::block, AstNode},
};

pub mod arguments;
pub mod call;

#[derive(Debug)]
pub struct FunctionExpression {
  pub name: String,
  pub arguments: Vec<AstNode>,
  pub body: Vec<AstNode>,
}

impl FunctionExpression {
  pub fn new(name: String, arguments: Vec<AstNode>, body: Vec<AstNode>) -> AstNode {
    return AstNode::FunctionExpression(Self {
      name,
      arguments,
      body,
    });
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.lookahead() {
    Some(Token::Keyword(kw, _)) if kw == "function" => {
      lexer.consume();

      let identifier = match identifier::name::parse(lexer) {
        Some(AstNode::IdentifierName(identifier)) => identifier,
        _ => panic!("Expected identifier"),
      };

      lexer
        .consume_if(|t| matches!(t, Token::Punctuator(p, _) if p == "("))
        .expect("Expected '(' after function name");

      let mut arguments = Vec::new();

      loop {
        match lexer.lookahead() {
          Some(Token::Punctuator(p, _)) if p == ")" => {
            lexer.consume();
            break;
          }

          Some(Token::Punctuator(p, _)) if p == "," => {
            lexer.consume();
          }

          _ => {
            arguments.push(identifier::name::parse(lexer).expect("Expected identifier"));
          }
        }
      }

      let body = match block::parse(lexer) {
        Some(AstNode::BlockStatement(block)) => block,
        _ => panic!("Expected function body"),
      };

      return Some(FunctionExpression::new(
        identifier.name,
        arguments,
        body.statements,
      ));
    }
    _ => None,
  }
}
