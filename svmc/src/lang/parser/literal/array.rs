use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.peek() {
    Some(Token::Punctuator(lexeme, _)) if lexeme == "[" => {
      lexer.consume();

      let elements = Vec::new();

      loop {
        match lexer.peek() {
          Some(Token::Punctuator(lexeme, _)) if lexeme == "]" => {
            lexer.consume();
            return Some(AstNode::ArrayLiteral(elements));
          }
          Some(Token::Punctuator(lexeme, _)) if lexeme == "," => {
            lexer.consume();
            continue;
          }
          _ => {}
        }
      }
    }
    _ => None,
  };
}
