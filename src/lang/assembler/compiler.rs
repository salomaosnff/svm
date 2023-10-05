use std::fs::File;
use std::io::BufRead;

use super::bytecode::Bytecode;
use super::{DataType, StackValue};

#[derive(Debug)]
pub enum Operand {
  Type(DataType),
  Value(StackValue),
}

#[derive(Debug)]
pub struct InstructionOpCode {
  pub opcode: String,
  pub operands: Vec<Operand>,
}

#[derive(Debug)]
pub enum Instruction {
  OpCode(InstructionOpCode),
  Label(String),
}

fn parse_number(number: &str, number_type: DataType) -> StackValue {
  if number.starts_with("0x") {
    return match number_type {
      DataType::I8 => StackValue::I8(i8::from_str_radix(&number[2..], 16).unwrap()),
      DataType::I16 => StackValue::I16(i16::from_str_radix(&number[2..], 16).unwrap()),
      DataType::I32 => StackValue::I32(i32::from_str_radix(&number[2..], 16).unwrap()),
      DataType::I64 => StackValue::I64(i64::from_str_radix(&number[2..], 16).unwrap()),
      DataType::U8 => StackValue::U8(u8::from_str_radix(&number[2..], 16).unwrap()),
      DataType::U16 => StackValue::U16(u16::from_str_radix(&number[2..], 16).unwrap()),
      DataType::U32 => StackValue::U32(u32::from_str_radix(&number[2..], 16).unwrap()),
      DataType::U64 => StackValue::U64(u64::from_str_radix(&number[2..], 16).unwrap()),
      DataType::Usize => StackValue::Usize(usize::from_str_radix(&number[2..], 16).unwrap()),
      _ => panic!("Cannot parse hex number to {number_type}"),
    };
  }

  if number.starts_with("0o") {
    return match number_type {
      DataType::I8 => StackValue::I8(i8::from_str_radix(&number[2..], 8).unwrap()),
      DataType::I16 => StackValue::I16(i16::from_str_radix(&number[2..], 8).unwrap()),
      DataType::I32 => StackValue::I32(i32::from_str_radix(&number[2..], 8).unwrap()),
      DataType::I64 => StackValue::I64(i64::from_str_radix(&number[2..], 8).unwrap()),
      DataType::U8 => StackValue::U8(u8::from_str_radix(&number[2..], 8).unwrap()),
      DataType::U16 => StackValue::U16(u16::from_str_radix(&number[2..], 8).unwrap()),
      DataType::U32 => StackValue::U32(u32::from_str_radix(&number[2..], 8).unwrap()),
      DataType::U64 => StackValue::U64(u64::from_str_radix(&number[2..], 8).unwrap()),
      DataType::Usize => StackValue::Usize(usize::from_str_radix(&number[2..], 16).unwrap()),
      _ => panic!("Cannot parse oct number to {number_type}"),
    };
  }

  if number.starts_with("0b") {
    return match number_type {
      DataType::I8 => StackValue::I8(i8::from_str_radix(&number[2..], 2).unwrap()),
      DataType::I16 => StackValue::I16(i16::from_str_radix(&number[2..], 2).unwrap()),
      DataType::I32 => StackValue::I32(i32::from_str_radix(&number[2..], 2).unwrap()),
      DataType::I64 => StackValue::I64(i64::from_str_radix(&number[2..], 2).unwrap()),
      DataType::U8 => StackValue::U8(u8::from_str_radix(&number[2..], 2).unwrap()),
      DataType::U16 => StackValue::U16(u16::from_str_radix(&number[2..], 2).unwrap()),
      DataType::U32 => StackValue::U32(u32::from_str_radix(&number[2..], 2).unwrap()),
      DataType::U64 => StackValue::U64(u64::from_str_radix(&number[2..], 2).unwrap()),
      DataType::Usize => StackValue::Usize(usize::from_str_radix(&number[2..], 16).unwrap()),
      _ => panic!("Cannot parse bin number to {number_type}"),
    };
  }

  match number_type {
    DataType::I8 => StackValue::I8(i8::from_str_radix(number, 10).unwrap()),
    DataType::I16 => StackValue::I16(i16::from_str_radix(number, 10).unwrap()),
    DataType::I32 => StackValue::I32(i32::from_str_radix(number, 10).unwrap()),
    DataType::I64 => StackValue::I64(i64::from_str_radix(number, 10).unwrap()),
    DataType::U8 => StackValue::U8(u8::from_str_radix(number, 10).unwrap()),
    DataType::U16 => StackValue::U16(u16::from_str_radix(number, 10).unwrap()),
    DataType::U32 => StackValue::U32(u32::from_str_radix(number, 10).unwrap()),
    DataType::U64 => StackValue::U64(u64::from_str_radix(number, 10).unwrap()),
    DataType::Usize => StackValue::Usize(usize::from_str_radix(number, 10).unwrap()),
    _ => panic!("Cannot parse dec number to {number_type}"),
  }
}

