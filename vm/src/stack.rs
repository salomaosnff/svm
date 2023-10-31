const USIZE_LEN: usize = std::mem::size_of::<usize>();

use svm_lang::{DataType, Value};

use super::util::vm_panic;

pub const MAX_REGISTER_COUNT: usize = 4;

#[derive(Debug)]
pub struct Stack {
  pub data: Vec<u8>,
  pub saves: Vec<usize>,
  pub registers: Vec<u8>,
  pub size: usize,
  pub sp: usize,
}

impl Stack {
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![],
      saves: vec![],
      registers: vec![0; USIZE_LEN * MAX_REGISTER_COUNT],
      size,
      sp: 0,
    }
  }

  pub fn save(&mut self) {
    self.saves.push(self.sp);
  }

  pub fn restore(&mut self) {
    if self.saves.len() == 0 {
      vm_panic("NoSavedStack", "No saved stack to restore!");
    }

    self.sp = self.saves.pop().unwrap();
  }

  pub fn read_bytes(&mut self, size: usize) -> &[u8] {
    if self.sp < size {
      vm_panic("StackUnderflow", "Cannot read from empty stack!");
    }

    let range = self.sp - size..self.sp;

    return &self.data[range];
  }

  pub fn peek(&mut self, item_type: &DataType) -> &[u8] {
    if self.sp == 0 {
      vm_panic("StackUnderflow", "Cannot peek from empty stack!");
    }

    match item_type {
      DataType::U8 => self.read_bytes(1),
      DataType::U16 => self.read_bytes(2),
      DataType::U32 => self.read_bytes(4),
      DataType::U64 => self.read_bytes(8),
      DataType::I8 => self.read_bytes(1),
      DataType::I16 => self.read_bytes(2),
      DataType::I32 => self.read_bytes(4),
      DataType::I64 => self.read_bytes(8),
      DataType::F32 => self.read_bytes(4),
      DataType::F64 => self.read_bytes(8),
      DataType::Usize => self.read_bytes(std::mem::size_of::<usize>()),
      DataType::Bool => self.read_bytes(1),
    }
  }

  pub fn peek_value(&mut self, item_type: &DataType) -> Value {
    return Value::from_stack_bytes(self.peek(item_type).to_vec(), item_type);
  }

  pub fn push_value(&mut self, value: Value) {
    self.push(value.to_bytes());
  }

  pub fn push(&mut self, value: Vec<u8>) {
    if self.sp >= self.size {
      vm_panic("StackOverflow", "Maximum stack size exceeded!");
    }

    let current_size = self.data.len();
    let buffer_size = value.len();
    let available_space = current_size - self.sp;

    if available_space < buffer_size {
      self
        .data
        .resize(current_size + buffer_size - available_space, 0);
    }

    for i in 0..buffer_size {
      self.data[self.sp + i] = value[i];
    }

    self.sp += buffer_size;
  }

  pub fn pop(&mut self, item_type: &DataType) -> Vec<u8> {
    if self.sp == 0 {
      vm_panic("StackUnderflow", "Cannot pop from empty stack!");
    }

    let result = self.peek(item_type).to_vec();

    self.data.splice(self.sp - result.len()..self.sp, vec![]);

    self.sp -= result.len();

    return result;
  }

  pub fn pop_value(&mut self, item_type: &DataType) -> Value {
    return Value::from_stack_bytes(self.pop(item_type), item_type);
  }

  pub fn set_sp(&mut self, offset: usize) {
    if offset > self.size {
      vm_panic("StackOverflow", "Stack pointer out of bounds!");
    }

    self.sp = offset;
  }

  pub fn get_sp(&self) -> usize {
    return self.sp;
  }

  pub fn get_size(&self) -> usize {
    return self.size;
  }

  pub fn dump(&self) -> &Vec<u8> {
    return &self.data;
  }

  pub fn peek_register(&mut self, register: u8, item_type: &DataType) -> Vec<u8> {
    if register as usize >= (self.registers.len() / USIZE_LEN) {
      vm_panic("RegisterOutOfBounds", "Register index out of bounds!");
    }

    let end = ((register - 1) as usize * USIZE_LEN) + USIZE_LEN;

    return match item_type {
      DataType::Bool => self.registers[end - 1..end].to_vec(),
      DataType::U8 => self.registers[end - 1..end].to_vec(),
      DataType::I8 => self.registers[end - 1..end].to_vec(),
      DataType::U16 => self.registers[end - 2..end].to_vec(),
      DataType::I16 => self.registers[end - 2..end].to_vec(),
      DataType::U32 => self.registers[end - 4..end].to_vec(),
      DataType::I32 => self.registers[end - 4..end].to_vec(),
      DataType::U64 => self.registers[end - 8..end].to_vec(),
      DataType::I64 => self.registers[end - 8..end].to_vec(),
      DataType::F32 => self.registers[end - 4..end].to_vec(),
      DataType::F64 => self.registers[end - 8..end].to_vec(),
      DataType::Usize => self.registers[end - USIZE_LEN..end].to_vec(),
    };
  }

  pub fn set_register(&mut self, register: u8, value: Vec<u8>) {
    if register == 0 {
      vm_panic("RegisterOutOfBounds", "Cannot set register 0!");
    }

    if register > MAX_REGISTER_COUNT as u8 {
      vm_panic("RegisterOutOfBounds", "Register index out of bounds!");
    }

    if value.len() > USIZE_LEN {
      vm_panic(
        "RegisterOverflow",
        "Cannot set register with value larger than usize!",
      );
    }

    let start = (register - 1) as usize * USIZE_LEN;
    let end = start + USIZE_LEN;

    let mut bytes = vec![0; USIZE_LEN];

    bytes.splice(USIZE_LEN - value.len()..USIZE_LEN, value);

    self.registers.splice(start..end, bytes);
  }
}
