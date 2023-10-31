use std::fmt::Debug;

use super::{Type, Value};

const USIZE_LEN: usize = std::mem::size_of::<usize>();

pub const NOP: u8 = 0x00;
pub const HALT: u8 = 0x01;
pub const MOVE: u8 = 0x02;
pub const REGISTER: u8 = 0x03;
pub const PROGRAM_COUNTER: u8 = 0x04;
pub const STACK_POINTER: u8 = 0x05;
pub const MOVE_STACK_POINTER: u8 = 0x06;
pub const PUSH: u8 = 0x07;
pub const PUSH_ALL_U8: u8 = 0x08;
pub const PUSH_ALL_U16: u8 = 0x09;
pub const PUSH_ALL_U32: u8 = 0x10;
pub const PUSH_ALL_U64: u8 = 0x11;
pub const POP: u8 = 0x12;
pub const COPY: u8 = 0x13;
pub const INCREMENT: u8 = 0x14;
pub const DECREMENT: u8 = 0x15;
pub const ADD: u8 = 0x16;
pub const SUBTRACTION: u8 = 0x17;
pub const MULTIPLY: u8 = 0x18;
pub const DIVIDE: u8 = 0x19;
pub const MODULO: u8 = 0x1A;
pub const NEGATIVE: u8 = 0x1B;
pub const POWER: u8 = 0x1C;
pub const AND: u8 = 0x1D;
pub const OR: u8 = 0x1E;
pub const XOR: u8 = 0x1F;
pub const NOT: u8 = 0x20;
pub const SHIFT_LEFT: u8 = 0x21;
pub const SHIFT_RIGHT: u8 = 0x22;
pub const EQUALS: u8 = 0x23;
pub const NOT_EQUALS: u8 = 0x24;
pub const GREATER_THAN: u8 = 0x25;
pub const GREATER_THAN_OR_EQUAL: u8 = 0x26;
pub const LESS_THAN: u8 = 0x27;
pub const LESS_THAN_OR_EQUAL: u8 = 0x28;
pub const JUMP: u8 = 0x29;
pub const JUMP_IF_ZERO: u8 = 0x30;
pub const JUMP_IF_NOT_ZERO: u8 = 0x31;
pub const GOTO: u8 = 0x32;
pub const GOTO_IF_ZERO: u8 = 0x33;
pub const GOTO_IF_NOT_ZERO: u8 = 0x34;
pub const EXTERNAL: u8 = 0x35;
pub const CALL: u8 = 0x36;
pub const RETURN: u8 = 0x37;
pub const PUSH_BYTES_U8: u8 = 0x38;
pub const PUSH_BYTES_U16: u8 = 0x39;
pub const PUSH_BYTES_U32: u8 = 0x40;
pub const PUSH_BYTES_U64: u8 = 0x41;

#[derive(Clone, Debug)]
pub enum OpCode {
  // Misc
  NoOperation,
  Halt,
  Move(u8, Value),
  Register(u8, Type),
  ProgramCounter,
  StackPointer,

  // Stack
  MoveStackPointer(isize),
  Push(Value),
  PushBytes(Vec<u8>),
  PushAll(Vec<Value>),
  Pop(Type, Option<u8>),
  Copy(Type),

  // Arithmetic
  Increment(Type),
  Decrement(Type),
  Add(Type),
  Subtraction(Type),
  Multiply(Type),
  Divide(Type),
  Modulo(Type),
  Negative(Type),
  Power(Type),

  // Bitwise
  And(Type),
  Or(Type),
  XOr(Type),
  Not(Type),
  ShiftLeft(Type),
  ShiftRight(Type),

  // Comparison
  Equals(Type),
  NotEquals(Type),
  GreaterThan(Type),
  GreaterThanOrEqual(Type),
  LessThan(Type),
  LessThanOrEqual(Type),

  // Control Flow
  Jump(usize),
  JumpIfZero(usize),
  JumpIfNotZero(usize),
  Goto,
  GotoIfZero,
  GotoIfNotZero,

  // Extensions
  External(usize),

  // Function
  Call(usize),
  Return,
}

