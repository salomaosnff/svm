use crate::lexer::Lexer;

use super::AstNode;

pub mod block;
pub mod break_;
pub mod breakable;
pub mod continue_;
pub mod declaration;
pub mod expression;
pub mod if_;
pub mod labelled;
pub mod return_;
pub mod throw;
pub mod try_;
pub mod variable;

pub fn parse(lexer: &mut Lexer) -> Option<AstNode> {
  return block::parse(lexer)
    .or(variable::parse(lexer))
    .or(expression::parse(lexer))
    .or(if_::parse(lexer))
    .or(breakable::parse(lexer))
    .or(continue_::parse(lexer))
    .or(break_::parse(lexer))
    .or(return_::parse(lexer))
    .or(labelled::parse(lexer))
    .or(throw::parse(lexer))
    .or(try_::parse(lexer));
}
