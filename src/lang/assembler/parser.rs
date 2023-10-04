use super::opcodes::{self, OpCode};

pub fn parse(mut data: Vec<u8>) -> Vec<OpCode> {
  let mut result = Vec::new();

  while data.len() > 0 {
    match data.remove(0) {
      opcodes::NOP => result.push(OpCode::NOP),
      opcodes::HALT => result.push(OpCode::HALT),
      // opcodes::INC => result.push(OpCode::INC),
      // opcodes::DEC => result.push(OpCode::DEC),
      // opcodes::ADD => result.push(OpCode::ADD),
      // opcodes::SUB => result.push(OpCode::SUB),
      // opcodes::MUL => result.push(OpCode::MUL),
      // opcodes::DIV => result.push(OpCode::DIV),
      // opcodes::MOD => result.push(OpCode::MOD),
      // opcodes::POW => result.push(OpCode::POW),
      opcodes::WRITE => result.push(OpCode::WRITE),
      opcodes::JUMP => result.push(OpCode::JUMP),
      // opcodes::COPY => result.push(OpCode::COPY),
      // opcodes::POP => result.push(OpCode::POP),
      // opcodes::LT => result.push(OpCode::LT),
      opcodes::CMP => result.push(OpCode::CMP),
      // opcodes::EQ => result.push(OpCode::EQ),
      // opcodes::GT => result.push(OpCode::GT),
      opcodes::PUSH => {
        // let value = data.splice(0..4, vec![]).collect::<Vec<u8>>();
        // result.push(OpCode::PUSH(i32::from_be_bytes(value.try_into().unwrap())));
      }
      opcodes::PUSHALL => {
        let count = data.remove(0);
        let mut values = Vec::new();

        for _ in 0..count {
          let value = data.splice(0..4, vec![]).collect::<Vec<u8>>();

          values.push(i32::from_be_bytes(value.try_into().unwrap()));
        }

        // result.push(OpCode::PUSHALL(values));
      }

      opcodes::MSP => result.push(OpCode::MSP),
      opcodes::SP => result.push(OpCode::SP),
      opcodes::PC => result.push(OpCode::PC),

      op => panic!("Unknown opcode: [0x{:x}]", op),
    };
  }

  return result;
}
