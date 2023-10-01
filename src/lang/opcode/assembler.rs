use std::{
  fs::File,
  io::{BufRead, BufReader},
};

use super::opcode::OpCode;

pub trait Assembler {
  fn assemble(&self) -> Vec<u8>;
  fn disassemble(&self) -> String;
  fn from_bytes(bytes: &mut Vec<u8>) -> Option<OpCode>;
}

pub fn assemble(asm: &Vec<OpCode>) -> Vec<u8> {
  let mut buffer = Vec::new();

  for op in asm {
    buffer.append(&mut op.assemble());
  }

  return buffer;
}

pub fn disassemble(buffer: &Vec<u8>) -> Vec<OpCode> {
  let mut asm = Vec::new();
  let mut bytes = buffer.clone();

  while bytes.len() > 0 {
    let op = OpCode::from_bytes(&mut bytes);

    match op {
      Some(op) => asm.push(op),
      None => panic!("Invalid opcode"),
    }
  }

  return asm;
}

pub fn compile_file(file: File) -> Vec<OpCode> {
  let reader = BufReader::new(file);
  let mut buffer = Vec::new();

  for line in reader.lines() {
    let line = line.unwrap();
    let mut line = line.chars().collect::<Vec<char>>().into_iter();
    let mut op = String::new();
    let mut args: Vec<i32> = Vec::new();

    while line.len() > 0 {
      let c = line.next().unwrap();

      if c.is_alphanumeric() {
        op.push(c);
        continue;
      }

      if c.is_whitespace() {
        let c = line.next().unwrap();
        if c == '\'' {
          let char = line.next().expect("Invalid char");

          if char == '\\' {
            let char = line.next().expect("Invalid char");

            match char {
              'n' => args.push('\n' as i32),
              't' => args.push('\t' as i32),
              'r' => args.push('\r' as i32),
              _ => args.push(char as i32),
            }
          } else {
            args.push(char as i32);
          }

          let end = line.next().expect("Invalid char");

          if end != '\'' {
            panic!("Invalid char");
          }

          continue;
        }

        if c == '"' {
          let mut char = line.next().expect("Invalid char");
          let mut length = 0;

          while char != '"' {
            let arg = if char == '\\' {
              char = line.next().expect("Invalid char");

              match char {
                'n' => '\n' as i32,
                't' => '\t' as i32,
                'r' => '\r' as i32,
                _ => char as i32,
              }
            } else {
              char as i32
            };

            buffer.push(OpCode::SPUSH(arg));
            length += 1;
            char = line.next().expect("Invalid char");
          }

          buffer.push(OpCode::SPUSH(length));

          op.clear();

          continue;
        }

        if c == '0' {
          match line.next() {
            Some('x') => {
              let mut hex = String::new();

              while let Some(c) = line.next() {
                if !c.is_ascii_hexdigit() {
                  break;
                }

                hex.push(c);
              }

              args.push(i32::from_str_radix(&hex, 16).unwrap());
            }
            Some('b') => {
              let mut bin = String::new();

              while let Some(c) = line.next() {
                if !c.is_ascii_hexdigit() {
                  break;
                }

                bin.push(c);
              }

              args.push(i32::from_str_radix(&bin, 2).unwrap());
            }
            Some('o') => {
              let mut oct = String::new();

              while let Some(c) = line.next() {
                if !c.is_ascii_hexdigit() {
                  break;
                }

                oct.push(c);
              }

              args.push(i32::from_str_radix(&oct, 8).unwrap());
            }
            Some(c) if c.is_numeric() => {
              let mut dec = String::new();

              dec.push(c);

              while let Some(c) = line.next() {
                if !c.is_numeric() {
                  break;
                }

                dec.push(c);
              }

              args.push(i32::from_str_radix(&dec, 10).unwrap());
            }
            None => {
              args.push(0);
            }
            _ => panic!("Invalid char"),
          }

          continue;
        }

        if c.is_numeric() {
          let mut dec = String::new();

          dec.push(c);

          while let Some(c) = line.next() {
            if !c.is_numeric() {
              break;
            }

            dec.push(c);
          }

          args.push(i32::from_str_radix(&dec, 10).unwrap());

          continue;
        }
      }
    }

    if op.len() == 0 {
      continue;
    }

    match op.as_str() {
      "NOOP" => buffer.push(OpCode::NOP),
      "HALT" => buffer.push(OpCode::HALT),
      "SPEEK" => buffer.push(OpCode::SPEEK),
      "SPUSH" => buffer.push(OpCode::SPUSH(args[0])),
      "INC" => buffer.push(OpCode::INC),
      "DEC" => buffer.push(OpCode::DEC),
      "ADD" => buffer.push(OpCode::ADD),
      "SUB" => buffer.push(OpCode::SUB),
      "MUL" => buffer.push(OpCode::MUL),
      "DIV" => buffer.push(OpCode::DIV),
      "MOD" => buffer.push(OpCode::MOD),
      "POW" => buffer.push(OpCode::POW),
      "WRITE" => buffer.push(OpCode::WRITE),
      "LABEL" => buffer.push(OpCode::LABEL(args[0])),
      "JUMP" => buffer.push(OpCode::JUMP),
      "JUMPI" => buffer.push(OpCode::JUMPI),
      "LT" => buffer.push(OpCode::LT),
      op => panic!("Invalid opcode {op}"),
    }
  }

  return buffer;
}
