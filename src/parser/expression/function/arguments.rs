use crate::{
  lexer::{Lexer, Token},
  parser::{operators::assignment, AstNode},
};

pub fn parse(lexer: &mut Lexer) -> Vec<AstNode> {
  let mut arguments: Vec<AstNode> = Vec::new();

  match lexer.lookahead() {
    Some(Token::Punctuator(p, _)) if p.to_string() == "(" => loop {
      lexer.consume();

      match lexer.lookahead() {
        Some(Token::Punctuator(p, _)) if p.to_string() == ")" => {
          lexer.consume();
          break;
        }
        Some(Token::Punctuator(p, _)) if p.to_string() == "," => {
          lexer.consume();
        }
        _ => match assignment::parse(lexer) {
          Some(argument) => arguments.push(argument),
          None => break,
        },
      }
    },
    _ => {}
  }

  return arguments;
}
