use crate::{
  lexer::{Lexer, Token},
  parser::{expression::member, AstNode, statement::{self, list}}, runner::{run::Run, scope::{Scope, ScopeType}, value::Value},
};

use super::arguments;

#[derive(Debug, Clone)]
pub struct CallExpression {
  pub callee: Box<AstNode>,
  pub arguments: Vec<AstNode>,
}

impl CallExpression {
  pub fn new(callee: AstNode, arguments: Vec<AstNode>) -> AstNode {
    return AstNode::CallExpression(Self {
      callee: Box::new(callee),
      arguments,
    });
  }
}

impl Run for CallExpression {
  fn run<'a>(&self, scope: &mut Scope) -> crate::runner::value::Value {
    let mut fn_scope = scope.fork(ScopeType::Function);

    if let Value::Function(function) = self.callee.run(&mut fn_scope) {
      let args: Vec<Value> = self.arguments.iter().map(|arg| arg.run(&mut fn_scope)).collect();
      let result;
      
      fn_scope.set_args(args, function.params.clone());

      list::run(&function.body, &mut fn_scope);

      println!("Result: {:#?}", fn_scope);
      result = fn_scope.return_value;

      return result;
    }

    print!("Expected function, got {:?}", self.callee);
    std::process::exit(1);
  }
}

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  let mut exp = member::parse(lexer)?;

  loop {
    match lexer.peek() {
      Some(Token::Punctuator(p, _)) if p.to_string() == "(" => {
        exp = CallExpression::new(exp, arguments::parse(lexer));
      }
      _ => break,
    }
  }

  return Some(exp);
}
