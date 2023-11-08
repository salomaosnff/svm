use std::{collections::HashMap, fs::File, process::exit, vec};

use svm_lang::{
  lexer::code::{self, Code},
  opcodes::OpCode,
  Program, Type, Value,
};

#[derive(Debug)]
pub enum Operand {
  Number(String),
  Register(String),
  Label(String),
  String(String),
  Type(String),
  Char(String),
  Bool(String),
}

#[derive(Debug)]
pub struct OpcodeToken {
  mnemonic: String,
  operands: Vec<Operand>,
}

#[derive(Debug)]
pub enum InstructionToken {
  Opcode(OpcodeToken),
  Label(String),
}

pub struct AssemblerTokenizer {
  code: Code,
}

fn to_number_digits(number: String) -> (String, u32) {
  let base = if number.starts_with("-0x") || number.starts_with("0x") {
    16
  } else if number.starts_with("-0b") || number.starts_with("0b") {
    2
  } else if number.starts_with("-0o") || number.starts_with("0o") {
    8
  } else {
    10
  };

  let digits = if base == 10 {
    if number.starts_with("-") {
      String::from("-") + &number[1..]
    } else {
      number
    }
  } else if number.starts_with("-") {
    String::from("-") + &number[3..]
  } else {
    String::new() + &number[2..]
  };

  return (digits, base);
}

fn parse_and_infer_number_type(number: String) -> Value {
  if number == "true" || number == "false" {
    return Value::Bool(match number.as_str() {
      "true" => true,
      "false" => false,
      _ => panic!("Expected boolean"),
    });
  }

  let (digits, base) = to_number_digits(number);

  if digits.starts_with("-") {
    return i8::from_str_radix(&digits, base)
      .map(|x| Value::I8(x as i8))
      .or_else(|_| i16::from_str_radix(&digits, base).map(|x| Value::I16(x as i16)))
      .or_else(|_| i32::from_str_radix(&digits, base).map(|x| Value::I32(x as i32)))
      .or_else(|_| i64::from_str_radix(&digits, base).map(|x| Value::I64(x as i64)))
      .expect("Invalid number");
  }

  return u8::from_str_radix(&digits, base)
    .map(|x| Value::U8(x as u8))
    .or_else(|_| u16::from_str_radix(&digits, base).map(|x| Value::U16(x as u16)))
    .or_else(|_| u32::from_str_radix(&digits, base).map(|x| Value::U32(x as u32)))
    .or_else(|_| u64::from_str_radix(&digits, base).map(|x| Value::U64(x as u64)))
    .expect("Invalid number");
}

fn parse_value(value: String, number_type: Option<Type>) -> Value {
  if number_type.is_none() {
    return parse_and_infer_number_type(value);
  }

  let number_type = number_type.unwrap();

  let (digits, base) = to_number_digits(value.clone());

  if digits.starts_with("-") {
    return match number_type {
      Type::I8 => i8::from_str_radix(&digits, base)
        .map(|x| Value::I8(x as i8))
        .expect("Invalid number"),
      Type::I16 => i16::from_str_radix(&digits, base)
        .map(|x| Value::I16(x as i16))
        .expect("Invalid number"),
      Type::I32 => i32::from_str_radix(&digits, base)
        .map(|x| Value::I32(x as i32))
        .expect("Invalid number"),
      Type::I64 => i64::from_str_radix(&digits, base)
        .map(|x| Value::I64(x as i64))
        .expect("Invalid number"),
      _ => panic!("Invalid number type"),
    };
  }

  return match number_type {
    Type::Bool => Value::Bool(match value.as_str() {
      "true" => true,
      "false" => false,
      _ => panic!("Expected boolean"),
    }),
    Type::U8 => u8::from_str_radix(&digits, base)
      .map(|x| Value::U8(x as u8))
      .expect("Invalid number"),
    Type::U16 => u16::from_str_radix(&digits, base)
      .map(|x| Value::U16(x as u16))
      .expect("Invalid number"),
    Type::U32 => u32::from_str_radix(&digits, base)
      .map(|x| Value::U32(x as u32))
      .expect("Invalid number"),
    Type::U64 => u64::from_str_radix(&digits, base)
      .map(|x| Value::U64(x as u64))
      .expect("Invalid number"),
    _ => panic!("Invalid number type"),
  };
}