fn parse_line(line: &str) -> Option<Instruction> {
  let mut line = line.trim().to_string();

  if line.is_empty() {
    return None;
  }

  if line.starts_with("@") {
    line.remove(0);
    return Some(Instruction::Label(line));
  }

  let mut opcode = String::new();
  let mut operands: Vec<Operand> = Vec::new();

  while line.len() > 0
    && line
      .chars()
      .next()
      .expect("Expected opcode!")
      .is_alphanumeric()
  {
    opcode.push(line.remove(0));
  }

  line = line.trim().to_string();

  if opcode.is_empty() {
    return None;
  }

  while line.len() > 0 {
    line = line.trim().to_string();
    let mut operand = String::new();

    if line.starts_with(";") {
      break;
    }

    let number = if line.starts_with("0x") {
      let mut number: String = line[0..2].to_string();
      line = line[2..].to_string();

      while line.len() > 0 && line.chars().next().unwrap().is_ascii_hexdigit() {
        number.push(line.remove(0));
      }

      number
    } else if line.starts_with("0b") {
      let mut number: String = line[0..2].to_string();
      line = line[2..].to_string();

      while line.len() > 0 && line.chars().next().unwrap().is_ascii_hexdigit() {
        number.push(line.remove(0));
      }

      number
    } else if line.starts_with("0o") {
      let mut number: String = line[0..2].to_string();
      line = line[2..].to_string();

      while line.len() > 0 && line.chars().next().unwrap().is_ascii_hexdigit() {
        number.push(line.remove(0));
      }

      number
    } else {
      let mut number = String::new();

      while line.len() > 0 && line.chars().next().unwrap().is_ascii_digit() {
        number.push(line.remove(0));
      }

      number
    };

    // parse number
    if !number.is_empty() {
      let number_type = match operands.last() {
        Some(Operand::Type(data_type)) => {
          let number_type = match data_type {
            DataType::I8 => Some(DataType::I8),
            DataType::I16 => Some(DataType::I16),
            DataType::I32 => Some(DataType::I32),
            DataType::I64 => Some(DataType::I64),
            DataType::U8 => Some(DataType::U8),
            DataType::U16 => Some(DataType::U16),
            DataType::U32 => Some(DataType::U32),
            DataType::U64 => Some(DataType::U64),
            DataType::F32 => Some(DataType::F32),
            DataType::F64 => Some(DataType::F64),
            DataType::Usize => Some(DataType::Usize),
            _ => None,
          };

          if number_type.is_some() {
            operands.pop();
            number_type.unwrap()
          } else {
            DataType::I32
          }
        }
        _ => DataType::I32,
      };

      operands.push(Operand::Value(parse_number(number.as_str(), number_type)));

      continue;
    }

    // parse string
    if line.starts_with("\"") {
      line.remove(0);

      let mut string = String::new();

      while !line.is_empty() && !line.starts_with("\"") {
        let ch = line.remove(0);

        if ch == '\\' {
          let ch = line.remove(0);

          match ch {
            'n' => string.push('\n'),
            'r' => string.push('\r'),
            't' => string.push('\t'),
            '0' => string.push('\0'),
            c => string.push(c),
          }
        } else {
          string.push(ch);
        }
      }

      if line.is_empty() {
        panic!("Expected '\"'");
      }

      line.remove(0);

      operands.push(Operand::Value(StackValue::String(string)));

      continue;
    }

    // parse char
    if line.starts_with("'") {
      let mut string = String::new();

      while !line.is_empty() && !line.starts_with("'") {
        let ch = line.remove(0);

        if ch == '\\' {
          let ch = line.remove(0);

          match ch {
            'n' => string.push('\n'),
            'r' => string.push('\r'),
            't' => string.push('\t'),
            '0' => string.push('\0'),
            c => string.push(c),
          }
        } else {
          string.push(ch);
        }
      }

      if line.is_empty() {
        panic!("Expected \"'\"");
      }

      line.remove(0);

      if string.chars().count() > 1 {
        panic!("Expected single character");
      }

      operands.push(Operand::Value(StackValue::Char(
        string.chars().next().unwrap(),
      )));

      continue;
    }

    // true
    if line.starts_with("true") {
      line = line[4..].to_string();
      operands.push(Operand::Value(StackValue::Bool(true)));
      continue;
    }

    // false
    if line.starts_with("false") {
      line = line[5..].to_string();
      operands.push(Operand::Value(StackValue::Bool(false)));
      continue;
    }

    while line.len() > 0 && !line.starts_with(" ") {
      let c = line.remove(0);
      operand.push(c);

      if c == '<' {
        while !line.is_empty() && !line.starts_with(">") {
          operand.push(line.remove(0));
        }

        if line.is_empty() {
          panic!("Expected '>'");
        }

        operand.push(line.remove(0));
      }
    }

    if !operand.is_empty() {
      match operand.as_str() {
        "i8" => operands.push(Operand::Type(DataType::I8)),
        "i16" => operands.push(Operand::Type(DataType::I16)),
        "i32" => operands.push(Operand::Type(DataType::I32)),
        "i64" => operands.push(Operand::Type(DataType::I64)),
        "u8" => operands.push(Operand::Type(DataType::U8)),
        "u16" => operands.push(Operand::Type(DataType::U16)),
        "u32" => operands.push(Operand::Type(DataType::U32)),
        "u64" => operands.push(Operand::Type(DataType::U64)),
        "f32" => operands.push(Operand::Type(DataType::F32)),
        "f64" => operands.push(Operand::Type(DataType::F64)),
        "str" => operands.push(Operand::Type(DataType::String)),
        "char" => operands.push(Operand::Type(DataType::Char)),
        "bool" => operands.push(Operand::Type(DataType::Bool)),
        "usize" => operands.push(Operand::Type(DataType::Usize)),
        _ => panic!("Unknown operand: {}", operand),
      }
    }
  }

  return Some(Instruction::OpCode(InstructionOpCode { opcode, operands }));
}

