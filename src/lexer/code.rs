use super::consumer::Consumer;
use std::{
  fmt::Debug,
  fs::File,
  io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct FileReader {
  reader: BufReader<File>,
  queue: Vec<char>,
}

impl FileReader {
  pub fn new(file: File) -> Self {
    Self {
      reader: BufReader::new(file),
      queue: Vec::new(),
    }
  }
}

impl Iterator for FileReader {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    if self.queue.len() <= 0 {
      let mut buffer = String::new();

      self
        .reader
        .read_line(&mut buffer)
        .expect("Falha na leitura do arquivo!");

      if buffer.len() <= 0 {
        return None;
      }

      self.queue.extend(buffer.chars());
    }

    Some(self.queue.remove(0))
  }
}

pub fn from_file(file: File) -> Consumer<char> {
  return Consumer::new(FileReader::new(file));
}

// pub fn from_str<'a>(code: &'a str) -> Code<'a> {
//   Code::new(code.chars())
// }