fn to_isize(number: Value) -> isize {
  match number {
    Value::U8(n) => n as isize,
    Value::U16(n) => n as isize,
    Value::U32(n) => n as isize,
    Value::U64(n) => n as isize,
    Value::I8(n) => n as isize,
    Value::I16(n) => n as isize,
    Value::I32(n) => n as isize,
    Value::I64(n) => n as isize,
    _ => panic!("Expected number"),
  }
}

fn to_usize(number: Value) -> usize {
  match number {
    Value::U8(n) => n as usize,
    Value::U16(n) => n as usize,
    Value::U32(n) => n as usize,
    Value::U64(n) => n as usize,
    Value::I8(n) => n as usize,
    Value::I16(n) => n as usize,
    Value::I32(n) => n as usize,
    Value::I64(n) => n as usize,
    _ => panic!("Expected number"),
  }
}

impl AssemblerTokenizer {
  pub fn next_token(&mut self) -> Option<InstructionToken> {
    loop {
      self.code.consume_while(|c| c.is_whitespace());
      let ch = self.code.peek();

      if ch.is_none() {
        return None;
      }

      let ch = ch.unwrap();

      match ch {
        ';' => {
          self.code.consume_while(|c| *c != '\n');
          continue;
        }
        c if c.is_alphabetic() => {
          let text = self
            .code
            .consume_while(|c| c.is_alphanumeric() || *c == ':')
            .iter()
            .collect::<String>();

          if text.ends_with(":") {
            return Some(InstructionToken::Label(text));
          }

          let mut operands: Vec<Operand> = vec![];

          loop {
            let ch = self.code.peek();

            if ch.is_none() {
              break;
            }

            let ch = ch.unwrap();

            match ch {
              ' ' => {
                self.code.consume();
                continue;
              }
              '\n' => {
                self.code.consume();
                break;
              }
              ';' => {
                self.code.consume_while(|c| *c != '\n');
                break;
              }
              '%' => {
                self.code.consume();
                let text = self
                  .code
                  .consume_while(|c| c.is_alphanumeric())
                  .iter()
                  .collect::<String>();

                operands.push(Operand::Register(text));
              }
              '.' => {
                self.code.consume();
                let text = self
                  .code
                  .consume_while(|c| c.is_alphanumeric())
                  .iter()
                  .collect::<String>();

                operands.push(Operand::Label(text));
              }
              '0' | '-' => {
                let mut text = self.code.consume().unwrap().to_string();

                if text == "-" {
                  match self.code.consume() {
                    Some('0') => text.push('0'),
                    Some(c) if c.is_digit(10) => text.push(c),
                    _ => {
                      println!("Expected number after '-'");
                      exit(1)
                    }
                  }
                }

                match self.code.peek() {
                  Some('x') => {
                    text.push(self.code.consume().unwrap());

                    text.push_str(
                      &self
                        .code
                        .consume_while(|c| c.is_digit(16))
                        .iter()
                        .collect::<String>(),
                    );
                  }
                  Some('b') => {
                    text.push(self.code.consume().unwrap());

                    text.push_str(
                      &self
                        .code
                        .consume_while(|c| c.is_digit(2))
                        .iter()
                        .collect::<String>(),
                    );
                  }
                  Some('o') => {
                    text.push(self.code.consume().unwrap());

                    text.push_str(
                      &self
                        .code
                        .consume_while(|c| c.is_digit(8))
                        .iter()
                        .collect::<String>(),
                    );
                  }
                  Some(c) if c.is_digit(10) => {
                    text.push_str(
                      &self
                        .code
                        .consume_while(|c| c.is_digit(10))
                        .iter()
                        .collect::<String>(),
                    );
                  }
                  _ => {}
                }

                operands.push(Operand::Number(text));
              }
              c if c.is_digit(10) => {
                let text = self
                  .code
                  .consume_while(|c| c.is_digit(10) || *c == '-')
                  .iter()
                  .collect::<String>();

                operands.push(Operand::Number(text));
              }
              '\'' => {
                self.code.consume();

                let mut text = String::new();

                while let Some(ch) = self.code.consume() {
                  if ch == '\'' {
                    operands.push(Operand::Char(text));
                    break;
                  }

                  if text.len() > 0 {
                    println!("Expected single character");
                    exit(1)
                  }

                  if ch == '\\' {
                    let next = self
                      .code
                      .consume()
                      .expect("Expected character after escape");

                    match next {
                      'n' => text.push('\n'),
                      'r' => text.push('\r'),
                      't' => text.push('\t'),
                      '0' => text.push('\0'),
                      c => text.push(c),
                    }
                  }

                  text.push(ch);
                }
              }
              '"' => {
                self.code.consume();

                let mut text = String::new();

                while let Some(ch) = self.code.consume() {
                  if ch == '\"' {
                    operands.push(Operand::String(text));
                    break;
                  }

                  if ch == '\\' {
                    let next = self
                      .code
                      .consume()
                      .expect("Expected character after escape");

                    match next {
                      'n' => text.push('\n'),
                      'r' => text.push('\r'),
                      't' => text.push('\t'),
                      '0' => text.push('\0'),
                      c => text.push(c),
                    }

                    continue;
                  }

                  text.push(ch);
                }
              }
              c if c.is_alphabetic() => {
                let text = self
                  .code
                  .consume_while(|c| c.is_alphanumeric())
                  .iter()
                  .collect::<String>();

                if text == "true" || text == "false" {
                  operands.push(Operand::Bool(text));
                  continue;
                }

                operands.push(Operand::Type(text));
              }
              _ => {
                println!("Unknown character: {}", ch);
                exit(1)
              }
            }
          }

          return Some(InstructionToken::Opcode(OpcodeToken {
            mnemonic: text,
            operands,
          }));
        }
        c => {
          println!("Unknown character: {}", c);
          exit(1)
        }
      }
    }
  }
}

