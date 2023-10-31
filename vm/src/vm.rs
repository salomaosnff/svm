use std::fmt::Debug;

use svm_lang::{opcodes::OpCode, DataType, Program, Value};

use crate::stdio::{Stderr, Stdin, Stdout};

use super::{util::vm_panic, Stack};

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
  pub program: Program,
  pub pc: usize,
  pub running: bool,
  pub io: Vec<Box<dyn IO>>,
}

impl VM {
  pub fn new() -> Self {
    Self {
      stack: Stack::new(1024),
      program: Program::empty(),
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
      if self.pc >= self.program.opcodes.len() {
        self.running = false;
        break;
      }

      let op = self.program.opcodes[self.pc].clone();

      match op.clone() {
        OpCode::NoOperation => (),
        OpCode::Halt => self.halt(),
        OpCode::Push(value) => self.push(value),
        OpCode::PushAll(values) => self.push_all(values),
        OpCode::Increment(t) => self.inc(t),
        OpCode::Decrement(t) => self.dec(t),
        OpCode::Add(t) => self.add(t),
        OpCode::Subtraction(t) => self.sub(t),
        OpCode::Multiply(t) => self.mul(t),
        OpCode::Divide(t) => self.div(t),
        OpCode::Modulo(t) => self.modulo(t),
        OpCode::Power(t) => self.pow(t),
        // OpCode::Jump() => self.jump(),
        OpCode::Copy(t) => self.copy(t),
        OpCode::Equals(t) => self.eq(t),
        OpCode::LessThan(t) => self.lt(t),
        OpCode::GreaterThan(t) => self.gt(t),
        OpCode::LessThanOrEqual(t) => self.lte(t),
        OpCode::GreaterThanOrEqual(t) => self.gte(t),
        OpCode::Pop(t, r) => self.pop(t, r),
        OpCode::ProgramCounter => self.pc(),
        OpCode::StackPointer => self.sp(),
        OpCode::And(t) => self.and(t),
        OpCode::Or(t) => self.or(t),
        OpCode::XOr(t) => self.xor(t),
        OpCode::Not(t) => self.not(t),
        OpCode::ShiftLeft(t) => self.shl(t),
        OpCode::ShiftRight(t) => self.shr(t),
        OpCode::Move(reg, value) => self.mov(reg, value),
        OpCode::Register(reg, item_type) => self.reg(reg, item_type),
        _ => {
          vm_panic("InvalidOpCode", "Invalid opcode");
          return;
        }
      };

      self.pc += 1;
    }
  }

  fn halt(&mut self) {
    self.running = false;
  }

  fn push(&mut self, value: Value) {
    self.stack.push(value.to_bytes());
  }

  fn push_all(&mut self, values: Vec<Value>) {
    self.stack.push(
      values
        .iter()
        .map(|x| x.to_bytes())
        .flatten()
        .collect::<Vec<u8>>(),
    );
  }

  fn inc(&mut self, t: DataType) {
    let value = self.stack.pop_value(&t);

    let new_value = match value {
      Value::U8(value) => Value::U8(value + 1),
      Value::U16(value) => Value::U16(value + 1),
      Value::U32(value) => Value::U32(value + 1),
      Value::U64(value) => Value::U64(value + 1),
      Value::I8(value) => Value::I8(value + 1),
      Value::I16(value) => Value::I16(value + 1),
      Value::I32(value) => Value::I32(value + 1),
      Value::I64(value) => Value::I64(value + 1),
      Value::F32(value) => Value::F32(value + 1.0),
      Value::F64(value) => Value::F64(value + 1.0),
      Value::Usize(value) => Value::Usize(value + 1),
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
      Value::U8(value) => Value::U8(value - 1),
      Value::U16(value) => Value::U16(value - 1),
      Value::U32(value) => Value::U32(value - 1),
      Value::U64(value) => Value::U64(value - 1),
      Value::I8(value) => Value::I8(value - 1),
      Value::I16(value) => Value::I16(value - 1),
      Value::I32(value) => Value::I32(value - 1),
      Value::I64(value) => Value::I64(value - 1),
      Value::F32(value) => Value::F32(value - 1.0),
      Value::F64(value) => Value::F64(value - 1.0),
      Value::Usize(value) => Value::Usize(value - 1),
      _ => {
        vm_panic("InvalidType", "Cannot increment non-integer value");
        return;
      }
    };

    self.stack.push_value(new_value)
  }

