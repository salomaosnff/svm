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
  statement::{
    block::BlockStatement, if_::IfStatement, return_::ReturnStatement,
    variable::VariableDeclaration,
  },
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
