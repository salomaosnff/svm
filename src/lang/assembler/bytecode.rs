use std::collections::HashMap;

use super::opcodes;

pub struct Bytecode {
  pub data: Vec<u8>,
  labels: HashMap<String, i32>,
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

  pub fn push_one(&mut self, item: &str) -> &mut Self {
    if item.starts_with("\"") {
      let mut str = delimited_string('"', item.clone());
      let len = str.chars().count();

      if len <= 2 {
        for i in 0..len {
          self._opcode(opcodes::PUSH);
          let bytes = (str.chars().nth(i).unwrap() as i32).to_be_bytes();
          self.data.extend(bytes);
        }
        return self;
      }

      while !str.is_empty() {
        let slice = &str[..std::cmp::min(str.len(), 255)];
        let len = slice.chars().count() as u8;

        if len <= 2 {
          self._opcode(opcodes::PUSH);
        } else {
          self._opcode(opcodes::PUSHALL);
          self.data.push(len);
        }

        // Write as 4 bytes per char
        self
          .data
          .extend(slice.chars().flat_map(|x| (x as i32).to_be_bytes()));

        str = str[slice.len()..].to_string();
      }

      return self;
    }

    self._opcode(opcodes::PUSH);
    self._value(item);

    return self;
  }

  pub fn label(&mut self, label: &str) -> &mut Self {
    self
      .labels
      .insert(label.to_string(), self.instruction_count as i32 - 1);
    return self;
  }

  pub fn push_all(&mut self, items: Vec<String>) -> &mut Self {
    self._opcode(opcodes::PUSHALL);
    self.data.push(items.len() as u8);

    for item in items {
      self._value(item.as_str());
    }

    return self;
  }

  pub fn push(&mut self, items: Vec<String>) -> &mut Self {
    if items.len() >= 2 {
      return self.push_all(items);
    }

    for item in items {
      self.push_one(item.as_str());
    }

    return self;
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

  pub fn copy(&mut self) -> &mut Self {
    self._opcode(opcodes::COPY);
    return self;
  }

  pub fn pop(&mut self) -> &mut Self {
    self._opcode(opcodes::POP);
    return self;
  }

  pub fn add(&mut self) -> &mut Self {
    self._opcode(opcodes::ADD);
    return self;
  }

  pub fn sub(&mut self) -> &mut Self {
    self._opcode(opcodes::SUB);
    return self;
  }

  pub fn mul(&mut self) -> &mut Self {
    self._opcode(opcodes::MUL);
    return self;
  }

  pub fn div(&mut self) -> &mut Self {
    self._opcode(opcodes::DIV);
    return self;
  }

  pub fn modulo(&mut self) -> &mut Self {
    self._opcode(opcodes::MOD);
    return self;
  }

  pub fn pow(&mut self) -> &mut Self {
    self._opcode(opcodes::POW);
    return self;
  }

  pub fn inc(&mut self) -> &mut Self {
    self._opcode(opcodes::INC);
    return self;
  }

  pub fn dec(&mut self) -> &mut Self {
    self._opcode(opcodes::DEC);
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

  pub fn lt(&mut self) -> &mut Self {
    self._opcode(opcodes::LT);
    return self;
  }

  pub fn eq(&mut self) -> &mut Self {
    self._opcode(opcodes::EQ);
    return self;
  }

  pub fn gt(&mut self) -> &mut Self {
    self._opcode(opcodes::GT);
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
}
