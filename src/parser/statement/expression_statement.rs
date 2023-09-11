use std::process::exit;

use crate::{
  lexer::{Lexer, Token},
  parser::expression,
};

use super::AstNode;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let expression = expression::parse(lexer)?;

  match expression {
    AstNode::FunctionExpression(_) => {
      end(lexer);
    }
    _ => {
      end(lexer).expect("Expected \";\" after end of statement");
    }
  }

  return Some(expression);
}

pub fn end(lexer: &mut Lexer) -> Option<()> {
  let mut consumed = false;

  loop {
    match lexer.peek() {
      Some(Token::End(_)) => {
        lexer.consume();
        consumed = true;
      }
      Some(Token::Punctuator(p, _)) if p == ";" => {
        lexer.consume();
        consumed = true;
      }
      _ => break,
    }
  }

  if !consumed {
    return None;
  }

  return Some(());
}