  fn add(&mut self, item_type: DataType) {
    let b = Value::from_stack_bytes(self.stack.pop(&item_type), &item_type);
    let a = Value::from_stack_bytes(self.stack.pop(&item_type), &item_type);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::U8(a + b),
      (Value::U16(a), Value::U16(b)) => Value::U16(a + b),
      (Value::U32(a), Value::U32(b)) => Value::U32(a + b),
      (Value::U64(a), Value::U64(b)) => Value::U64(a + b),
      (Value::I8(a), Value::I8(b)) => Value::I8(a + b),
      (Value::I16(a), Value::I16(b)) => Value::I16(a + b),
      (Value::I32(a), Value::I32(b)) => Value::I32(a + b),
      (Value::I64(a), Value::I64(b)) => Value::I64(a + b),
      (Value::F32(a), Value::F32(b)) => Value::F32(a + b),
      (Value::F64(a), Value::F64(b)) => Value::F64(a + b),
      (Value::Usize(a), Value::Usize(b)) => Value::Usize(a + b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::U8(a - b),
      (Value::U16(a), Value::U16(b)) => Value::U16(a - b),
      (Value::U32(a), Value::U32(b)) => Value::U32(a - b),
      (Value::U64(a), Value::U64(b)) => Value::U64(a - b),
      (Value::I8(a), Value::I8(b)) => Value::I8(a - b),
      (Value::I16(a), Value::I16(b)) => Value::I16(a - b),
      (Value::I32(a), Value::I32(b)) => Value::I32(a - b),
      (Value::I64(a), Value::I64(b)) => Value::I64(a - b),
      (Value::F32(a), Value::F32(b)) => Value::F32(a - b),
      (Value::F64(a), Value::F64(b)) => Value::F64(a - b),
      (Value::Usize(a), Value::Usize(b)) => Value::Usize(a - b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::U8(a * b),
      (Value::U16(a), Value::U16(b)) => Value::U16(a * b),
      (Value::U32(a), Value::U32(b)) => Value::U32(a * b),
      (Value::U64(a), Value::U64(b)) => Value::U64(a * b),
      (Value::I8(a), Value::I8(b)) => Value::I8(a * b),
      (Value::I16(a), Value::I16(b)) => Value::I16(a * b),
      (Value::I32(a), Value::I32(b)) => Value::I32(a * b),
      (Value::I64(a), Value::I64(b)) => Value::I64(a * b),
      (Value::F32(a), Value::F32(b)) => Value::F32(a * b),
      (Value::F64(a), Value::F64(b)) => Value::F64(a * b),
      (Value::Usize(a), Value::Usize(b)) => Value::Usize(a * b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::U8(a / b),
      (Value::U16(a), Value::U16(b)) => Value::U16(a / b),
      (Value::U32(a), Value::U32(b)) => Value::U32(a / b),
      (Value::U64(a), Value::U64(b)) => Value::U64(a / b),
      (Value::I8(a), Value::I8(b)) => Value::I8(a / b),
      (Value::I16(a), Value::I16(b)) => Value::I16(a / b),
      (Value::I32(a), Value::I32(b)) => Value::I32(a / b),
      (Value::I64(a), Value::I64(b)) => Value::I64(a / b),
      (Value::F32(a), Value::F32(b)) => Value::F32(a / b),
      (Value::F64(a), Value::F64(b)) => Value::F64(a / b),
      (Value::Usize(a), Value::Usize(b)) => Value::Usize(a / b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::U8(a % b),
      (Value::U16(a), Value::U16(b)) => Value::U16(a % b),
      (Value::U32(a), Value::U32(b)) => Value::U32(a % b),
      (Value::U64(a), Value::U64(b)) => Value::U64(a % b),
      (Value::I8(a), Value::I8(b)) => Value::I8(a % b),
      (Value::I16(a), Value::I16(b)) => Value::I16(a % b),
      (Value::I32(a), Value::I32(b)) => Value::I32(a % b),
      (Value::I64(a), Value::I64(b)) => Value::I64(a % b),
      (Value::F32(a), Value::F32(b)) => Value::F32(a % b),
      (Value::F64(a), Value::F64(b)) => Value::F64(a % b),
      (Value::Usize(a), Value::Usize(b)) => Value::Usize(a % b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::U8(a.pow(b.into())),
      (Value::U16(a), Value::U16(b)) => Value::U16(a.pow(b.into())),
      (Value::U32(a), Value::U32(b)) => Value::U32(a.pow(b)),
      (Value::U64(a), Value::U64(b)) => Value::U64(a.pow(b.try_into().unwrap())),
      (Value::I8(a), Value::I8(b)) => Value::I8(a.pow(b.try_into().unwrap())),
      (Value::I16(a), Value::I16(b)) => Value::I16(a.pow(b.try_into().unwrap())),
      (Value::I32(a), Value::I32(b)) => Value::I32(a.pow(b.try_into().unwrap())),
      (Value::I64(a), Value::I64(b)) => Value::I64(a.pow(b.try_into().unwrap())),
      (Value::F32(a), Value::F32(b)) => Value::F32(a.powf(b)),
      (Value::F64(a), Value::F64(b)) => Value::F64(a.powf(b)),
      (Value::Usize(a), Value::Usize(b)) => Value::Usize(a.pow(b as u32)),
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

  pub fn move_sp(&mut self) {
    let offset = match self.stack.pop_value(&DataType::Usize) {
      Value::Usize(value) => value,
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
    self.stack.push_value(Value::Usize(self.pc))
  }

  pub fn sp(&mut self) {
    self.stack.push_value(Value::Usize(self.stack.sp))
  }

  pub fn jump(&mut self) {
    let address = match self.stack.pop_value(&DataType::Usize) {
      Value::Usize(value) => value,
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::Bool(a < b),
      (Value::U16(a), Value::U16(b)) => Value::Bool(a < b),
      (Value::U32(a), Value::U32(b)) => Value::Bool(a < b),
      (Value::U64(a), Value::U64(b)) => Value::Bool(a < b),
      (Value::I8(a), Value::I8(b)) => Value::Bool(a < b),
      (Value::I16(a), Value::I16(b)) => Value::Bool(a < b),
      (Value::I32(a), Value::I32(b)) => Value::Bool(a < b),
      (Value::I64(a), Value::I64(b)) => Value::Bool(a < b),
      (Value::F32(a), Value::F32(b)) => Value::Bool(a < b),
      (Value::F64(a), Value::F64(b)) => Value::Bool(a < b),
      (Value::Usize(a), Value::Usize(b)) => Value::Bool(a < b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::Bool(a <= b),
      (Value::U16(a), Value::U16(b)) => Value::Bool(a <= b),
      (Value::U32(a), Value::U32(b)) => Value::Bool(a <= b),
      (Value::U64(a), Value::U64(b)) => Value::Bool(a <= b),
      (Value::I8(a), Value::I8(b)) => Value::Bool(a <= b),
      (Value::I16(a), Value::I16(b)) => Value::Bool(a <= b),
      (Value::I32(a), Value::I32(b)) => Value::Bool(a <= b),
      (Value::I64(a), Value::I64(b)) => Value::Bool(a <= b),
      (Value::F32(a), Value::F32(b)) => Value::Bool(a <= b),
      (Value::F64(a), Value::F64(b)) => Value::Bool(a <= b),
      (Value::Usize(a), Value::Usize(b)) => Value::Bool(a <= b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::Bool(a == b),
      (Value::U16(a), Value::U16(b)) => Value::Bool(a == b),
      (Value::U32(a), Value::U32(b)) => Value::Bool(a == b),
      (Value::U64(a), Value::U64(b)) => Value::Bool(a == b),
      (Value::I8(a), Value::I8(b)) => Value::Bool(a == b),
      (Value::I16(a), Value::I16(b)) => Value::Bool(a == b),
      (Value::I32(a), Value::I32(b)) => Value::Bool(a == b),
      (Value::I64(a), Value::I64(b)) => Value::Bool(a == b),
      (Value::F32(a), Value::F32(b)) => Value::Bool(a == b),
      (Value::F64(a), Value::F64(b)) => Value::Bool(a == b),
      (Value::Usize(a), Value::Usize(b)) => Value::Bool(a == b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::Bool(a > b),
      (Value::U16(a), Value::U16(b)) => Value::Bool(a > b),
      (Value::U32(a), Value::U32(b)) => Value::Bool(a > b),
      (Value::U64(a), Value::U64(b)) => Value::Bool(a > b),
      (Value::I8(a), Value::I8(b)) => Value::Bool(a > b),
      (Value::I16(a), Value::I16(b)) => Value::Bool(a > b),
      (Value::I32(a), Value::I32(b)) => Value::Bool(a > b),
      (Value::I64(a), Value::I64(b)) => Value::Bool(a > b),
      (Value::F32(a), Value::F32(b)) => Value::Bool(a > b),
      (Value::F64(a), Value::F64(b)) => Value::Bool(a > b),
      (Value::Usize(a), Value::Usize(b)) => Value::Bool(a > b),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    let result = match (a, b) {
      (Value::U8(a), Value::U8(b)) => Value::Bool(a >= b),
      (Value::U16(a), Value::U16(b)) => Value::Bool(a >= b),
      (Value::U32(a), Value::U32(b)) => Value::Bool(a >= b),
      (Value::U64(a), Value::U64(b)) => Value::Bool(a >= b),
      (Value::I8(a), Value::I8(b)) => Value::Bool(a >= b),
      (Value::I16(a), Value::I16(b)) => Value::Bool(a >= b),
      (Value::I32(a), Value::I32(b)) => Value::Bool(a >= b),
      (Value::I64(a), Value::I64(b)) => Value::Bool(a >= b),
      (Value::F32(a), Value::F32(b)) => Value::Bool(a >= b),
      (Value::F64(a), Value::F64(b)) => Value::Bool(a >= b),
      (Value::Usize(a), Value::Usize(b)) => Value::Bool(a >= b),
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

  fn pop(&mut self, t: DataType, register: Option<u8>) {
    let value = self.stack.pop(&t);

    if let Some(r) = register {
      self.stack.set_register(r, value);
    }
  }

  fn and(&mut self, t: DataType) {
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    match (a, b) {
      (Value::Bool(a), Value::Bool(b)) => self.stack.push_value(Value::Bool(a && b)),
      (Value::U8(a), Value::U8(b)) => self.stack.push_value(Value::U8(a & b)),
      (Value::U16(a), Value::U16(b)) => self.stack.push_value(Value::U16(a & b)),
      (Value::U32(a), Value::U32(b)) => self.stack.push_value(Value::U32(a & b)),
      (Value::U64(a), Value::U64(b)) => self.stack.push_value(Value::U64(a & b)),
      (Value::I8(a), Value::I8(b)) => self.stack.push_value(Value::I8(a & b)),
      (Value::I16(a), Value::I16(b)) => self.stack.push_value(Value::I16(a & b)),
      (Value::I32(a), Value::I32(b)) => self.stack.push_value(Value::I32(a & b)),
      (Value::I64(a), Value::I64(b)) => self.stack.push_value(Value::I64(a & b)),
      (Value::Usize(a), Value::Usize(b)) => self.stack.push_value(Value::Usize(a & b)),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    match (a, b) {
      (Value::Bool(a), Value::Bool(b)) => self.stack.push_value(Value::Bool(a || b)),
      (Value::U8(a), Value::U8(b)) => self.stack.push_value(Value::U8(a | b)),
      (Value::U16(a), Value::U16(b)) => self.stack.push_value(Value::U16(a | b)),
      (Value::U32(a), Value::U32(b)) => self.stack.push_value(Value::U32(a | b)),
      (Value::U64(a), Value::U64(b)) => self.stack.push_value(Value::U64(a | b)),
      (Value::I8(a), Value::I8(b)) => self.stack.push_value(Value::I8(a | b)),
      (Value::I16(a), Value::I16(b)) => self.stack.push_value(Value::I16(a | b)),
      (Value::I32(a), Value::I32(b)) => self.stack.push_value(Value::I32(a | b)),
      (Value::I64(a), Value::I64(b)) => self.stack.push_value(Value::I64(a | b)),
      (Value::Usize(a), Value::Usize(b)) => self.stack.push_value(Value::Usize(a | b)),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    match (a, b) {
      (Value::Bool(a), Value::Bool(b)) => self.stack.push_value(Value::Bool(a ^ b)),
      (Value::U8(a), Value::U8(b)) => self.stack.push_value(Value::U8(a ^ b)),
      (Value::U16(a), Value::U16(b)) => self.stack.push_value(Value::U16(a ^ b)),
      (Value::U32(a), Value::U32(b)) => self.stack.push_value(Value::U32(a ^ b)),
      (Value::U64(a), Value::U64(b)) => self.stack.push_value(Value::U64(a ^ b)),
      (Value::I8(a), Value::I8(b)) => self.stack.push_value(Value::I8(a ^ b)),
      (Value::I16(a), Value::I16(b)) => self.stack.push_value(Value::I16(a ^ b)),
      (Value::I32(a), Value::I32(b)) => self.stack.push_value(Value::I32(a ^ b)),
      (Value::I64(a), Value::I64(b)) => self.stack.push_value(Value::I64(a ^ b)),
      (Value::Usize(a), Value::Usize(b)) => self.stack.push_value(Value::Usize(a ^ b)),
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
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    match a {
      Value::Bool(a) => self.stack.push_value(Value::Bool(!a)),
      Value::U8(a) => self.stack.push_value(Value::U8(!a)),
      Value::U16(a) => self.stack.push_value(Value::U16(!a)),
      Value::U32(a) => self.stack.push_value(Value::U32(!a)),
      Value::U64(a) => self.stack.push_value(Value::U64(!a)),
      Value::I8(a) => self.stack.push_value(Value::I8(!a)),
      Value::I16(a) => self.stack.push_value(Value::I16(!a)),
      Value::I32(a) => self.stack.push_value(Value::I32(!a)),
      Value::I64(a) => self.stack.push_value(Value::I64(!a)),
      Value::Usize(a) => self.stack.push_value(Value::Usize(!a)),
      a => {
        vm_panic("InvalidType", format!("Cannot not {:?}", a).as_str());
        return;
      }
    }
  }

  fn shl(&mut self, t: DataType) {
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&DataType::U8), &DataType::U8);

    match (a, b) {
      (Value::U8(a), Value::U8(b)) => self.stack.push_value(Value::U8(a << b)),
      (Value::U16(a), Value::U16(b)) => self.stack.push_value(Value::U16(a << b)),
      (Value::U32(a), Value::U32(b)) => self.stack.push_value(Value::U32(a << b)),
      (Value::U64(a), Value::U64(b)) => self.stack.push_value(Value::U64(a << b)),
      (Value::I8(a), Value::I8(b)) => self.stack.push_value(Value::I8(a << b)),
      (Value::I16(a), Value::I16(b)) => self.stack.push_value(Value::I16(a << b)),
      (Value::I32(a), Value::I32(b)) => self.stack.push_value(Value::I32(a << b)),
      (Value::I64(a), Value::I64(b)) => self.stack.push_value(Value::I64(a << b)),
      (Value::Usize(a), Value::Usize(b)) => self.stack.push_value(Value::Usize(a << b)),
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
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&DataType::U8), &DataType::U8);

    match (a, b) {
      (Value::U8(a), Value::U8(b)) => self.stack.push_value(Value::U8(a >> b)),
      (Value::U16(a), Value::U16(b)) => self.stack.push_value(Value::U16(a >> b)),
      (Value::U32(a), Value::U32(b)) => self.stack.push_value(Value::U32(a >> b)),
      (Value::U64(a), Value::U64(b)) => self.stack.push_value(Value::U64(a >> b)),
      (Value::I8(a), Value::I8(b)) => self.stack.push_value(Value::I8(a >> b)),
      (Value::I16(a), Value::I16(b)) => self.stack.push_value(Value::I16(a >> b)),
      (Value::I32(a), Value::I32(b)) => self.stack.push_value(Value::I32(a >> b)),
      (Value::I64(a), Value::I64(b)) => self.stack.push_value(Value::I64(a >> b)),
      (Value::Usize(a), Value::Usize(b)) => self.stack.push_value(Value::Usize(a >> b)),
      (a, b) => {
        vm_panic(
          "InvalidType",
          format!("Cannot shr {:?} and {:?}", a, b).as_str(),
        );
        return;
      }
    }
  }

  fn mov(&mut self, reg: u8, value: Value) {
    if reg == 0 {
      vm_panic("InvalidRegister", "Cannot move to register 0");
      return;
    }

    self.stack.set_register(reg, value.to_bytes());
  }

  fn reg(&mut self, reg: u8, item_type: DataType) {
    let value = self.stack.peek_register(reg, &item_type);
    self.stack.push(value);
  }
}
