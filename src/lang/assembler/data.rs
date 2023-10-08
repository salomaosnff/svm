use std::fmt::Display;

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
  Record(Box<DataType>, Box<DataType>),
  Buffer,
}

impl Display for DataType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return self.to_string().fmt(f);
  }
}

impl DataType {
  pub fn from_str(str: &str) -> Self {
    return match str {
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
      "char" => DataType::Char,
      "bool" => DataType::Bool,
      "str" => DataType::String,
      "buffer" => DataType::Buffer,
      t if t.starts_with("record<") && t.ends_with(">") => {
        let mut types = t[7..t.len() - 1].split(",");

        return DataType::Record(
          Box::new(DataType::from_str(types.next().unwrap())),
          Box::new(DataType::from_str(types.next().unwrap())),
        );
      }
      _ => panic!("Unknown data type: {}", str),
    };
  }

  pub fn to_str(&self) -> String {
    return match self {
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
      DataType::Char => String::from("char"),
      DataType::Bool => String::from("bool"),
      DataType::String => String::from("str"),
      DataType::Record(k, v) => format!("record<{},{}>", k.to_str(), v.to_str()),
      DataType::Buffer => String::from("buffer"),
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
      DataType::Char => vec![0x0B],
      DataType::Bool => vec![0x0C],
      DataType::String => vec![0x0D],
      DataType::Record(key_type, value_type) => {
        let mut buffer = vec![14];

        buffer.extend(key_type.to_bytes());
        buffer.extend(value_type.to_bytes());

        return buffer;
      }
      DataType::Buffer => vec![0x0E],
    };
  }

  pub fn from_bytes(data: &mut Vec<u8>) -> Self {
    return match data.remove(0) {
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
      0x0B => DataType::Char,
      0x0C => DataType::Bool,
      0x0D => DataType::String,
      0x0E => DataType::Buffer,
      // 0x0F => {
      //   let key_type = DataType::from_bytes(data[1..].to_vec());
      //   let value_type = DataType::from_bytes(data[1 + key_type.to_bytes().len()..].to_vec());

      //   return DataType::Record(Box::new(key_type), Box::new(value_type));
      // }
      _ => panic!("Unknown type_code: [0x{:x}]", data[0]),
    };
  }
}

impl StackValue {
  pub fn to_bytes(&self) -> Vec<u8> {
    return match self {
      StackValue::U8(value) => value.to_be_bytes().to_vec(),
      StackValue::I8(value) => value.to_be_bytes().to_vec(),
      StackValue::U16(value) => value.to_be_bytes().to_vec(),
      StackValue::I16(value) => value.to_be_bytes().to_vec(),
      StackValue::U32(value) => value.to_be_bytes().to_vec(),
      StackValue::I32(value) => value.to_be_bytes().to_vec(),
      StackValue::U64(value) => value.to_be_bytes().to_vec(),
      StackValue::I64(value) => value.to_be_bytes().to_vec(),
      StackValue::F32(value) => value.to_be_bytes().to_vec(),
      StackValue::F64(value) => value.to_be_bytes().to_vec(),
      StackValue::Usize(value) => value.to_be_bytes().to_vec(),
      StackValue::Bool(value) => {
        if *value {
          vec![1]
        } else {
          vec![0]
        }
      }
      StackValue::Char(value) => {
        let mut buffer = [0; 4];

        value.encode_utf8(&mut buffer);

        return buffer.to_vec();
      }
      StackValue::String(value) => {
        let mut result = vec![0];

        result.extend_from_slice(value.as_bytes());

        return result;
      }
      StackValue::Record(value) => {
        let mut buffer = vec![];

        for (key, value) in value {
          buffer.extend(key.to_bytes());
          buffer.extend(value.to_bytes());
        }

        buffer.extend(value.len().to_ne_bytes());

        return buffer;
      }
      StackValue::Buffer(value) => value.clone(),
    };
  }

  pub fn from_stack_bytes(data: Vec<u8>, data_type: &DataType) -> Self {
    return match data_type {
      DataType::U8 => StackValue::U8(u8::from_be_bytes(data[0..1].try_into().unwrap())),
      DataType::I8 => StackValue::I8(i8::from_be_bytes(data[0..1].try_into().unwrap())),
      DataType::U16 => StackValue::U16(u16::from_be_bytes(data[0..2].try_into().unwrap())),
      DataType::I16 => StackValue::I16(i16::from_be_bytes(data[0..2].try_into().unwrap())),
      DataType::U32 => StackValue::U32(u32::from_be_bytes(data[0..4].try_into().unwrap())),
      DataType::I32 => StackValue::I32(i32::from_be_bytes(data[0..4].try_into().unwrap())),
      DataType::U64 => StackValue::U64(u64::from_be_bytes(data[0..8].try_into().unwrap())),
      DataType::I64 => StackValue::I64(i64::from_be_bytes(data[0..8].try_into().unwrap())),
      DataType::F32 => StackValue::F32(f32::from_be_bytes(data[0..4].try_into().unwrap())),
      DataType::F64 => StackValue::F64(f64::from_be_bytes(data[0..8].try_into().unwrap())),
      DataType::Usize => StackValue::Usize(usize::from_be_bytes(data[0..8].try_into().unwrap())),
      DataType::Bool => StackValue::Bool(data[0] != 0),
      DataType::Char => StackValue::Char(char::from_u32(u32::from_be_bytes(data.try_into().unwrap())).unwrap()),
      DataType::String => {
        let bytes = if data.get(0) == Some(&0) {
          data[1..].to_vec()
        } else {
          data
        };

        return StackValue::String(String::from_utf8(bytes).unwrap());
      },
      DataType::Record(key_type, value_type) => {
        let mut result = vec![];
        let mut index = 0;

        while index < data.len() - std::mem::size_of::<usize>() {
          let key = StackValue::from_stack_bytes(data[index..].to_vec(), key_type);
          let key_size = key.to_bytes().len();

          index += key_size;

          let value = StackValue::from_stack_bytes(data[index..].to_vec(), value_type);
          let value_size = value.to_bytes().len();

          index += value_size;

          result.push((key, value));
        }

        return StackValue::Record(result);
      }
      DataType::Buffer => StackValue::Buffer(data.to_vec()),
    };
  }

  pub fn vec_to_bytes(value: Vec<StackValue>) -> Vec<u8> {
    return value.iter().flat_map(|x| x.to_bytes()).collect::<Vec<u8>>();
  }
}
