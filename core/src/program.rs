use std::{fs::File, io::Read};

use crate::opcodes::OpCode;

#[derive(Debug)]
pub struct Program {
  pub opcodes: Vec<OpCode>,
}

impl Program {
  pub fn empty() -> Self {
    return Self {
      opcodes: Vec::new(),
    };
  }
  pub fn from_file(mut file: File) -> Self {
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).expect("Could not read file");

    return Self::from_bytes(&mut bytes);
  }
  pub fn from_bytes(bytes: &mut Vec<u8>) -> Self {
    let mut opcodes = Vec::new();

    while bytes.len() > 0 {
      opcodes.push(OpCode::from_bytes(bytes));
    }

    return Self { opcodes };
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    return self
      .opcodes
      .iter()
      .map(|x| x.to_bytes())
      .flatten()
      .collect();
  }
}
