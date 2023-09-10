use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
};

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return match lexer.lookahead() {
    Some(Token::Punctuator(lexeme, _)) if lexeme == "{" => {
      lexer.consume();

      let entries = Vec::new();

      loop {
        match lexer.lookahead() {
          Some(Token::Punctuator(lexeme, _)) if lexeme == "}" => {
            lexer.consume();
            return Some(AstNode::ObjectLiteral(entries));
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