impl OpCode {
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      OpCode::NoOperation => vec![NOP],
      OpCode::Halt => vec![HALT],
      OpCode::Move(register, value) => vec![MOVE]
        .into_iter()
        .chain(register.to_be_bytes())
        .chain(value.data_type().to_bytes())
        .chain(value.to_bytes())
        .collect(),
      OpCode::Register(a, b) => vec![REGISTER]
        .into_iter()
        .chain(b.to_bytes())
        .chain(a.to_be_bytes())
        .collect(),
      OpCode::ProgramCounter => vec![PROGRAM_COUNTER],
      OpCode::StackPointer => vec![STACK_POINTER],
      OpCode::MoveStackPointer(a) => vec![MOVE_STACK_POINTER]
        .into_iter()
        .chain(a.to_be_bytes())
        .collect(),
      OpCode::Push(a) => vec![PUSH]
        .into_iter()
        .chain(a.data_type().to_bytes())
        .chain(a.to_bytes())
        .collect(),
      OpCode::PushBytes(a) => match a.len() {
        len if len <= u8::MAX as usize => vec![PUSH_BYTES_U8]
          .into_iter()
          .chain((len as u8).to_be_bytes())
          .chain(a.clone())
          .collect::<Vec<u8>>(),
        len if len <= u16::MAX as usize => vec![PUSH_BYTES_U16]
          .into_iter()
          .chain((len as u16).to_be_bytes())
          .chain(a.clone())
          .collect::<Vec<u8>>(),
        len if len <= u32::MAX as usize => vec![PUSH_BYTES_U32]
          .into_iter()
          .chain((len as u32).to_be_bytes())
          .chain(a.clone())
          .collect::<Vec<u8>>(),
        len if len <= u64::MAX as usize => vec![PUSH_BYTES_U64]
          .into_iter()
          .chain((len as u64).to_be_bytes())
          .chain(a.clone())
          .collect::<Vec<u8>>(),
        _ => panic!("Maximum number of values exceeded!"),
      },
      OpCode::PushAll(items) => {
        let bytes = match items.len() {
          len if len == 1 => vec![PUSH],
          len if len <= u8::MAX as usize => vec![PUSH_ALL_U8]
            .into_iter()
            .chain(items[0].data_type().to_bytes())
            .chain((len as u8).to_be_bytes())
            .collect(),
          len if len <= u16::MAX as usize => vec![PUSH_ALL_U16]
            .into_iter()
            .chain(items[0].data_type().to_bytes())
            .chain((len as u16).to_be_bytes())
            .collect(),
          len if len <= u32::MAX as usize => vec![PUSH_ALL_U32]
            .into_iter()
            .chain(items[0].data_type().to_bytes())
            .chain((len as u32).to_be_bytes())
            .collect(),
          len if len <= u64::MAX as usize => vec![PUSH_ALL_U64]
            .into_iter()
            .chain(items[0].data_type().to_bytes())
            .chain((len as u64).to_be_bytes())
            .collect(),

          _ => panic!("Maximum number of values exceeded!"),
        };

        bytes
          .into_iter()
          .chain(items.iter().flat_map(|v| v.to_bytes()))
          .collect()
      }
      OpCode::Pop(a, b) => vec![POP]
        .into_iter()
        .chain(a.to_bytes())
        .chain(b.unwrap_or(0).to_be_bytes())
        .collect(),
      OpCode::Copy(a) => vec![COPY].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Increment(a) => vec![INCREMENT].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Decrement(a) => vec![DECREMENT].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Add(a) => vec![ADD].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Subtraction(a) => vec![SUBTRACTION].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Multiply(a) => vec![MULTIPLY].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Divide(a) => vec![DIVIDE].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Modulo(a) => vec![MODULO].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Negative(a) => vec![NEGATIVE].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Power(a) => vec![POWER].into_iter().chain(a.to_bytes()).collect(),
      OpCode::And(a) => vec![AND].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Or(a) => vec![OR].into_iter().chain(a.to_bytes()).collect(),
      OpCode::XOr(a) => vec![XOR].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Not(a) => vec![NOT].into_iter().chain(a.to_bytes()).collect(),
      OpCode::ShiftLeft(a) => vec![SHIFT_LEFT].into_iter().chain(a.to_bytes()).collect(),
      OpCode::ShiftRight(a) => vec![SHIFT_RIGHT].into_iter().chain(a.to_bytes()).collect(),
      OpCode::Equals(a) => vec![EQUALS].into_iter().chain(a.to_bytes()).collect(),
      OpCode::NotEquals(a) => vec![NOT_EQUALS].into_iter().chain(a.to_bytes()).collect(),
      OpCode::GreaterThan(a) => vec![GREATER_THAN].into_iter().chain(a.to_bytes()).collect(),
      OpCode::GreaterThanOrEqual(a) => vec![GREATER_THAN_OR_EQUAL]
        .into_iter()
        .chain(a.to_bytes())
        .collect(),
      OpCode::LessThan(a) => vec![LESS_THAN].into_iter().chain(a.to_bytes()).collect(),
      OpCode::LessThanOrEqual(a) => vec![LESS_THAN_OR_EQUAL]
        .into_iter()
        .chain(a.to_bytes())
        .collect(),
      OpCode::Goto => vec![GOTO],
      OpCode::GotoIfZero => vec![GOTO_IF_ZERO],
      OpCode::GotoIfNotZero => vec![GOTO_IF_NOT_ZERO],
      OpCode::Jump(a) => vec![JUMP].into_iter().chain(a.to_be_bytes()).collect(),
      OpCode::JumpIfZero(a) => vec![JUMP_IF_ZERO]
        .into_iter()
        .chain(a.to_be_bytes())
        .collect(),
      OpCode::JumpIfNotZero(a) => vec![JUMP_IF_NOT_ZERO]
        .into_iter()
        .chain(a.to_be_bytes())
        .collect(),
      OpCode::External(a) => vec![EXTERNAL].into_iter().chain(a.to_be_bytes()).collect(),
      OpCode::Call(a) => vec![CALL].into_iter().chain(a.to_be_bytes()).collect(),
      OpCode::Return => vec![RETURN],
    }
  }

  pub fn from_bytes(bytes: &mut Vec<u8>) -> OpCode {
    let opcode = bytes.remove(0);

    match opcode {
      NOP => OpCode::NoOperation,
      HALT => OpCode::Halt,
      MOVE => {
        let reg = bytes.remove(0);
        let data_type = Type::from_u8(bytes.remove(0));

        let value = Value::from_stack_bytes(bytes.clone(), &data_type);

        bytes.splice(0..data_type.size(), vec![]);

        OpCode::Move(reg, value)
      }
      REGISTER => {
        let data_type = Type::from_u8(bytes.remove(0));
        let reg = bytes.remove(0);

        OpCode::Register(reg, data_type)
      }
      PROGRAM_COUNTER => OpCode::ProgramCounter,
      STACK_POINTER => OpCode::StackPointer,
      MOVE_STACK_POINTER => {
        let len_bytes = bytes.splice(0..USIZE_LEN, vec![]).collect::<Vec<u8>>();
        let value = isize::from_be_bytes(len_bytes.as_slice().try_into().unwrap());

        OpCode::MoveStackPointer(value)
      }
      PUSH => {
        let data_type = Type::from_u8(bytes.remove(0));
        let value = Value::from_stack_bytes(bytes.clone(), &data_type);

        bytes.splice(0..data_type.size(), vec![]);

        OpCode::Push(value)
      }
      PUSH_ALL_U8 => {
        let data_type = Type::from_u8(bytes.remove(0));
        let count = bytes.remove(0);

        let values = bytes
          .splice(0..(count as usize * data_type.size()), vec![])
          .collect::<Vec<u8>>()
          .chunks(data_type.size())
          .map(|v| Value::from_stack_bytes(v.to_vec(), &data_type))
          .collect::<Vec<Value>>();

        OpCode::PushAll(values)
      }
      PUSH_ALL_U16 => {
        let data_type = Type::from_u8(bytes.remove(0));
        let len = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let values = bytes
          .splice(0..(len * data_type.size()), vec![])
          .collect::<Vec<u8>>()
          .chunks(data_type.size())
          .map(|v| Value::from_stack_bytes(v.to_vec(), &data_type))
          .collect::<Vec<Value>>();

        OpCode::PushAll(values)
      }
      PUSH_ALL_U32 => {
        let data_type = Type::from_u8(bytes.remove(0));
        let len = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let values = bytes
          .splice(0..(len * data_type.size()), vec![])
          .collect::<Vec<u8>>()
          .chunks(data_type.size())
          .map(|v| Value::from_stack_bytes(v.to_vec(), &data_type))
          .collect::<Vec<Value>>();

        OpCode::PushAll(values)
      }
      PUSH_ALL_U64 => {
        let data_type = Type::from_u8(bytes.remove(0));
        let len = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let values = bytes
          .splice(0..(len * data_type.size()), vec![])
          .collect::<Vec<u8>>()
          .chunks(data_type.size())
          .map(|v| Value::from_stack_bytes(v.to_vec(), &data_type))
          .collect::<Vec<Value>>();

        OpCode::PushAll(values)
      }
      PUSH_BYTES_U8 => {
        let len = u8::from_be_bytes(
          bytes
            .splice(0..1, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let value = bytes.splice(0..len as usize, vec![]).collect::<Vec<u8>>();

        OpCode::PushBytes(value)
      }
      PUSH_BYTES_U16 => {
        let len = u16::from_be_bytes(
          bytes
            .splice(0..2, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let value = bytes.splice(0..len as usize, vec![]).collect::<Vec<u8>>();

        OpCode::PushBytes(value)
      }
      PUSH_BYTES_U32 => {
        let len = u32::from_be_bytes(
          bytes
            .splice(0..4, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let value = bytes.splice(0..len as usize, vec![]).collect::<Vec<u8>>();

        OpCode::PushBytes(value)
      }
      PUSH_BYTES_U64 => {
        let len = u64::from_be_bytes(
          bytes
            .splice(0..8, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        let value = bytes.splice(0..len as usize, vec![]).collect::<Vec<u8>>();

        OpCode::PushBytes(value)
      }
      POP => {
        let data_type = Type::from_u8(bytes.remove(0));
        let reg = bytes.remove(0);

        OpCode::Pop(data_type, if reg == 0 { None } else { Some(reg) })
      }
      COPY => OpCode::Copy(Type::from_u8(bytes.remove(0))),
      INCREMENT => OpCode::Increment(Type::from_u8(bytes.remove(0))),
      DECREMENT => OpCode::Decrement(Type::from_u8(bytes.remove(0))),
      ADD => OpCode::Add(Type::from_u8(bytes.remove(0))),
      SUBTRACTION => OpCode::Subtraction(Type::from_u8(bytes.remove(0))),
      MULTIPLY => OpCode::Multiply(Type::from_u8(bytes.remove(0))),
      DIVIDE => OpCode::Divide(Type::from_u8(bytes.remove(0))),
      MODULO => OpCode::Modulo(Type::from_u8(bytes.remove(0))),
      NEGATIVE => OpCode::Negative(Type::from_u8(bytes.remove(0))),
      POWER => OpCode::Power(Type::from_u8(bytes.remove(0))),
      AND => OpCode::And(Type::from_u8(bytes.remove(0))),
      OR => OpCode::Or(Type::from_u8(bytes.remove(0))),
      XOR => OpCode::XOr(Type::from_u8(bytes.remove(0))),
      NOT => OpCode::Not(Type::from_u8(bytes.remove(0))),
      SHIFT_LEFT => OpCode::ShiftLeft(Type::from_u8(bytes.remove(0))),
      SHIFT_RIGHT => OpCode::ShiftRight(Type::from_u8(bytes.remove(0))),
      EQUALS => OpCode::Equals(Type::from_u8(bytes.remove(0))),
      NOT_EQUALS => OpCode::NotEquals(Type::from_u8(bytes.remove(0))),
      GREATER_THAN => OpCode::GreaterThan(Type::from_u8(bytes.remove(0))),
      GREATER_THAN_OR_EQUAL => OpCode::GreaterThanOrEqual(Type::from_u8(bytes.remove(0))),
      LESS_THAN => OpCode::LessThan(Type::from_u8(bytes.remove(0))),
      LESS_THAN_OR_EQUAL => OpCode::LessThanOrEqual(Type::from_u8(bytes.remove(0))),
      GOTO => OpCode::Goto,
      GOTO_IF_ZERO => OpCode::GotoIfZero,
      GOTO_IF_NOT_ZERO => OpCode::GotoIfNotZero,
      JUMP => {
        let address = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        OpCode::Jump(address)
      }
      JUMP_IF_ZERO => {
        let address = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        OpCode::JumpIfZero(address)
      }
      JUMP_IF_NOT_ZERO => {
        let address = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        OpCode::JumpIfNotZero(address)
      }
      EXTERNAL => {
        let address = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        OpCode::External(address)
      }
      CALL => {
        let address = usize::from_be_bytes(
          bytes
            .splice(0..USIZE_LEN, vec![])
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap(),
        );

        OpCode::Call(address)
      }
      RETURN => OpCode::Return,
      byte => panic!("Unknown opcode: [0x{:x}]", byte),
    }
  }
}
