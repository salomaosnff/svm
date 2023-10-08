use std::collections::HashMap;

use super::{opcodes, DataType, StackValue};

pub const USIZE_LEN: usize = std::mem::size_of::<usize>();

pub struct Bytecode {
  pub data: Vec<u8>,
  pub labels: HashMap<String, usize>,
  instruction_count: usize,
}

pub fn delimited_string(delimiter: char, str: &str) -> String {
  let string = &mut str.to_string();
  let mut result = String::new();

  if string.starts_with(delimiter) {
    string.remove(0);
  }

  while !string.starts_with(delimiter) {
    let ch = string.remove(0);

    if ch == '\\' {
      let ch = string.remove(0);

      match ch {
        'n' => result.push('\n'),
        'r' => result.push('\r'),
        't' => result.push('\t'),
        '0' => result.push('\0'),
        c => result.push(c),
      }
    } else {
      result.push(ch);
    }
  }

  return result;
}

impl Bytecode {
  pub fn new() -> Self {
    return Bytecode {
      data: Vec::new(),
      labels: HashMap::new(),
      instruction_count: 0,
    };
  }

  fn _opcode(&mut self, opcode: u8) -> &mut Self {
    self.data.push(opcode);
    self.instruction_count += 1;
    return self;
  }

  pub fn label(&mut self, label: &str) -> &mut Self {
    self
      .labels
      .insert(label.to_string(), self.instruction_count - 1);
    return self;
  }

  pub fn push_all(&mut self, items: Vec<u8>) -> &mut Self {
    self._opcode(opcodes::PUSHALL);
    self.data.extend(StackValue::Usize(items.len()).to_bytes());
    self.data.extend(items);

    return self;
  }

  pub fn push(&mut self, item: &[u8]) -> &mut Self {
    for byte in item {
      self._opcode(opcodes::PUSH);
      self.data.push(*byte);
    }
    return self;
  }

  pub fn push_values(&mut self, values: Vec<StackValue>) -> &mut Self {
    let bytes = StackValue::vec_to_bytes(values);

    if bytes.len() >= USIZE_LEN {
      return self.push_all(bytes);
    }

    return self.push(&bytes);
  }

  pub fn _write_char(&mut self, ch: char) -> &mut Self {
    self.data.extend((ch as i32).to_be_bytes());
    return self;
  }

