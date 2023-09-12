use crate::parser::AstNode;

use self::scope::Scope;

pub mod run;
pub mod scope;
pub mod value;

pub fn execute(program: &AstNode) -> value::Value {
  return run::Run::run(program, &mut Scope::new());
}
