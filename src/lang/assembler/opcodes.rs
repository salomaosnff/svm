use std::fmt::Debug;

pub const NOP: u8 = 0x00;
pub const HALT: u8 = 0x01;
pub const COPY: u8 = 0x02;
pub const PUSH: u8 = 0x03;
pub const PUSHALL: u8 = 0x04;
pub const POP: u8 = 0x05;
pub const ADD: u8 = 0x06;
pub const SUB: u8 = 0x07;
pub const MUL: u8 = 0x08;
pub const DIV: u8 = 0x09;
pub const MOD: u8 = 0x0A;
pub const POW: u8 = 0x0B;
pub const INC: u8 = 0x0C;
pub const DEC: u8 = 0x0D;
pub const WRITE: u8 = 0x0E;
pub const JUMP: u8 = 0x0F;
pub const CMP: u8 = 0x10;
pub const LT: u8 = 0x11;
pub const EQ: u8 = 0x12;
pub const GT: u8 = 0x13;


#[derive(Clone, Debug)]
pub enum OpCode {
  NOP,
  HALT,
  PUSH(i32),
  PUSHALL(Vec<i32>),
  POP,
  COPY,
  ADD,
  SUB,
  MUL,
  DIV,
  MOD,
  POW,
  WRITE,
  JUMP,
  INC,
  DEC,
  LT,
  CMP,
  EQ,
  GT,
}