pub fn compile(source_file: File) -> Bytecode {
  let reader = std::io::BufReader::new(source_file);
  let mut file = Bytecode::new();

  for line in reader.lines() {
    let line = line.unwrap();

    match parse_line(line.as_str()) {
      Some(Instruction::Label(label)) => {
        file.label(label.as_str());
      }
      Some(Instruction::OpCode(mut opcode)) => {
        match opcode.opcode.as_str() {
          "PUSH" => {
            let values = opcode
              .operands
              .iter()
              .map(|operand| match operand {
                Operand::Value(value) => value.clone(),
                _ => panic!("Expected value"),
              })
              .collect::<Vec<StackValue>>();

            file.push_values(values);
          }
          "ADD" => {
            if opcode.operands.len() <= 0 {
              panic!("ADD expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.add(item_type);
              }
              _ => panic!("ADD expect a type!"),
            }
          }
          "SUB" => {
            if opcode.operands.len() <= 0 {
              panic!("SUB expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.sub(item_type);
              }
              _ => panic!("SUB expect a type!"),
            }
          }
          "MUL" => {
            if opcode.operands.len() <= 0 {
              panic!("MUL expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.mul(item_type);
              }
              _ => panic!("MUL expect a type!"),
            }
          }
          "DIV" => {
            if opcode.operands.len() <= 0 {
              panic!("DIV expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.div(item_type);
              }
              _ => panic!("DIV expect a type!"),
            }
          }
          "MOD" => {
            if opcode.operands.len() <= 0 {
              panic!("MOD expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.modulo(item_type);
              }
              _ => panic!("MOD expect a type!"),
            }
          }
          "POW" => {
            if opcode.operands.len() <= 0 {
              panic!("POW expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.pow(item_type);
              }
              _ => panic!("POW expect a type!"),
            }
          }
          "WRITE" => {
            file.write();
          }
          "JUMP" => {
            file.jump();
          }
          "CMP" => {
            file.cmp();
          }
          "COPY" => {
            if opcode.operands.len() <= 0 {
              panic!("COPY expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.copy(item_type);
              }
              _ => panic!("COPY expect a type!"),
            }
          }
          "LT" => {
            if opcode.operands.len() <= 0 {
              panic!("LT expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.lt(item_type);
              }
              _ => panic!("LT expect a type!"),
            }
          }
          "GT" => {
            if opcode.operands.len() <= 0 {
              panic!("GT expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.gt(item_type);
              }
              _ => panic!("GT expect a type!"),
            }
          }
          "GTE" => {
            if opcode.operands.len() <= 0 {
              panic!("GTE expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.gte(item_type);
              }
              _ => panic!("GTE expect a type!"),
            }
          }
          "LTE" => {
            if opcode.operands.len() <= 0 {
              panic!("LTE expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.lte(item_type);
              }
              _ => panic!("LTE expect a type!"),
            }
          }
          "EQ" => {
            if opcode.operands.len() <= 0 {
              panic!("EQ expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.eq(item_type);
              }
              _ => panic!("EQ expect a type!"),
            }
          }
          "NEQ" => {
            if opcode.operands.len() <= 0 {
              panic!("NEQ expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.neq(item_type);
              }
              _ => panic!("NEQ expect a type!"),
            }
          }
          "INC" => {
            if opcode.operands.len() <= 0 {
              panic!("INC expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.inc(item_type);
              }
              _ => panic!("INC expect a type!"),
            }
          }
          "DEC" => {
            if opcode.operands.len() <= 0 {
              panic!("DEC expect a type!")
            }

            match opcode.operands.remove(0) {
              Operand::Type(item_type) => {
                file.dec(item_type);
              }
              _ => panic!("DEC expect a type!"),
            }
          }
          _ => panic!("Opcode {:#?} not implemented in compiler", opcode),
        };
      }
      _ => continue,
    };
  }

  return file;
}
