use std::{fmt::Debug, fs::File, io::BufRead};

use super::assembler::Assembler;

const NOOP_OPCODE: u8 = 0x00;
const HALT_OPCODE: u8 = 0x01;
const SPUSH_OPCODE: u8 = 0x02;
const ADD_OPCODE: u8 = 0x03;
const SUB_OPCODE: u8 = 0x04;
const MUL_OPCODE: u8 = 0x05;
const DIV_OPCODE: u8 = 0x06;
const MOD_OPCODE: u8 = 0x07;
const POW_OPCODE: u8 = 0x08;
const WRITE_OPCODE: u8 = 0x09;

#[derive(Clone)]
pub enum OpCode {
  NOP,
  HALT,
  SPUSH(i32),
  ADD,
  SUB,
  MUL,
  DIV,
  MOD,
  POW,
  WRITE,
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
      OpCode::ADD => vec![ADD_OPCODE],
      OpCode::SUB => vec![SUB_OPCODE],
      OpCode::MUL => vec![MUL_OPCODE],
      OpCode::DIV => vec![DIV_OPCODE],
      OpCode::MOD => vec![MOD_OPCODE],
      OpCode::POW => vec![POW_OPCODE],
      OpCode::WRITE => vec![WRITE_OPCODE],
      _ => panic!("Invalid opcode"),
    }
  }

  fn disassemble(&self) -> String {
    match self {
      OpCode::NOP => String::from("NOP"),
      OpCode::HALT => String::from("HALT"),
      OpCode::SPUSH(value) => format!("SPUSH {}", value),
      OpCode::ADD => String::from("ADD"),
      OpCode::SUB => String::from("SUB"),
      OpCode::MUL => String::from("MUL"),
      OpCode::DIV => String::from("DIV"),
      OpCode::MOD => String::from("MOD"),
      OpCode::POW => String::from("POW"),
      OpCode::WRITE => String::from("WRITE"),

      _ => panic!("Invalid opcode"),
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
      },
      WRITE_OPCODE => {
        bytes.remove(0);
        Some(OpCode::WRITE)
      }
      _ => None,
    }
  }
}