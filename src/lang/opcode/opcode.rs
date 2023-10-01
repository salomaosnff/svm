use std::{fmt::Debug, fs::File, io::BufRead};

use super::assembler::Assembler;

const NOOP_OPCODE: u8 = 0x00;
const HALT_OPCODE: u8 = 0x01;
const SPUSH_OPCODE: u8 = 0x02;
const SPEEK_OPCODE: u8 = 0x03;
const INC_OPCODE: u8 = 0x04;
const DEC_OPCODE: u8 = 0x05;
const ADD_OPCODE: u8 = 0x06;
const SUB_OPCODE: u8 = 0x07;
const MUL_OPCODE: u8 = 0x08;
const DIV_OPCODE: u8 = 0x09;
const MOD_OPCODE: u8 = 0x0A;
const POW_OPCODE: u8 = 0x0B;
const WRITE_OPCODE: u8 = 0x0C;
const LABEL_OPCODE: u8 = 0x0D;
const JUMP_OPCODE: u8 = 0x0E;
const LT_OPCODE: u8 = 0x0F;
const JUMPI_OPCODE: u8 = 0x10;

#[derive(Clone)]
pub enum OpCode {
  NOP,
  HALT,
  SPUSH(i32),
  SPEEK,
  ADD,
  SUB,
  MUL,
  DIV,
  MOD,
  POW,
  WRITE,
  LABEL(i32),
  JUMP,
  INC,
  DEC,
  LT,
  JUMPI,
}

impl Debug for OpCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.disassemble())
  }
}

impl Assembler for OpCode {
  fn assemble(&self) -> Vec<u8> {
    match self {
      OpCode::NOP => vec![NOOP_OPCODE],
      OpCode::HALT => vec![HALT_OPCODE],
      OpCode::SPUSH(op) => {
        let mut bytes = vec![SPUSH_OPCODE];
        bytes.extend(op.to_be_bytes().iter());
        bytes
      }
      OpCode::SPEEK => vec![SPEEK_OPCODE],
      OpCode::INC => vec![INC_OPCODE],
      OpCode::DEC => vec![DEC_OPCODE],
      OpCode::ADD => vec![ADD_OPCODE],
      OpCode::SUB => vec![SUB_OPCODE],
      OpCode::MUL => vec![MUL_OPCODE],
      OpCode::DIV => vec![DIV_OPCODE],
      OpCode::MOD => vec![MOD_OPCODE],
      OpCode::POW => vec![POW_OPCODE],
      OpCode::WRITE => vec![WRITE_OPCODE],
      OpCode::LABEL(op) => {
        let mut bytes = vec![LABEL_OPCODE];
        bytes.extend(op.to_be_bytes().iter());
        bytes
      }
      OpCode::JUMP => vec![JUMP_OPCODE],
      OpCode::LT => vec![LT_OPCODE],
      OpCode::JUMPI => vec![JUMPI_OPCODE],
    }
  }

  fn disassemble(&self) -> String {
    match self {
      OpCode::NOP => String::from("NOP"),
      OpCode::HALT => String::from("HALT"),
      OpCode::SPUSH(value) => format!("SPUSH {}", value),
      OpCode::SPEEK => String::from("SPEEK"),
      OpCode::INC => String::from("INC"),
      OpCode::DEC => String::from("DEC"),
      OpCode::ADD => String::from("ADD"),
      OpCode::SUB => String::from("SUB"),
      OpCode::MUL => String::from("MUL"),
      OpCode::DIV => String::from("DIV"),
      OpCode::MOD => String::from("MOD"),
      OpCode::POW => String::from("POW"),
      OpCode::WRITE => String::from("WRITE"),
      OpCode::LABEL(value) => format!("LABEL {}", value),
      OpCode::JUMP => String::from("JUMP"),
      OpCode::LT => String::from("LT"),
      OpCode::JUMPI => String::from("JUMPI"),
    }
  }

  fn from_bytes(bytes: &mut Vec<u8>) -> Option<Self> {
    match bytes[0] {
      NOOP_OPCODE => {
        bytes.remove(0);
        Some(OpCode::NOP)
      }
      HALT_OPCODE => {
        bytes.remove(0);
        Some(OpCode::HALT)
      }
      SPUSH_OPCODE => {
        bytes.remove(0);
        let mut buffer = [0; 4];

        for i in 0..4 {
          buffer[i] = bytes.remove(0);
        }

        Some(OpCode::SPUSH(i32::from_be_bytes(buffer)))
      }
      SPEEK_OPCODE => {
        bytes.remove(0);
        Some(OpCode::SPEEK)
      }
      INC_OPCODE => {
        bytes.remove(0);
        Some(OpCode::INC)
      }
      DEC_OPCODE => {
        bytes.remove(0);
        Some(OpCode::DEC)
      }
      ADD_OPCODE => {
        bytes.remove(0);
        Some(OpCode::ADD)
      }
      SUB_OPCODE => {
        bytes.remove(0);
        Some(OpCode::SUB)
      }
      MUL_OPCODE => {
        bytes.remove(0);
        Some(OpCode::MUL)
      }
      DIV_OPCODE => {
        bytes.remove(0);
        Some(OpCode::DIV)
      }
      MOD_OPCODE => {
        bytes.remove(0);
        Some(OpCode::MOD)
      }
      POW_OPCODE => {
        bytes.remove(0);
        Some(OpCode::POW)
      }
      WRITE_OPCODE => {
        bytes.remove(0);
        Some(OpCode::WRITE)
      }
      LABEL_OPCODE => {
        bytes.remove(0);
        let mut buffer = [0; 4];

        for i in 0..4 {
          buffer[i] = bytes.remove(0);
        }

        Some(OpCode::LABEL(i32::from_be_bytes(buffer)))
      }
      JUMP_OPCODE => {
        bytes.remove(0);
        Some(OpCode::JUMP)
      }
      LT_OPCODE => {
        bytes.remove(0);
        Some(OpCode::LT)
      }
      JUMPI_OPCODE => {
        bytes.remove(0);
        Some(OpCode::JUMPI)
      }
      _ => None,
    }
  }
}
