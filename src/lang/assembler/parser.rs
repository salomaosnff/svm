use crate::lang::assembler::StackValue;

use super::{opcodes::{self, OpCode}, DataType, USIZE_LEN};

pub fn parse(mut data: Vec<u8>) -> Vec<OpCode> {
  let mut result = Vec::new();

  while data.len() > 0 {
    match data.remove(0) {
      opcodes::NOP => result.push(OpCode::NOP),
      opcodes::HALT => result.push(OpCode::HALT),
      opcodes::INC => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::INC(item_type))
      },
      opcodes::DEC => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::DEC(item_type))
      },
      opcodes::ADD => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::ADD(item_type));
      },
      opcodes::SUB => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::SUB(item_type));
      }
      opcodes::MUL => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::MUL(item_type));
      },
      opcodes::DIV => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::DIV(item_type));
      },
      opcodes::MOD => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::MOD(item_type));
      },
      opcodes::POW => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::POW(item_type));
      },
      opcodes::WRITE => result.push(OpCode::WRITE),
      opcodes::JUMP => result.push(OpCode::JUMP),
      opcodes::POP => {
        let item_type = DataType::from_bytes(&mut data);
        let reg = data.remove(0);

        result.push(OpCode::POP(item_type, reg));
      }
      opcodes::LT => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::LT(item_type));
      },
      opcodes::LTE => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::LTE(item_type));
      },
      opcodes::GTE => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::GTE(item_type));
      },
      opcodes::CMP => result.push(OpCode::CMP),
      opcodes::EQ => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::EQ(item_type));
      },
      opcodes::GT => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::GT(item_type));
      },
      opcodes::PUSH => {
        let byte = data.remove(0);
        result.push(OpCode::PUSH(byte));
      }
      opcodes::PUSHALL => {
        let bytes = data.splice(0..USIZE_LEN, vec![]).collect::<Vec<u8>>();
        let size = match StackValue::from_stack_bytes(bytes, &DataType::Usize) {
          StackValue::Usize(value) => value,
          _ => panic!("Expected usize after PUSHALL"),
        };

        let bytes = data.splice(0..size, Vec::new()).collect::<Vec<u8>>();

        result.push(OpCode::PUSHALL(bytes));
      }

      opcodes::MSP => result.push(OpCode::MSP),
      opcodes::SP => result.push(OpCode::SP),
      opcodes::PC => result.push(OpCode::PC),
      opcodes::COPY => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::COPY(item_type));
      },
      opcodes::AND => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::AND(item_type));
      },
      opcodes::OR => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::OR(item_type));
      },
      opcodes::XOR => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::XOR(item_type));
      },
      opcodes::NOT => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::NOT(item_type));
      },
      opcodes::SHL => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::SHL(item_type));
      },
      opcodes::SHR => {
        let item_type = DataType::from_bytes(&mut data);
        result.push(OpCode::SHR(item_type));
      },
      opcodes::MOV => {
        let reg = data.remove(0);
        let value = data.splice(0..8, vec![]).collect::<Vec<u8>>();

        result.push(OpCode::MOV(reg, value));
      }

      opcodes::REG => {
        let reg = data.remove(0);
        let item_type = DataType::from_bytes(&mut data);

        result.push(OpCode::REG(reg, item_type));
      }

      op => panic!("Unknown opcode: [0x{:x}]", op),
    };
  }

  return result;
}
