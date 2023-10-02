use std::fmt::Debug;

use crate::lang::assembler::opcodes::OpCode;

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
        OpCode::PUSHALL(values) => {
          for value in values {
            self.push(value);
          }
        }
        OpCode::INC => {
          let value = i32::from_be_bytes(self.stack.pop());
          self.stack.push((value + 1).to_be_bytes());
        }
        OpCode::DEC => {
          let value = i32::from_be_bytes(self.stack.pop());
          self.stack.push((value - 1).to_be_bytes());
        }
        OpCode::ADD => self.add(),
        OpCode::SUB => self.sub(),
        OpCode::MUL => self.mul(),
        OpCode::DIV => self.div(),
        OpCode::MOD => self.modulo(),
        OpCode::POW => self.pow(),
        OpCode::WRITE => self.write(),
        OpCode::JUMP => {
          let label = i32::from_be_bytes(self.stack.pop());

          self.pc = label as usize;
        }
        OpCode::COPY => {
          let value = self.stack.peek();

          self.stack.push(value);
        }
        OpCode::LT => {
          let b = i32::from_be_bytes(self.stack.pop());
          let a = i32::from_be_bytes(self.stack.pop());

          self.stack.push(((a < b) as i32).to_be_bytes());
        }
        OpCode::CMP => {
          let label = i32::from_be_bytes(self.stack.pop());
          let condition = i32::from_be_bytes(self.stack.pop());

          if condition != 0 {
            self.pc = label as usize;
          }
        }
        OpCode::EQ => {
          let b = i32::from_be_bytes(self.stack.pop());
          let a = i32::from_be_bytes(self.stack.pop());

          self.stack.push(((a == b) as i32).to_be_bytes());
        }
        OpCode::GT => {
          let b = i32::from_be_bytes(self.stack.pop());
          let a = i32::from_be_bytes(self.stack.pop());

          self.stack.push(((a > b) as i32).to_be_bytes());
        }
        OpCode::POP => {
          self.stack.pop();
        }
      }

      self.pc += 1;
    }
  }

  fn halt(&mut self) {
    self.running = false;
  }

  fn push(&mut self, value: i32) {
    self.stack.push(value.to_be_bytes());
  }

  fn add(&mut self) {
    let b = i32::from_be_bytes(self.stack.pop());
    let a = i32::from_be_bytes(self.stack.pop());

    self.stack.push((a + b).to_be_bytes());
  }

  fn sub(&mut self) {
    let b = i32::from_be_bytes(self.stack.pop());
    let a = i32::from_be_bytes(self.stack.pop());

    self.stack.push((a - b).to_be_bytes());
  }

  fn mul(&mut self) {
    let b = i32::from_be_bytes(self.stack.pop());
    let a = i32::from_be_bytes(self.stack.pop());

    self.stack.push((a * b).to_be_bytes());
  }

  fn div(&mut self) {
    let b = i32::from_be_bytes(self.stack.pop());
    let a = i32::from_be_bytes(self.stack.pop());

    self.stack.push((a / b).to_be_bytes());
  }

  fn modulo(&mut self) {
    let b = i32::from_be_bytes(self.stack.pop());
    let a = i32::from_be_bytes(self.stack.pop());

    self.stack.push((a % b).to_be_bytes());
  }

  fn pow(&mut self) {
    let b = i32::from_be_bytes(self.stack.pop());
    let a = i32::from_be_bytes(self.stack.pop());

    self.stack.push(a.pow(b as u32).to_be_bytes());
  }

  fn write(&mut self) {
    let descriptor = i32::from_be_bytes(self.stack.pop());
    let count = i32::from_be_bytes(self.stack.pop()) as usize;

    let mut buffer = vec![0; count as usize];

    for i in 0..count {
      let value = i32::from_be_bytes(self.stack.pop());

      buffer[count - i - 1] = value as u8;
    }

    self.io[descriptor as usize].write(&buffer);
  }
}
