use std::fs::File;
use std::io::BufRead;

use super::bytecode::Bytecode;


#[derive(Debug)]
pub struct InstructionOpCode {
  pub opcode: String,
  pub operands: Vec<String>,
}

#[derive(Debug)]
pub enum Instruction {
  OpCode(InstructionOpCode),
  Label(String),
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
  let mut operands: Vec<String> = Vec::new();

  while line.len() > 0 && line.chars().next().expect("Expected opcode!").is_alphanumeric() {
    opcode.push(line.remove(0));
  }

  line = line.trim().to_string();

  
  while line.len() > 0 {
    let mut operand = String::new();
    
    if line.starts_with(";") {
      break;
    }

    if line.starts_with("\"") || line.starts_with("'") {
      let delimiter = line.remove(0);

      operand.push(delimiter);

      while !line.starts_with(delimiter) {
        operand.push(line.remove(0));
      }

      operand.push(line.remove(0));
    } else {
      while line.len() > 0 && !line.starts_with(" ") {
        operand.push(line.remove(0));
      }
    }

    line = line.trim().to_string();

    operands.push(operand);
  }

  return Some(Instruction::OpCode(InstructionOpCode { opcode, operands }));
}

pub fn compile(mut source_file: File) -> Bytecode {
  let reader = std::io::BufReader::new(source_file);
  let mut file = Bytecode::new();

  for line in reader.lines() {
    let line = line.unwrap();

    match parse_line(line.as_str()) {
      Some(Instruction::Label(label)) => file.label(label.as_str()),
      Some(Instruction::OpCode(opcode)) => match opcode.opcode.as_str() {
        "NOP" => file.nop(),
        "HALT" => file.halt(),
        "COPY" => file.copy(),
        "PUSH" => file.push(opcode.operands),
        "POP" => file.pop(),
        "INC" => file.inc(),
        "DEC" => file.dec(),
        "ADD" => file.add(),
        "SUB" => file.sub(),
        "MUL" => file.mul(),
        "DIV" => file.div(),
        "MOD" => file.modulo(),
        "POW" => file.pow(),
        "WRITE" => file.write(),
        "JUMP" => file.jump(),
        "CMP" => file.cmp(),
        "LT" => file.lt(),
        "EQ" => file.eq(),
        "GT" => file.gt(),
        op => panic!("Opcode {op} not implemented in compiler"),
      },
      _ => continue,
    };
  }

  return file;
}
