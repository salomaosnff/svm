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
  literal::{boolean::BooleanLiteral, Literal},
  statement::{block::BlockStatement, variable::VariableDeclaration},
};

#[derive(Debug)]
pub enum AstNode {
  ThisExpression,
  NullLiteral,
  BooleanLiteral(BooleanLiteral),
  IdentifierName(IdentifierName),
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
  Program(Vec<AstNode>),
}
