use crate::runner::run::Run;

use super::{
  expression::{
    binary::BinaryExpression,
    conditional::ConditionalExpression,
    function::{call::CallExpression, FunctionExpression},
    left_hand_side::new::NewExpression,
    member::MemberExpression,
    unary::UnaryExpression,
    update::UpdateExpression,
  },
  identifier::name::IdentifierName,
  literal::{boolean::BooleanLiteral, numeric::NumberLiteral, string::StringLiteral, Literal},
  program::Program,
  statement::{block::BlockStatement, variable::VariableDeclaration},
};

#[derive(Debug, Clone)]
pub enum AstNode {
  ThisExpression,
  NullLiteral,
  BooleanLiteral(BooleanLiteral),
  IdentifierName(IdentifierName),
  StringLiteral(StringLiteral),
  NumberLiteral(NumberLiteral),
  Literal(Literal),
  ArrayLiteral(Vec<AstNode>),
  ObjectLiteral(Vec<(AstNode, AstNode)>),
  MemberExpression(MemberExpression),
  ConditionalExpression(ConditionalExpression),
  BinaryExpression(BinaryExpression),
  UnaryExpression(UnaryExpression),
  NewExpression(NewExpression),
  CallExpression(CallExpression),
  UpdateExpression(UpdateExpression),
  FunctionExpression(FunctionExpression),
  BlockStatement(BlockStatement),
  VariableDeclaration(VariableDeclaration),
  Program(Program),
}

impl Run for AstNode {
  fn run(&self, scope: &mut crate::runner::scope::Scope) -> crate::runner::value::Value {
    match self {
      AstNode::Program(node) => node.run(scope),
      AstNode::StringLiteral(node) => node.run(scope),
      AstNode::NumberLiteral(node) => node.run(scope),
      AstNode::BinaryExpression(node) => node.run(scope),
      AstNode::VariableDeclaration(node) => node.run(scope),
      node => panic!("Cannot run node: {:?}", node),
    }
  }
}
