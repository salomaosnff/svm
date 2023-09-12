use crate::runner::{run::Run, scope::Scope};

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
  statement::{block::BlockStatement, variable::VariableDeclaration, return_::ReturnStatement, if_::IfStatement},
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
  ReturnStatement(ReturnStatement),
  IfStatement(IfStatement),
  Program(Program),
}

impl Run for AstNode {
  fn run(&self, scope: &mut Scope) -> crate::runner::value::Value {
    match self {
      AstNode::Program(node) => node.run(scope),
      AstNode::StringLiteral(node) => node.run(scope),
      AstNode::NumberLiteral(node) => node.run(scope),
      AstNode::BinaryExpression(node) => node.run(scope),
      AstNode::VariableDeclaration(node) => node.run(scope),
      AstNode::IdentifierName(node) => node.run(scope),
      AstNode::BlockStatement(node) => node.run(scope),
      AstNode::FunctionExpression(node) => node.run(scope),
      AstNode::CallExpression(node) => node.run(scope),
      AstNode::ReturnStatement(node) => node.run(scope),
      AstNode::IfStatement(node) => node.run(scope),
      node => panic!("Cannot run node: {:?}", node),
    }
  }
}
