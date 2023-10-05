use std::fmt::Debug;

use super::DataType;

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
pub const LTE: u8 = 0x12;
pub const EQ: u8 = 0x13;
pub const NEQ: u8 = 0x14;
pub const GT: u8 = 0x15;
pub const GTE: u8 = 0x16;
pub const MSP: u8 = 0x17;
pub const SP: u8 = 0x18;
pub const PC: u8 = 0x19;

#[derive(Clone, Debug)]
pub enum OpCode {
  NOP,
  HALT,
  PUSH(u8),
  PUSHALL(Vec<u8>),
  POP(DataType),
  COPY(DataType),
  ADD(DataType),
  SUB(DataType),
  MUL(DataType),
  DIV(DataType),
  MOD(DataType),
  POW(DataType),
  WRITE,
  JUMP,
  INC(DataType),
  DEC(DataType),
  CMP,
  EQ(DataType),
  LT(DataType),
  GT(DataType),
  LTE(DataType),
  GTE(DataType),
  MSP,
  SP,
  PC,
}
