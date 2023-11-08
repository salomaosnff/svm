use std::fmt::Display;

const USIZE_LEN: usize = std::mem::size_of::<usize>();
const ISIZE_LEN: usize = std::mem::size_of::<isize>();

#[derive(Debug, Clone, PartialEq)]
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
  Isize(isize),
  Bytes(Vec<u8>),
  String(String),
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
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
  Isize,
  String,
  Bytes,
}

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return self.to_string().fmt(f);
  }
}

impl Type {
  pub fn size(self) -> usize {
    match self {
      Type::Bool | Type::U8 | Type::I8 => 1,
      Type::U16 | Type::I16 => 2,
      Type::U32 | Type::I32 | Type::F32 => 4,
      Type::U64 | Type::I64 | Type::F64 => 8,
      Type::Usize => USIZE_LEN,
      Type::Isize => ISIZE_LEN,
      Self::Bytes => panic!("Cannot get size of Bytes"),
      Self::String => panic!("Cannot get size of String"),
    }
  }

  pub fn from_str(str: &str) -> Self {
    return match str {
      "bool" => Type::Bool,
      "u8" => Type::U8,
      "i8" => Type::I8,
      "u16" => Type::U16,
      "i16" => Type::I16,
      "u32" => Type::U32,
      "i32" => Type::I32,
      "u64" => Type::U64,
      "i64" => Type::I64,
      "f32" => Type::F32,
      "f64" => Type::F64,
      "usize" => Type::Usize,
      "str" => Type::String,
      "bytes" => Type::Bytes,
      _ => panic!("Unknown data type: {}", str),
    };
  }

  pub fn to_str(&self) -> String {
    return match self {
      Type::Bool => String::from("bool"),
      Type::U8 => String::from("u8"),
      Type::I8 => String::from("i8"),
      Type::U16 => String::from("u16"),
      Type::I16 => String::from("i16"),
      Type::U32 => String::from("u32"),
      Type::I32 => String::from("i32"),
      Type::U64 => String::from("u64"),
      Type::I64 => String::from("i64"),
      Type::F32 => String::from("f32"),
      Type::F64 => String::from("f64"),
      Type::Usize => String::from("usize"),
      Type::Isize => String::from("isize"),
      Type::String => String::from("str"),
      Type::Bytes => String::from("bytes"),
    };
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    return match self {
      Type::U8 => vec![0x00],
      Type::I8 => vec![0x01],
      Type::U16 => vec![0x02],
      Type::I16 => vec![0x03],
      Type::U32 => vec![0x04],
      Type::I32 => vec![0x05],
      Type::U64 => vec![0x06],
      Type::I64 => vec![0x07],
      Type::F32 => vec![0x08],
      Type::F64 => vec![0x09],
      Type::Usize => vec![0x0A],
      Type::Bool => vec![0x0C],
      Type::Isize => vec![0x0D],
      Type::String => vec![0x0E],
      Type::Bytes => vec![0x0F],
    };
  }

  pub fn from_u8(code: u8) -> Self {
    return match code {
      0x00 => Type::U8,
      0x01 => Type::I8,
      0x02 => Type::U16,
      0x03 => Type::I16,
      0x04 => Type::U32,
      0x05 => Type::I32,
      0x06 => Type::U64,
      0x07 => Type::I64,
      0x08 => Type::F32,
      0x09 => Type::F64,
      0x0A => Type::Usize,
      0x0C => Type::Bool,
      0x0D => Type::Isize,
      0x0E => Type::String,
      0x0F => Type::Bytes,
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
      Value::Isize(value) => value.to_be_bytes().to_vec(),
      Value::Bytes(value) => value.clone(),
      Value::String(value) => value.clone().into_bytes(),
    };
  }

  pub fn from_stack_bytes(data: Vec<u8>, data_type: &Type) -> Self {
    return match data_type {
      Type::U8 => Value::U8(u8::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::I8 => Value::I8(i8::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::U16 => Value::U16(u16::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::I16 => Value::I16(i16::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::U32 => Value::U32(u32::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::I32 => Value::I32(i32::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::U64 => Value::U64(u64::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::I64 => Value::I64(i64::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::F32 => Value::F32(f32::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::F64 => Value::F64(f64::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::Usize => Value::Usize(usize::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::Isize => Value::Isize(isize::from_be_bytes(
        data[0..data_type.clone().size()].try_into().unwrap(),
      )),
      Type::Bool => Value::Bool(data[0] != 0),
      Type::String => {
        return Value::String(String::from_utf8(data[1..].to_vec()).unwrap());
      }
      Type::Bytes => Value::Bytes(data),
    };
  }

  pub fn vec_to_bytes(value: Vec<Value>) -> Vec<u8> {
    return value.iter().flat_map(|x| x.to_bytes()).collect::<Vec<u8>>();
  }

  pub fn data_type(&self) -> Type {
    return match self {
      Value::Bool(_) => Type::Bool,
      Value::U8(_) => Type::U8,
      Value::I8(_) => Type::I8,
      Value::U16(_) => Type::U16,
      Value::I16(_) => Type::I16,
      Value::U32(_) => Type::U32,
      Value::I32(_) => Type::I32,
      Value::U64(_) => Type::U64,
      Value::I64(_) => Type::I64,
      Value::F32(_) => Type::F32,
      Value::F64(_) => Type::F64,
      Value::Usize(_) => Type::Usize,
      Value::Isize(_) => Type::Isize,
      Value::Bytes(_) => Type::Bytes,
      Value::String(_) => Type::String,
    };
  }
}