  pub fn _write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
    self.data.extend(bytes);
    return self;
  }

  pub fn _write_label(&mut self, mut label: &str) -> &mut Self {
    if label.starts_with("@") {
      label = &label[1..];
    }

    let address = self.labels.get(&label.to_string()).expect("Invalid label");

    self.data.extend(address.to_be_bytes());
    return self;
  }

  pub fn _write_number_base(&mut self, prefix: &str, base: u32, mut value: &str) -> &mut Self {
    let mut result = String::new();

    if value.starts_with("-") {
      value = &value[1..];
      result.push('-');
    }

    if value.starts_with(prefix) {
      value = &value[prefix.len()..];
      result.push_str(value);
    }

    let value = i32::from_str_radix(result.as_str(), base).expect("Invalid hex number");

    self.data.extend(value.to_be_bytes());

    return self;
  }

  pub fn _type(&mut self, data_type: DataType) -> &mut Self {
    self.data.extend(data_type.to_bytes());

    return self;
  }

  pub fn _value(&mut self, literal: &str) -> &mut Self {
    // Hex number
    if literal.starts_with("0x") || literal.starts_with("-0x") {
      return self._write_number_base("0x", 16, literal);
    }

    // Binary number
    if literal.starts_with("0b") || literal.starts_with("-0b") {
      return self._write_number_base("0b", 2, literal);
    }

    // Octal number
    if literal.starts_with("0o") || literal.starts_with("-0x") {
      return self._write_number_base("0o", 8, literal);
    }

    // Char literal
    if literal.starts_with("'") {
      let char = delimited_string('\'', literal);

      if char.len() != 1 {
        panic!("Invalid char literal");
      }

      return self._write_char(char.chars().next().unwrap());
    }

    // String literal
    if literal.starts_with("\"") {
      let string = delimited_string('"', literal);

      self
        .data
        .extend(string.chars().flat_map(|x| (x as i32).to_be_bytes()));

      return self;
    }

    // Label
    if literal.starts_with("@") {
      return self._write_label(literal);
    }

    // Decimal number
    let value = i32::from_str_radix(&literal, 10).expect("Invalid decimal number");

    self.data.extend(value.to_be_bytes());

    return self;
  }

  pub fn nop(&mut self) -> &mut Self {
    self._opcode(opcodes::NOP);
    return self;
  }

  pub fn halt(&mut self) -> &mut Self {
    self._opcode(opcodes::HALT);
    return self;
  }

  pub fn copy(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::COPY);
    self._type(item_type);
    return self;
  }

  pub fn pop(&mut self, item_type: DataType, reg: u8) -> &mut Self {
    self._opcode(opcodes::POP);
    self._type(item_type);
    self.data.push(reg);
    return self;
  }

  pub fn add(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::ADD);
    self._type(item_type);
    return self;
  }

  pub fn sub(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::SUB);
    self._type(item_type);
    return self;
  }

  pub fn mul(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::MUL);
    self._type(item_type);
    return self;
  }

  pub fn div(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::DIV);
    self._type(item_type);
    return self;
  }

  pub fn modulo(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::MOD);
    self._type(item_type);
    return self;
  }

  pub fn pow(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::POW);
    self._type(item_type);
    return self;
  }

  pub fn inc(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::INC);
    self._type(item_type);
    return self;
  }

  pub fn dec(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::DEC);
    self._type(item_type);
    return self;
  }

  pub fn write(&mut self) -> &mut Self {
    self._opcode(opcodes::WRITE);
    return self;
  }

  pub fn jump(&mut self) -> &mut Self {
    self._opcode(opcodes::JUMP);
    return self;
  }

  pub fn cmp(&mut self) -> &mut Self {
    self._opcode(opcodes::CMP);
    return self;
  }

  pub fn lt(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::LT);
    self._type(item_type);
    return self;
  }

  pub fn eq(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::EQ);
    self._type(item_type);
    return self;
  }

  pub fn neq(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::NEQ);
    self._type(item_type);
    return self;
  }

  pub fn gt(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::GT);
    self._type(item_type);
    return self;
  }

  pub fn gte(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::GTE);
    self._type(item_type);
    return self;
  }

  pub fn lte(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::LTE);
    self._type(item_type);
    return self;
  }

  pub fn msp(&mut self) -> &mut Self {
    self._opcode(opcodes::MSP);
    return self;
  }

  pub fn sp(&mut self) -> &mut Self {
    self._opcode(opcodes::SP);
    return self;
  }

  pub fn pc(&mut self) -> &mut Self {
    self._opcode(opcodes::PC);
    return self;
  }

  pub fn and(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::AND);
    self._type(item_type);
    return self;
  }

  pub fn or(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::OR);
    self._type(item_type);
    return self;
  }

  pub fn xor(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::XOR);
    self._type(item_type);
    return self;
  }

  pub fn not(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::NOT);
    self._type(item_type);
    return self;
  }

  pub fn shl(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::SHL);
    self._type(item_type);
    return self;
  }

  pub fn shr(&mut self, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::SHR);
    self._type(item_type);
    return self;
  }

  pub fn mov(&mut self, register: u8, value: StackValue) -> &mut Self {
    self._opcode(opcodes::MOV);
    self.data.push(register);
    let mut bytes = value.to_bytes();

    if bytes.len() < 8 {
      bytes.splice(0..0, vec![0; 8 - bytes.len()]);
    }

    self.data.extend(bytes);

    return self;
  }

  pub fn reg(&mut self, register: u8, item_type: DataType) -> &mut Self {
    self._opcode(opcodes::REG);
    self.data.push(register);
    self._type(item_type);
    return self;
  }
}