impl Iterator for AssemblerTokenizer {
  type Item = InstructionToken;

  fn next(&mut self) -> Option<Self::Item> {
    return self.next_token();
  }
}

pub fn tokens(file: File) -> AssemblerTokenizer {
  return AssemblerTokenizer {
    code: code::from_file(file),
  };
}

fn get_number(operands: &mut Vec<Operand>) -> Option<Operand> {
  let next_operand = operands.get(0)?;

  match next_operand {
    Operand::Number(_) => return Some(operands.remove(0)),
    _ => return None,
  }
}

fn get_type(operands: &mut Vec<Operand>) -> Option<Operand> {
  let next_operand = operands.get(0)?;

  match next_operand {
    Operand::Type(_) => return Some(operands.remove(0)),
    _ => return None,
  }
}

fn get_register(operands: &mut Vec<Operand>) -> Option<Operand> {
  let next_operand = operands.get(0)?;

  match next_operand {
    Operand::Register(_) => return Some(operands.remove(0)),
    _ => return None,
  }
}

pub fn get_bool(operands: &mut Vec<Operand>) -> Option<Operand> {
  let next_operand = operands.get(0)?;

  match next_operand {
    Operand::Bool(_) => return Some(operands.remove(0)),
    _ => return None,
  }
}

pub fn compile(file: File) -> Program {
  let mut labels = HashMap::new();
  let tokens = AssemblerTokenizer {
    code: code::from_file(file),
  }
  .collect::<Vec<InstructionToken>>();
  let mut opcodes: Vec<OpCode> = Vec::new();
  let mut registers = HashMap::new();
  let mut pc = 0;
  registers.insert("addr", 0x01);
  registers.insert("a", 0x02);
  registers.insert("b", 0x03);
  registers.insert("c", 0x04);

  for token in &tokens {
    match token {
      InstructionToken::Label(label) => {
        labels.insert(label[0..label.len() - 1].to_string(), pc);
      }
      InstructionToken::Opcode(_) => pc += 1,
    }
  }

  for token in tokens {
    match token {
      InstructionToken::Label(_) => continue,
      InstructionToken::Opcode(mut opcode) => match opcode.mnemonic.as_str() {
        "NOP" => opcodes.push(OpCode::NoOperation),
        "HALT" => opcodes.push(OpCode::Halt),
        "MOV" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(&data_type),
            _ => panic!("Expected type"),
          };
          let reg = match get_register(&mut opcode.operands) {
            Some(Operand::Register(register)) => register,
            _ => panic!("Expected register"),
          };

          if opcode.operands.len() <= 0 {
            panic!("Expected value");
          }

          let value = match opcode.operands.remove(0) {
            Operand::Bool(b) => Value::Bool(match b.as_str() {
              "true" => true,
              "false" => false,
              _ => panic!("Expected boolean"),
            }),
            Operand::Char(c) => Value::I32(c.chars().next().unwrap() as i32),
            Operand::Label(label) => Value::Usize(
              labels
                .get(label.as_str())
                .expect(format!("Label \"{}\" not found!", label).as_str())
                .clone(),
            ),
            Operand::Number(number) => match data_type {
              Type::U8
              | Type::U16
              | Type::U32
              | Type::U64
              | Type::I8
              | Type::I16
              | Type::I32
              | Type::I64 => parse_and_infer_number_type(number),
              Type::Bool => Value::Bool(match number.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("Expected boolean"),
              }),
              _ => panic!("Unknown data type {data_type}"),
            },
            Operand::Register(register) => Value::U8(
              registers
                .get(register.as_str())
                .expect("Register not found!")
                .clone(),
            ),
            Operand::String(str) => {
              let bytes = vec![0 as u8]
                .into_iter()
                .chain(str.as_bytes().iter().map(|x| *x))
                .collect::<Vec<u8>>();

              Value::Bytes(bytes)
            }
            _ => panic!("Expected value"),
          };

          opcodes.push(OpCode::Move(
            registers
              .get(reg.as_str())
              .expect(format!("Register \"{}\" is invalid!", reg).as_str())
              .clone(),
            value,
          ));
        }
        "REG" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(&data_type),
            _ => panic!("Expected type"),
          };
          let reg = match get_register(&mut opcode.operands) {
            Some(Operand::Register(register)) => register,
            _ => panic!("Expected register"),
          };

          opcodes.push(OpCode::Register(
            registers.get(reg.as_str()).unwrap().clone(),
            data_type,
          ));
        }
        "PC" => {
          opcodes.push(OpCode::ProgramCounter);
        }
        "SP" => {
          opcodes.push(OpCode::StackPointer);
        }
        "MSP" => {
          let offset = match get_number(&mut opcode.operands) {
            Some(Operand::Number(number)) => to_isize(parse_and_infer_number_type(number)),
            _ => panic!("Expected number"),
          };

          opcodes.push(OpCode::MoveStackPointer(offset));
        }
        "PUSH" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(&data_type),
            _ => {
              let mut operands = opcode
                .operands
                .iter()
                .map(|n| match n {
                  Operand::Number(number) => {
                    parse_and_infer_number_type(number.clone()).data_type()
                  }
                  Operand::Bool(_) => Type::Bool,
                  Operand::Char(_) => Type::I32,
                  Operand::Register(_) => Type::Usize,
                  Operand::String(_) => Type::U8,
                  v => panic!("Invalid operand: {:?}", v),
                })
                .collect::<Vec<Type>>();

              operands.sort_by(|a, b| b.size().cmp(&a.size()));

              operands[0]
            }
          };

          let to_data_type = match data_type {
            Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::Bool
            | Type::Usize => |n: String| parse_value(n, Some(data_type.clone())),
            _ => panic!("Unknown data type {data_type}"),
          };

          let mut stack_values: Vec<Value> = Vec::new();

          fn write_operands(opcodes: &mut Vec<OpCode>, stack_values: &mut Vec<Value>) {
            match stack_values.len() {
              0 => (),
              1 => {
                opcodes.push(OpCode::Push(stack_values.remove(0)));
              }

              _ => {
                opcodes.push(OpCode::PushAll(stack_values.clone()));
              }
            }
            stack_values.clear()
          }

          while opcode.operands.len() > 0 {
            match opcode.operands.remove(0) {
              Operand::Number(number) => stack_values.push(to_data_type(number.clone())),
              Operand::Bool(b) => stack_values.push(to_data_type(b.clone())),
              Operand::Char(c) => stack_values.push(to_data_type(c.clone())),
              Operand::Label(label) => stack_values.push(Value::Usize(
                labels
                  .get(label.as_str())
                  .expect(format!("Label \"{}\" not found!", label).as_str())
                  .clone(),
              )),
              Operand::Register(register) => stack_values.push(Value::U8(
                registers
                  .get(register.as_str())
                  .expect("Register not found!")
                  .clone(),
              )),
              Operand::String(str) => {
                write_operands(&mut opcodes, &mut stack_values);

                let bytes = vec![0 as u8]
                  .into_iter()
                  .chain(str.as_bytes().iter().map(|x| *x))
                  .collect::<Vec<u8>>();

                opcodes.push(OpCode::PushBytes(bytes));
              }
              Operand::Type(t) => panic!("Unexpected type: {}", t),
            }
          }
          write_operands(&mut opcodes, &mut stack_values);
        }
        "TYPE" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(&data_type),
            _ => panic!("Expected type"),
          };

          opcodes.push(OpCode::Push(Value::U8(data_type.to_bytes()[0])));
        }
        "POP" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          let reg = match get_register(&mut opcode.operands) {
            Some(Operand::Register(register)) => Some(register),
            _ => None,
          };

          opcodes.push(OpCode::Pop(
            data_type,
            reg.map(|r| {
              registers
                .get(r.as_str())
                .expect("Register not found!")
                .clone()
            }),
          ));
        }
        "COPY" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Copy(data_type));
        }
        "INC" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Increment(data_type));
        }
        "DEC" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Decrement(data_type));
        }
        "ADD" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Add(data_type));
        }
        "SUB" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Subtraction(data_type));
        }
        "MUL" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Multiply(data_type));
        }
        "DIV" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Divide(data_type));
        }
        "MOD" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };
          opcodes.push(OpCode::Modulo(data_type));
        }
        "NEG" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Negative(data_type));
        }
        "POW" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Power(data_type));
        }
        "AND" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::And(data_type));
        }
        "OR" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Or(data_type));
        }
        "XOR" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::XOr(data_type));
        }
        "NOT" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Not(data_type));
        }
        "SHL" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::ShiftLeft(data_type));
        }
        "SHR" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::ShiftRight(data_type));
        }
        "EQ" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::Equals(data_type));
        }
        "NEQ" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::NotEquals(data_type));
        }
        "GT" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::GreaterThan(data_type));
        }
        "GTE" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::GreaterThanOrEqual(data_type));
        }
        "LT" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::LessThan(data_type));
        }
        "LTE" => {
          let data_type = match get_type(&mut opcode.operands) {
            Some(Operand::Type(data_type)) => Type::from_str(data_type.as_str()),
            _ => Type::I32,
          };

          opcodes.push(OpCode::LessThanOrEqual(data_type));
        }
        "JMP" => {
          let label = match opcode.operands.remove(0) {
            Operand::Label(label) => label,
            _ => panic!("Expected label"),
          };

          opcodes.push(OpCode::Jump(
            labels
              .get(label.as_str())
              .expect(format!("Label \"{}\" not found!", label).as_str())
              .clone(),
          ));
        }
        "JZ" => {
          let label = match opcode.operands.remove(0) {
            Operand::Label(label) => label,
            _ => panic!("Expected label"),
          };

          opcodes.push(OpCode::JumpIfZero(
            labels
              .get(label.as_str())
              .expect(format!("Label \"{}\" not found!", label).as_str())
              .clone(),
          ));
        }
        "JNZ" => {
          let label = match opcode.operands.remove(0) {
            Operand::Label(label) => label,
            _ => panic!("Expected label"),
          };

          opcodes.push(OpCode::JumpIfNotZero(
            labels
              .get(label.as_str())
              .expect(format!("Label \"{}\" not found!", label).as_str())
              .clone(),
          ));
        }
        "GOTO" => {
          opcodes.push(OpCode::Goto);
        }
        "GZ" => {
          opcodes.push(OpCode::GotoIfZero);
        }
        "GNZ" => {
          opcodes.push(OpCode::GotoIfNotZero);
        }
        "EXT" => {
          let address = match get_number(&mut opcode.operands) {
            Some(Operand::Number(number)) => to_usize(parse_and_infer_number_type(number)),
            _ => panic!("Expected address"),
          };

          opcodes.push(OpCode::External(address));
        }
        "CALL" => {
          let address = match get_number(&mut opcode.operands) {
            Some(Operand::Number(number)) => to_usize(parse_and_infer_number_type(number)),
            _ => panic!("Expected address"),
          };

          opcodes.push(OpCode::Call(address));
        }
        "RET" => {
          opcodes.push(OpCode::Return);
        }
        _ => panic!("Unknown mnemonic: {}", opcode.mnemonic),
      },
    }
  }

  return Program { opcodes };
}
