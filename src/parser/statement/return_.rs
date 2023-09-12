use crate::{lexer::{Lexer, Token}, runner::{run::Run, scope::Scope, value::Value}, parser::expression};

use super::{AstNode, expression_statement::end};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
  expression: Option<Box<AstNode>>,
}

impl ReturnStatement {
  pub fn new(expression: Option<AstNode>) -> AstNode {
    AstNode::ReturnStatement(Self { expression: expression.map(Box::new) })
  }
}

impl Run for ReturnStatement {
  fn run(&self, scope: &mut Scope) -> Value {
    let return_value = self.expression.as_ref().map(|e| e.run(scope)).unwrap_or(Value::Undefined);
    scope.return_value(return_value);
    return Value::Undefined;
  }
}


pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  match lexer.peek()? {
    Token::Keyword(kw, _) if kw == "return" => {
      lexer.consume();
      let exp = expression::parse(lexer);
      end(lexer).expect("Expected ';' after return statement");
      return Some(ReturnStatement::new(exp));
    },
    _ => None,
  }
}
