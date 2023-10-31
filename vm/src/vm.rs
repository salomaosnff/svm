use std::{fmt::Debug, process::exit};

use svm_lang::{opcodes::OpCode, Program, Type, Value};

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

      self.pc += 1;

      let op = self.program.opcodes[self.pc - 1].clone();

      match op.clone() {
        OpCode::NoOperation => (),
        OpCode::Halt => self.halt(),
        OpCode::Push(value) => self.push(value),
        OpCode::PushAll(values) => self.push_all(values),
        OpCode::Increment(t) => self.increment(t),
        OpCode::Decrement(t) => self.decrement(t),
        OpCode::Add(t) => self.add(t),
        OpCode::Subtraction(t) => self.subtract(t),
        OpCode::Multiply(t) => self.multiply(t),
        OpCode::Divide(t) => self.divide(t),
        OpCode::Modulo(t) => self.modulo(t),
        OpCode::Power(t) => self.power(t),
        OpCode::Copy(t) => self.copy(t),
        OpCode::Equals(t) => self.equals(t),
        OpCode::LessThan(t) => self.less_than(t),
        OpCode::GreaterThan(t) => self.greater_than(t),
        OpCode::LessThanOrEqual(t) => self.less_than_or_equal(t),
        OpCode::GreaterThanOrEqual(t) => self.greater_than_or_equal(t),
        OpCode::Pop(t, r) => self.pop(t, r),
        OpCode::ProgramCounter => self.program_counter(),
        OpCode::StackPointer => self.stack_pointer(),
        OpCode::And(t) => self.and(t),
        OpCode::Or(t) => self.or(t),
        OpCode::XOr(t) => self.xor(t),
        OpCode::Not(t) => self.not(t),
        OpCode::ShiftLeft(t) => self.shift_left(t),
        OpCode::ShiftRight(t) => self.shift_right(t),
        OpCode::Move(reg, value) => self.mov(reg, value),
        OpCode::Register(reg, item_type) => self.register(reg, item_type),
        OpCode::Call(addr) => self.call(addr),
        OpCode::Return => self.ret(),
        OpCode::External(addr) => self.external(addr),
        OpCode::Goto => self.goto(),
        OpCode::GotoIfNotZero => self.goto_if_not_zero(),
        OpCode::GotoIfZero => self.goto_if_zero(),
        OpCode::Jump(addr) => self.jump(addr),
        OpCode::JumpIfNotZero(addr) => self.jump_if_not_zero(addr),
        OpCode::JumpIfZero(addr) => self.jump_if_zero(addr),
        OpCode::MoveStackPointer(offset) => self.move_stack_pointer(offset),
        OpCode::Negative(t) => self.negative(t),
        OpCode::NotEquals(t) => self.not_equals(t),
        OpCode::PushBytes(bytes) => self.push_bytes(bytes),
      };

      std::thread::sleep(std::time::Duration::from_millis(500));

      println!(
        "\x1bcPC={}\nSP={}\nSTACK: {:?}\nREGISTERS: {:?}\nINSTRUCTION: {:?}\n",
        self.pc,
        self.stack.sp,
        self.stack.data,
        self
          .stack
          .registers
          .chunks_exact(std::mem::size_of::<usize>())
          .collect::<Vec<_>>(),
        op,
      );
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

  fn push_bytes(&mut self, bytes: Vec<u8>) {
    self.stack.push(bytes);
  }

  fn increment(&mut self, t: Type) {
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
      Value::Isize(value) => Value::Isize(value + 1),
      _ => {
        vm_panic("InvalidType", "Cannot increment non-number value");
        return;
      }
    };

    self.stack.push_value(new_value)
  }

  fn decrement(&mut self, t: Type) {
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
      Value::Isize(value) => Value::Isize(value - 1),
      _ => {
        vm_panic("InvalidType", "Cannot increment non-number value");
        return;
      }
    };

    self.stack.push_value(new_value)
  }

  fn add(&mut self, item_type: Type) {
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

  fn subtract(&mut self, t: Type) {
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

  fn multiply(&mut self, t: Type) {
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

  fn divide(&mut self, t: Type) {
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

  fn modulo(&mut self, t: Type) {
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

  fn power(&mut self, t: Type) {
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

  fn negative(&mut self, t: Type) {
    let value = self.stack.pop_value(&t);

    let new_value = match value {
      Value::U8(value) => Value::I8(-(value as i8)),
      Value::U16(value) => Value::I16(-(value as i16)),
      Value::U32(value) => Value::I32(-(value as i32)),
      Value::U64(value) => Value::I64(-(value as i64)),
      Value::I8(value) => Value::I8(-value),
      Value::I16(value) => Value::I16(-value),
      Value::I32(value) => Value::I32(-value),
      Value::I64(value) => Value::I64(-value),
      Value::F32(value) => Value::F32(-value),
      Value::F64(value) => Value::F64(-value),
      Value::Usize(value) => Value::Isize(-(value as isize)),
      _ => {
        vm_panic("InvalidType", "Cannot negate non-integer value");
        return;
      }
    };

    self.stack.push_value(new_value)
  }

  pub fn move_stack_pointer(&mut self, offset: isize) {
    let new_sp = if offset.is_negative() {
      self.stack.sp.wrapping_sub(offset.wrapping_abs() as usize)
    } else {
      self.stack.sp.wrapping_add(offset as usize)
    };

    if new_sp > self.stack.size {
      vm_panic("MemoryError", "Stack pointer out of bounds");
    }

    self.stack.sp = new_sp;
  }

  pub fn program_counter(&mut self) {
    self.stack.push_value(Value::Usize(self.pc))
  }

  pub fn stack_pointer(&mut self) {
    self.stack.push_value(Value::Usize(self.stack.sp))
  }

  pub fn jump(&mut self, pc: usize) {
    self.pc = pc;
  }

  pub fn jump_if_not_zero(&mut self, pc: usize) {
    if self.stack.pop_value(&Type::Bool) != Value::Bool(false) {
      self.pc = pc;
    }
  }

  pub fn jump_if_zero(&mut self, pc: usize) {
    if self.stack.pop_value(&Type::Bool) == Value::Bool(false) {
      self.pc = pc;
    }
  }

  fn copy(&mut self, t: Type) {
    let value = self.stack.peek(&t).to_vec();
    self.stack.push(value);
  }

  fn less_than(&mut self, t: Type) {
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

  fn less_than_or_equal(&mut self, t: Type) {
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

  fn equals(&mut self, t: Type) {
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

  fn not_equals(&mut self, t: Type) {
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&t), &t);

    self.stack.push(vec![if a != b { 1 } else { 0 }]);
  }

  fn greater_than(&mut self, t: Type) {
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

  fn greater_than_or_equal(&mut self, t: Type) {
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

  fn pop(&mut self, t: Type, register: Option<u8>) {
    let value = self.stack.pop(&t);

    if let Some(r) = register {
      self.stack.set_register(r, value);
    }
  }

  fn and(&mut self, t: Type) {
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

  fn or(&mut self, t: Type) {
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

  fn xor(&mut self, t: Type) {
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

  fn not(&mut self, t: Type) {
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

  fn shift_left(&mut self, t: Type) {
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&Type::U8), &Type::U8);

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

  fn shift_right(&mut self, t: Type) {
    let b = Value::from_stack_bytes(self.stack.pop(&t), &t);
    let a = Value::from_stack_bytes(self.stack.pop(&Type::U8), &Type::U8);

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
    self.stack.set_register(reg, value.to_bytes());
  }

  fn register(&mut self, reg: u8, item_type: Type) {
    let value = self.stack.peek_register(reg, &item_type);
    self.stack.push(value);
  }

  fn call(&mut self, addr: usize) {
    todo!()
  }

  fn external(&mut self, addr: usize) {
    todo!()
  }

  fn ret(&mut self) {
    todo!()
  }

  fn goto(&mut self) {
    let addr = usize::from_be_bytes(
      self
        .stack
        .peek_register(0, &Type::Usize)
        .try_into()
        .unwrap(),
    );

    self.pc = addr;
  }

  fn goto_if_zero(&mut self) {
    let addr = usize::from_be_bytes(
      self
        .stack
        .peek_register(0, &Type::Usize)
        .try_into()
        .unwrap(),
    );
    let value = self.stack.pop_value(&Type::Bool);

    if value == Value::Bool(false) {
      self.pc = addr
    }
  }

  fn goto_if_not_zero(&mut self) {
    let addr = usize::from_be_bytes(
      self
        .stack
        .peek_register(0, &Type::Usize)
        .try_into()
        .unwrap(),
    );
    let value = self.stack.pop_value(&Type::Bool);

    if value != Value::Bool(false) {
      self.pc = addr
    }
  }
}
