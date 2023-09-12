use crate::{
  lexer::{Lexer, Token},
  parser::AstNode,
  runner::{run::Run, value::Value, scope::Scope},
};

#[derive(Debug, Clone)]
pub struct BinaryExpression {
  pub operator: String,
  pub left: Box<AstNode>,
  pub right: Box<AstNode>,
}

impl BinaryExpression {
  pub fn new(operator: String, left: AstNode, right: AstNode) -> AstNode {
    return AstNode::BinaryExpression(Self {
      operator,
      left: Box::new(left),
      right: Box::new(right),
    });
  }
}

impl Run for BinaryExpression {
  fn run(&self, scope: &mut Scope) -> Value {
    let left = self.left.run(scope);
    let right = self.right.run(scope);

    match self.operator.as_str() {
      "+" => match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
        (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
        (Value::String(a), Value::Number(b)) => Value::String(format!("{}{}", a, b.to_string())),
        (Value::Number(a), Value::String(b)) => Value::String(format!("{}{}", a.to_string(), b)),
        _ => panic!("Invalid operands for '+'"),
      },
      ">" => match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a > b),
        _ => panic!("Invalid operands for '>'"),
      },
      _ => panic!("Invalid operator: {}", self.operator),
    }
  }
}

pub fn parse<
  O: Fn(&str) -> bool,
  L: Fn(&mut Lexer) -> Option<AstNode>,
  R: Fn(&mut Lexer) -> Option<AstNode>,
>(
  lexer: &mut Lexer,
  operator: O,
  left_node: L,
  right_node: R,
) -> Option<AstNode> {
  let mut exp = left_node(lexer)?;

  loop {
    match lexer.peek() {
      Some(Token::Punctuator(p, _)) if operator(p.as_str()) => {
        let op = p.clone();
        lexer.consume();
        let right = right_node(lexer).expect("Expected right side of binary expression");
        exp = BinaryExpression::new(op, exp, right);
      }
      _ => break,
    }
  }

  return Some(exp);
}
