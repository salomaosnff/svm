use crate::lexer::Lexer;
use super::AstNode;

pub mod block;
pub mod break_;
pub mod breakable;
pub mod continue_;
pub mod declaration;
pub mod expression_statement;
pub mod if_;
pub mod labelled;
pub mod list;
pub mod return_;
pub mod throw;
pub mod try_;
pub mod variable;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return block::parse(lexer).map(|node| AstNode::BlockStatement(node))
    .or_else(|| variable::parse(lexer))
    .or_else(|| expression_statement::parse(lexer))
    .or_else(|| if_::parse(lexer))
    .or_else(|| breakable::parse(lexer))
    .or_else(|| continue_::parse(lexer))
    .or_else(|| break_::parse(lexer))
    .or_else(|| return_::parse(lexer))
    .or_else(|| labelled::parse(lexer))
    .or_else(|| throw::parse(lexer))
    .or_else(|| try_::parse(lexer));
}