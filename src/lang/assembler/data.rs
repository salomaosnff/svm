
#[derive(Debug, Clone)]
pub enum StackValue {
  U8(u8),
  I8(i8),
  U16(u16),
  I16(i16),
  U32(u32),
  I32(i32),
  U64(u64),
  I64(i64),
  F32(f32),
  F64(f64),
  Usize(usize),
  Bool(bool),
  Char(char),
  String(String),
  Record(Vec<(StackValue, StackValue)>),
  Buffer(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum DataType {
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  F32,
  F64,
  Usize,
  Char,
  Bool,
  String,
  Record(&'static DataType, &'static DataType),
  Buffer,
}

impl StackValue {
  pub fn to_stack_bytes(&self) -> Vec<u8> {
    return match self {
      StackValue::U8(value) => value.to_be_bytes().to_vec(),
      StackValue::I8(value) => value.to_le_bytes().to_vec(),
      StackValue::U16(value) => value.to_le_bytes().to_vec(),
      StackValue::I16(value) => value.to_le_bytes().to_vec(),
      StackValue::U32(value) => value.to_le_bytes().to_vec(),
      StackValue::I32(value) => value.to_le_bytes().to_vec(),
      StackValue::U64(value) => value.to_le_bytes().to_vec(),
      StackValue::I64(value) => value.to_le_bytes().to_vec(),
      StackValue::F32(value) => value.to_le_bytes().to_vec(),
      StackValue::F64(value) => value.to_le_bytes().to_vec(),
      StackValue::Usize(value) => value.to_le_bytes().to_vec(),
      StackValue::Bool(value) => if *value { vec![1] } else { vec![0] },
      StackValue::Char(value) => {
        let mut buffer = [0; 4];

        value.encode_utf8(&mut buffer);

        return buffer.to_vec();
      },
      StackValue::String(value) => {
        let mut result = vec![0];

        result.extend_from_slice(value.as_bytes());

        return result;
      },
      StackValue::Record(value) => {
        let mut buffer = vec![];

        for (key, value) in value {
          buffer.extend(key.to_stack_bytes());
          buffer.extend(value.to_stack_bytes());
        }

        buffer.extend(value.len().to_ne_bytes());

        return buffer;
      },
      StackValue::Buffer(value) => value.clone(),
    }
  }

  pub fn from_stack_bytes(data: Vec<u8>, data_type: &DataType) -> Self {
    return match data_type {
      DataType::U8 => StackValue::U8(u8::from_be_bytes(data.try_into().unwrap())),
      DataType::I8 => StackValue::I8(i8::from_be_bytes(data.try_into().unwrap())),
      DataType::U16 => StackValue::U16(u16::from_be_bytes(data.try_into().unwrap())),
      DataType::I16 => StackValue::I16(i16::from_be_bytes(data.try_into().unwrap())),
      DataType::U32 => StackValue::U32(u32::from_be_bytes(data.try_into().unwrap())),
      DataType::I32 => StackValue::I32(i32::from_be_bytes(data.try_into().unwrap())),
      DataType::U64 => StackValue::U64(u64::from_be_bytes(data.try_into().unwrap())),
      DataType::I64 => StackValue::I64(i64::from_be_bytes(data.try_into().unwrap())),
      DataType::F32 => StackValue::F32(f32::from_be_bytes(data.try_into().unwrap())),
      DataType::F64 => StackValue::F64(f64::from_be_bytes(data.try_into().unwrap())),
      DataType::Usize => StackValue::Usize(usize::from_be_bytes(data.try_into().unwrap())),
      DataType::Bool => StackValue::Bool(data[0] != 0),
      DataType::Char => StackValue::Char(char::from_u32(u32::from_be_bytes(data.try_into().unwrap())).unwrap()),
      DataType::String => StackValue::String(String::from_utf8(data[1..].to_vec()).unwrap()),
      DataType::Record(key_type, value_type) => {
        let mut result = vec![];
        let mut index = 0;

        while index < data.len() - std::mem::size_of::<usize>() {
          let key = StackValue::from_stack_bytes(data[index..].to_vec(), key_type);
          let key_size = key.to_stack_bytes().len();

          index += key_size;

          let value = StackValue::from_stack_bytes(data[index..].to_vec(), value_type);
          let value_size = value.to_stack_bytes().len();

          index += value_size;

          result.push((key, value));
        }

        return StackValue::Record(result);
      },
      DataType::Buffer => StackValue::Buffer(data.to_vec()),
    }
  }
}
