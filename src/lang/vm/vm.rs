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
        OpCode::PUSH(value) => {
          self.push(value);
        }
        OpCode::PUSHALL(values) => {
          for value in values {
            self.push(value);
          }
        }
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
        OpCode::LT(t) => self.lt(t),
        OpCode::CMP => self.cmp(),
        OpCode::EQ(t) => self.eq(t),
        OpCode::GT(t) => self.gt(t),
        OpCode::POP(t) => self.pop(t),
        OpCode::MSP => self.move_sp(),
        OpCode::PC => self.pc(),
        OpCode::SP => self.sp(),
      };

      self.pc += 1;
    }
  }

  fn halt(&mut self) {
    self.running = false;
  }

  fn push(&mut self, value: StackValue) {
    self.stack.push(value.to_stack_bytes());
  }

  fn inc(&mut self, t: DataType) {
    self.push(StackValue::Usize(1));
    self.add(t);
  }

  fn dec(&mut self, t: DataType) {
    self.push(StackValue::Usize(1));
    self.sub(t);
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
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
        vm_panic("InvalidType", "Cannot move stack pointer by non-integer value");
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
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

    self.stack.push(result.to_stack_bytes());
  }

  fn pop(&mut self, t: DataType) {
    self.stack.pop(&t);
  }
}
