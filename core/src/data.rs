use std::fmt::Display;

const USIZE_LEN: usize = std::mem::size_of::<usize>();

#[derive(Debug, Clone)]
pub enum Value {
  Bool(bool),
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
  Bytes(Vec<u8>),
}

#[derive(Clone, Copy, Debug)]
pub enum DataType {
  Bool,
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
}

impl Display for DataType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return self.to_string().fmt(f);
  }
}

impl DataType {
  pub fn size(self) -> usize {
    match self {
      DataType::Bool | DataType::U8 | DataType::I8 => 1,
      DataType::U16 | DataType::I16 => 2,
      DataType::U32 | DataType::I32 | DataType::F32 => 4,
      DataType::U64 | DataType::I64 | DataType::F64 => 8,
      DataType::Usize => USIZE_LEN,
    }
  }

  pub fn from_str(str: &str) -> Self {
    return match str {
      "bool" => DataType::Bool,
      "u8" => DataType::U8,
      "i8" => DataType::I8,
      "u16" => DataType::U16,
      "i16" => DataType::I16,
      "u32" => DataType::U32,
      "i32" => DataType::I32,
      "u64" => DataType::U64,
      "i64" => DataType::I64,
      "f32" => DataType::F32,
      "f64" => DataType::F64,
      "usize" => DataType::Usize,
      _ => panic!("Unknown data type: {}", str),
    };
  }

  pub fn to_str(&self) -> String {
    return match self {
      DataType::Bool => String::from("bool"),
      DataType::U8 => String::from("u8"),
      DataType::I8 => String::from("i8"),
      DataType::U16 => String::from("u16"),
      DataType::I16 => String::from("i16"),
      DataType::U32 => String::from("u32"),
      DataType::I32 => String::from("i32"),
      DataType::U64 => String::from("u64"),
      DataType::I64 => String::from("i64"),
      DataType::F32 => String::from("f32"),
      DataType::F64 => String::from("f64"),
      DataType::Usize => String::from("usize"),
    };
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    return match self {
      DataType::U8 => vec![0x00],
      DataType::I8 => vec![0x01],
      DataType::U16 => vec![0x02],
      DataType::I16 => vec![0x03],
      DataType::U32 => vec![0x04],
      DataType::I32 => vec![0x05],
      DataType::U64 => vec![0x06],
      DataType::I64 => vec![0x07],
      DataType::F32 => vec![0x08],
      DataType::F64 => vec![0x09],
      DataType::Usize => vec![0x0A],
      DataType::Bool => vec![0x0C],
    };
  }

  pub fn from_u8(code: u8) -> Self {
    return match code {
      0x00 => DataType::U8,
      0x01 => DataType::I8,
      0x02 => DataType::U16,
      0x03 => DataType::I16,
      0x04 => DataType::U32,
      0x05 => DataType::I32,
      0x06 => DataType::U64,
      0x07 => DataType::I64,
      0x08 => DataType::F32,
      0x09 => DataType::F64,
      0x0A => DataType::Usize,
      0x0C => DataType::Bool,
      _ => panic!("Unknown type_code: [0x{:x}]", code),
    };
  }
}

impl Value {
  pub fn to_bytes(&self) -> Vec<u8> {
    return match self {
      Value::Bool(value) => {
        if *value {
          vec![1]
        } else {
          vec![0]
        }
      }

      Value::U8(value) => value.to_be_bytes().to_vec(),
      Value::I8(value) => value.to_be_bytes().to_vec(),
      Value::U16(value) => value.to_be_bytes().to_vec(),
      Value::I16(value) => value.to_be_bytes().to_vec(),
      Value::U32(value) => value.to_be_bytes().to_vec(),
      Value::I32(value) => value.to_be_bytes().to_vec(),
      Value::U64(value) => value.to_be_bytes().to_vec(),
      Value::I64(value) => value.to_be_bytes().to_vec(),
      Value::F32(value) => value.to_be_bytes().to_vec(),
      Value::F64(value) => value.to_be_bytes().to_vec(),
      Value::Usize(value) => value.to_be_bytes().to_vec(),
      Value::Bytes(value) => value.clone(),
    };
  }

  pub fn from_stack_bytes(data: Vec<u8>, data_type: &DataType) -> Self {
    let size = data_type.clone().size();

    return match data_type {
      DataType::U8 => Value::U8(u8::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::I8 => Value::I8(i8::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::U16 => Value::U16(u16::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::I16 => Value::I16(i16::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::U32 => Value::U32(u32::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::I32 => Value::I32(i32::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::U64 => Value::U64(u64::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::I64 => Value::I64(i64::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::F32 => Value::F32(f32::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::F64 => Value::F64(f64::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::Usize => Value::Usize(usize::from_be_bytes(data[0..size].try_into().unwrap())),
      DataType::Bool => Value::Bool(data[0] != 0),
    };
  }

  pub fn vec_to_bytes(value: Vec<Value>) -> Vec<u8> {
    return value.iter().flat_map(|x| x.to_bytes()).collect::<Vec<u8>>();
  }

  pub fn data_type(&self) -> DataType {
    return match self {
      Value::Bool(_) => DataType::Bool,
      Value::U8(_) => DataType::U8,
      Value::I8(_) => DataType::I8,
      Value::U16(_) => DataType::U16,
      Value::I16(_) => DataType::I16,
      Value::U32(_) => DataType::U32,
      Value::I32(_) => DataType::I32,
      Value::U64(_) => DataType::U64,
      Value::I64(_) => DataType::I64,
      Value::F32(_) => DataType::F32,
      Value::F64(_) => DataType::F64,
      Value::Usize(_) => DataType::Usize,
      Value::Bytes(_) => panic!("Cannot get data type of bytes"),
    };
  }
}
