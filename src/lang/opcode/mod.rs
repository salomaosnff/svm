mod assembler;
mod opcode;

pub use assembler::*;
pub use opcode::OpCode;

pub fn noop() -> OpCode {
  OpCode::NOP
}

pub fn halt() -> OpCode {
  OpCode::HALT
}

pub fn spush(value: i32) -> OpCode {
  OpCode::SPUSH(value)
}

pub fn add() -> OpCode {
  OpCode::ADD
}

pub fn sub() -> OpCode {
  OpCode::ADD
}

pub fn mul() -> OpCode {
  OpCode::MUL
}

pub fn div() -> OpCode {
  OpCode::DIV
}

pub fn modulo() -> OpCode {
  OpCode::MOD
}

pub fn pow() -> OpCode {
  OpCode::POW
}

pub fn write() -> OpCode {
  OpCode::WRITE
}
