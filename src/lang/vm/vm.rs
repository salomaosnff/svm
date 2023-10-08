use std::fmt::Debug;

use crate::lang::assembler::{opcodes::OpCode, DataType, StackValue};

use super::{util::vm_panic, Stack, Stderr, Stdin, Stdout};

pub trait IO {
  fn read(&mut self, buffer: &mut [u8]);
  fn write(&mut self, buffer: &[u8]);
}

impl Debug for dyn IO {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<IO>")
  }
}

#[derive(Debug)]
pub struct VM {
  pub stack: Stack,
  pub program: Vec<OpCode>,
  pub pc: usize,
  pub running: bool,
  pub io: Vec<Box<dyn IO>>,
}

impl VM {
  pub fn new() -> Self {
    Self {
      stack: Stack::new(1024),
      program: Vec::new(),
      pc: 0,
      running: false,
      io: vec![Box::new(Stdin), Box::new(Stdout), Box::new(Stderr)],
    }
  }

  pub fn add_io<T: IO + 'static>(&mut self, io: T) {
    self.io.push(Box::new(io));
  }

  pub fn run(&mut self) {
    self.running = true;

    while self.running {
      if self.pc >= self.program.len() {
        self.running = false;
        break;
      }

      let op = self.program[self.pc].clone();

      match op.clone() {
        OpCode::NOP => (),
        OpCode::HALT => self.halt(),
        OpCode::PUSH(value) => self.push(value),
        OpCode::PUSHALL(values) => self.push_all(values),
        OpCode::INC(t) => self.inc(t),
        OpCode::DEC(t) => self.dec(t),
        OpCode::ADD(t) => self.add(t),
        OpCode::SUB(t) => self.sub(t),
        OpCode::MUL(t) => self.mul(t),
        OpCode::DIV(t) => self.div(t),
        OpCode::MOD(t) => self.modulo(t),
        OpCode::POW(t) => self.pow(t),
        OpCode::WRITE => self.write(),
        OpCode::JUMP => self.jump(),
        OpCode::COPY(t) => self.copy(t),
        OpCode::CMP => self.cmp(),
        OpCode::EQ(t) => self.eq(t),
        OpCode::LT(t) => self.lt(t),
        OpCode::GT(t) => self.gt(t),
        OpCode::LTE(t) => self.lte(t),
        OpCode::GTE(t) => self.gte(t),
        OpCode::POP(t, r) => self.pop(t, r),
        OpCode::MSP => self.move_sp(),
        OpCode::PC => self.pc(),
        OpCode::SP => self.sp(),
        OpCode::AND(t) => self.and(t),
        OpCode::OR(t) => self.or(t),
        OpCode::XOR(t) => self.xor(t),
        OpCode::NOT(t) => self.not(t),
        OpCode::SHL(t) => self.shl(t),
        OpCode::SHR(t) => self.shr(t),
        OpCode::MOV(reg, bytes) => self.mov(reg, bytes),
        OpCode::REG(reg, item_type) => self.reg(reg, item_type),
      };

      self.pc += 1;
    }
  }

  fn halt(&mut self) {
    self.running = false;
  }

  fn push(&mut self, value: u8) {
    self.stack.push(vec![value]);
  }

  fn push_all(&mut self, values: Vec<u8>) {
    self.stack.push(values);
  }

  fn inc(&mut self, t: DataType) {
    let value = self.stack.pop_value(&t);

    let new_value = match value {
      StackValue::U8(value) => StackValue::U8(value + 1),
      StackValue::U16(value) => StackValue::U16(value + 1),
      StackValue::U32(value) => StackValue::U32(value + 1),
      StackValue::U64(value) => StackValue::U64(value + 1),
      StackValue::I8(value) => StackValue::I8(value + 1),
      StackValue::I16(value) => StackValue::I16(value + 1),
      StackValue::I32(value) => StackValue::I32(value + 1),
      StackValue::I64(value) => StackValue::I64(value + 1),
      StackValue::F32(value) => StackValue::F32(value + 1.0),
      StackValue::F64(value) => StackValue::F64(value + 1.0),
      StackValue::Usize(value) => StackValue::Usize(value + 1),
      _ => {
        vm_panic("InvalidType", "Cannot increment non-integer value");
        return;
      }
    };

    self.stack.push_value(new_value)
  }

  fn dec(&mut self, t: DataType) {
    let value = self.stack.pop_value(&t);

    let new_value = match value {
      StackValue::U8(value) => StackValue::U8(value - 1),
      StackValue::U16(value) => StackValue::U16(value - 1),
      StackValue::U32(value) => StackValue::U32(value - 1),
      StackValue::U64(value) => StackValue::U64(value - 1),
      StackValue::I8(value) => StackValue::I8(value - 1),
      StackValue::I16(value) => StackValue::I16(value - 1),
      StackValue::I32(value) => StackValue::I32(value - 1),
      StackValue::I64(value) => StackValue::I64(value - 1),
      StackValue::F32(value) => StackValue::F32(value - 1.0),
      StackValue::F64(value) => StackValue::F64(value - 1.0),
      StackValue::Usize(value) => StackValue::Usize(value - 1),
      _ => {
        vm_panic("InvalidType", "Cannot increment non-integer value");
        return;
      }
    };

    self.stack.push_value(new_value)
  }

  fn add(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::U8(a + b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::U16(a + b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::U32(a + b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::U64(a + b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::I8(a + b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::I16(a + b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::I32(a + b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::I64(a + b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::F32(a + b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::F64(a + b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Usize(a + b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Char((a as u8 + b as u8) as char),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot add {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn sub(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::U8(a - b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::U16(a - b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::U32(a - b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::U64(a - b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::I8(a - b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::I16(a - b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::I32(a - b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::I64(a - b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::F32(a - b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::F64(a - b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Usize(a - b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Char((a as u8 - b as u8) as char),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot subtract {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn mul(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::U8(a * b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::U16(a * b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::U32(a * b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::U64(a * b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::I8(a * b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::I16(a * b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::I32(a * b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::I64(a * b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::F32(a * b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::F64(a * b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Usize(a * b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Char((a as u8 * b as u8) as char),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot multiply {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn div(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::U8(a / b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::U16(a / b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::U32(a / b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::U64(a / b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::I8(a / b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::I16(a / b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::I32(a / b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::I64(a / b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::F32(a / b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::F64(a / b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Usize(a / b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Char((a as u8 / b as u8) as char),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot divide {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn modulo(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::U8(a % b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::U16(a % b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::U32(a % b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::U64(a % b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::I8(a % b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::I16(a % b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::I32(a % b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::I64(a % b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::F32(a % b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::F64(a % b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Usize(a % b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Char((a as u8 % b as u8) as char),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot modulo {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn pow(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::U8(a.pow(b.into())),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::U16(a.pow(b.into())),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::U32(a.pow(b)),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::U64(a.pow(b.try_into().unwrap())),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::I8(a.pow(b.try_into().unwrap())),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::I16(a.pow(b.try_into().unwrap())),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::I32(a.pow(b.try_into().unwrap())),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::I64(a.pow(b.try_into().unwrap())),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::F32(a.powf(b)),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::F64(a.powf(b)),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Usize(a.pow(b as u32)),
      (StackValue::Char(a), StackValue::Char(b)) => {
        StackValue::Char((a as u8).pow(b as u32) as char)
      }
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot pow {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn write(&mut self) {
    let descriptor = match self.stack.pop_value(&DataType::Usize) {
      StackValue::Usize(value) => value,
      _ => {
        vm_panic("InvalidType", "Cannot write to non-integer descriptor");
        return;
      }
    };

    let buffer = self.stack.pop(&DataType::Buffer);

    self.io[descriptor as usize].write(buffer.as_slice());
  }

  pub fn move_sp(&mut self) {
    let offset = match self.stack.pop_value(&DataType::Usize) {
      StackValue::Usize(value) => value,
      _ => {
        vm_panic(
          "InvalidType",
          "Cannot move stack pointer by non-integer value",
        );
        return;
      }
    };
    let new_sp = self.stack.sp + offset;

    if new_sp > self.stack.size {
      vm_panic("StackOverflow", "Stack pointer out of bounds!");
    }

    self.stack.sp = new_sp;
  }

  pub fn pc(&mut self) {
    self.stack.push_value(StackValue::Usize(self.pc))
  }

  pub fn sp(&mut self) {
    self.stack.push_value(StackValue::Usize(self.stack.sp))
  }

  pub fn jump(&mut self) {
    let address = match self.stack.pop_value(&DataType::Usize) {
      StackValue::Usize(value) => value,
      _ => {
        vm_panic("InvalidType", "Cannot jump to non-integer address");
        return;
      }
    };

    self.pc = address;
  }

  fn copy(&mut self, t: DataType) {
    let value = self.stack.peek(&t).to_vec();
    self.stack.push(value);
  }

  fn lt(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::Bool(a < b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::Bool(a < b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::Bool(a < b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::Bool(a < b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::Bool(a < b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::Bool(a < b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::Bool(a < b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::Bool(a < b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::Bool(a < b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::Bool(a < b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Bool(a < b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Bool(a < b),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot compare {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn lte(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::Bool(a <= b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::Bool(a <= b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::Bool(a <= b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::Bool(a <= b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::Bool(a <= b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::Bool(a <= b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::Bool(a <= b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::Bool(a <= b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::Bool(a <= b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::Bool(a <= b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Bool(a <= b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Bool(a <= b),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot compare {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn eq(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::Bool(a == b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::Bool(a == b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::Bool(a == b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::Bool(a == b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::Bool(a == b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::Bool(a == b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::Bool(a == b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::Bool(a == b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::Bool(a == b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::Bool(a == b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Bool(a == b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Bool(a == b),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot compare {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn gt(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::Bool(a > b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::Bool(a > b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::Bool(a > b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::Bool(a > b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::Bool(a > b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::Bool(a > b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::Bool(a > b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::Bool(a > b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::Bool(a > b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::Bool(a > b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Bool(a > b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Bool(a > b),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot compare {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn gte(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => StackValue::Bool(a >= b),
      (StackValue::U16(a), StackValue::U16(b)) => StackValue::Bool(a >= b),
      (StackValue::U32(a), StackValue::U32(b)) => StackValue::Bool(a >= b),
      (StackValue::U64(a), StackValue::U64(b)) => StackValue::Bool(a >= b),
      (StackValue::I8(a), StackValue::I8(b)) => StackValue::Bool(a >= b),
      (StackValue::I16(a), StackValue::I16(b)) => StackValue::Bool(a >= b),
      (StackValue::I32(a), StackValue::I32(b)) => StackValue::Bool(a >= b),
      (StackValue::I64(a), StackValue::I64(b)) => StackValue::Bool(a >= b),
      (StackValue::F32(a), StackValue::F32(b)) => StackValue::Bool(a >= b),
      (StackValue::F64(a), StackValue::F64(b)) => StackValue::Bool(a >= b),
      (StackValue::Usize(a), StackValue::Usize(b)) => StackValue::Bool(a >= b),
      (StackValue::Char(a), StackValue::Char(b)) => StackValue::Bool(a >= b),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot compare {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };

    self.stack.push(result.to_bytes());
  }

  fn cmp(&mut self) {
    let address = match self.stack.pop_value(&DataType::Usize) {
      StackValue::Usize(value) => value,
      _ => {
        vm_panic("InvalidType", "Cannot jump to non-integer address");
        return;
      }
    };
    let value = match self.stack.pop_value(&DataType::Bool) {
      StackValue::Bool(value) => value,
      _ => {
        vm_panic("InvalidType", "Cannot check non-boolean value");
        return;
      }
    };

    if value {
      self.pc = address;
    }
  }

  fn pop(&mut self, t: DataType, r: u8) {
    let value = self.stack.pop(&t);

    if r > 0 {
      self.stack.set_register(r, value);
    }
  }

  fn and(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    match (a, b) {
      (StackValue::Bool(a), StackValue::Bool(b)) => self.stack.push_value(StackValue::Bool(a && b)),
      (StackValue::U8(a), StackValue::U8(b)) => self.stack.push_value(StackValue::U8(a & b)),
      (StackValue::U16(a), StackValue::U16(b)) => self.stack.push_value(StackValue::U16(a & b)),
      (StackValue::U32(a), StackValue::U32(b)) => self.stack.push_value(StackValue::U32(a & b)),
      (StackValue::U64(a), StackValue::U64(b)) => self.stack.push_value(StackValue::U64(a & b)),
      (StackValue::I8(a), StackValue::I8(b)) => self.stack.push_value(StackValue::I8(a & b)),
      (StackValue::I16(a), StackValue::I16(b)) => self.stack.push_value(StackValue::I16(a & b)),
      (StackValue::I32(a), StackValue::I32(b)) => self.stack.push_value(StackValue::I32(a & b)),
      (StackValue::I64(a), StackValue::I64(b)) => self.stack.push_value(StackValue::I64(a & b)),
      (StackValue::Usize(a), StackValue::Usize(b)) => {
        self.stack.push_value(StackValue::Usize(a & b))
      }
      (StackValue::Char(a), StackValue::Char(b)) => self
        .stack
        .push_value(StackValue::Char((a as u8 & b as u8) as char)),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot and {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    };
  }

  fn or(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    match (a, b) {
      (StackValue::Bool(a), StackValue::Bool(b)) => self.stack.push_value(StackValue::Bool(a || b)),
      (StackValue::U8(a), StackValue::U8(b)) => self.stack.push_value(StackValue::U8(a | b)),
      (StackValue::U16(a), StackValue::U16(b)) => self.stack.push_value(StackValue::U16(a | b)),
      (StackValue::U32(a), StackValue::U32(b)) => self.stack.push_value(StackValue::U32(a | b)),
      (StackValue::U64(a), StackValue::U64(b)) => self.stack.push_value(StackValue::U64(a | b)),
      (StackValue::I8(a), StackValue::I8(b)) => self.stack.push_value(StackValue::I8(a | b)),
      (StackValue::I16(a), StackValue::I16(b)) => self.stack.push_value(StackValue::I16(a | b)),
      (StackValue::I32(a), StackValue::I32(b)) => self.stack.push_value(StackValue::I32(a | b)),
      (StackValue::I64(a), StackValue::I64(b)) => self.stack.push_value(StackValue::I64(a | b)),
      (StackValue::Usize(a), StackValue::Usize(b)) => {
        self.stack.push_value(StackValue::Usize(a | b))
      }
      (StackValue::Char(a), StackValue::Char(b)) => self
        .stack
        .push_value(StackValue::Char((a as u8 | b as u8) as char)),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot or {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    }
  }

  fn xor(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    match (a, b) {
      (StackValue::Bool(a), StackValue::Bool(b)) => self.stack.push_value(StackValue::Bool(a ^ b)),
      (StackValue::U8(a), StackValue::U8(b)) => self.stack.push_value(StackValue::U8(a ^ b)),
      (StackValue::U16(a), StackValue::U16(b)) => self.stack.push_value(StackValue::U16(a ^ b)),
      (StackValue::U32(a), StackValue::U32(b)) => self.stack.push_value(StackValue::U32(a ^ b)),
      (StackValue::U64(a), StackValue::U64(b)) => self.stack.push_value(StackValue::U64(a ^ b)),
      (StackValue::I8(a), StackValue::I8(b)) => self.stack.push_value(StackValue::I8(a ^ b)),
      (StackValue::I16(a), StackValue::I16(b)) => self.stack.push_value(StackValue::I16(a ^ b)),
      (StackValue::I32(a), StackValue::I32(b)) => self.stack.push_value(StackValue::I32(a ^ b)),
      (StackValue::I64(a), StackValue::I64(b)) => self.stack.push_value(StackValue::I64(a ^ b)),
      (StackValue::Usize(a), StackValue::Usize(b)) => {
        self.stack.push_value(StackValue::Usize(a ^ b))
      }
      (StackValue::Char(a), StackValue::Char(b)) => self
        .stack
        .push_value(StackValue::Char((a as u8 ^ b as u8) as char)),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot xor {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    }
  }

  fn not(&mut self, t: DataType) {
    let a = StackValue::from_stack_bytes(self.stack.pop(&t), &t);

    match a {
      StackValue::Bool(a) => self.stack.push_value(StackValue::Bool(!a)),
      StackValue::U8(a) => self.stack.push_value(StackValue::U8(!a)),
      StackValue::U16(a) => self.stack.push_value(StackValue::U16(!a)),
      StackValue::U32(a) => self.stack.push_value(StackValue::U32(!a)),
      StackValue::U64(a) => self.stack.push_value(StackValue::U64(!a)),
      StackValue::I8(a) => self.stack.push_value(StackValue::I8(!a)),
      StackValue::I16(a) => self.stack.push_value(StackValue::I16(!a)),
      StackValue::I32(a) => self.stack.push_value(StackValue::I32(!a)),
      StackValue::I64(a) => self.stack.push_value(StackValue::I64(!a)),
      StackValue::Usize(a) => self.stack.push_value(StackValue::Usize(!a)),
      a => {
        vm_panic("InvalidType", format!("Cannot not {:?}", a).as_str());
        return;
      }
    }
  }

  fn shl(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&DataType::U8), &DataType::U8);

    match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => self.stack.push_value(StackValue::U8(a << b)),
      (StackValue::U16(a), StackValue::U16(b)) => self.stack.push_value(StackValue::U16(a << b)),
      (StackValue::U32(a), StackValue::U32(b)) => self.stack.push_value(StackValue::U32(a << b)),
      (StackValue::U64(a), StackValue::U64(b)) => self.stack.push_value(StackValue::U64(a << b)),
      (StackValue::I8(a), StackValue::I8(b)) => self.stack.push_value(StackValue::I8(a << b)),
      (StackValue::I16(a), StackValue::I16(b)) => self.stack.push_value(StackValue::I16(a << b)),
      (StackValue::I32(a), StackValue::I32(b)) => self.stack.push_value(StackValue::I32(a << b)),
      (StackValue::I64(a), StackValue::I64(b)) => self.stack.push_value(StackValue::I64(a << b)),
      (StackValue::Usize(a), StackValue::Usize(b)) => {
        self.stack.push_value(StackValue::Usize(a << b))
      }
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot shl {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    }
  }

  fn shr(&mut self, t: DataType) {
    let b = StackValue::from_stack_bytes(self.stack.pop(&t), &t);
    let a = StackValue::from_stack_bytes(self.stack.pop(&DataType::U8), &DataType::U8);

    match (a, b) {
      (StackValue::U8(a), StackValue::U8(b)) => self.stack.push_value(StackValue::U8(a >> b)),
      (StackValue::U16(a), StackValue::U16(b)) => self.stack.push_value(StackValue::U16(a >> b)),
      (StackValue::U32(a), StackValue::U32(b)) => self.stack.push_value(StackValue::U32(a >> b)),
      (StackValue::U64(a), StackValue::U64(b)) => self.stack.push_value(StackValue::U64(a >> b)),
      (StackValue::I8(a), StackValue::I8(b)) => self.stack.push_value(StackValue::I8(a >> b)),
      (StackValue::I16(a), StackValue::I16(b)) => self.stack.push_value(StackValue::I16(a >> b)),
      (StackValue::I32(a), StackValue::I32(b)) => self.stack.push_value(StackValue::I32(a >> b)),
      (StackValue::I64(a), StackValue::I64(b)) => self.stack.push_value(StackValue::I64(a >> b)),
      (StackValue::Usize(a), StackValue::Usize(b)) => {
        self.stack.push_value(StackValue::Usize(a >> b))
      }
      (StackValue::Char(a), StackValue::Char(b)) => self
        .stack
        .push_value(StackValue::Char((a as u8 >> b as u8) as char)),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot shr {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    }
  }

  fn mov(&mut self, reg: u8, bytes: Vec<u8>) {
    if reg == 0 {
      vm_panic("InvalidRegister", "Cannot move to register 0");
      return;
    }

    self.stack.set_register(reg, bytes);
  }

  fn reg(&mut self, reg: u8, item_type: DataType) {
    let value = self.stack.peek_register(reg, &item_type);
    self.stack.push(value);
  }
}
