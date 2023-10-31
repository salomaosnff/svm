use std::io::{Read, Write};

use super::IO;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl IO for Stdin {
  fn read(&mut self, buffer: &mut [u8]) {
    std::io::stdin()
      .read_exact(buffer)
      .expect("Falha na leitura do teclado!");
  }

  fn write(&mut self, _: &[u8]) {
    eprintln!("STDIN é somente leitura!")
  }
}

impl IO for Stdout {
  fn read(&mut self, _: &mut [u8]) {
    eprintln!("STDOUT é somente escrita!")
  }

  fn write(&mut self, buffer: &[u8]) {
    std::io::stdout()
      .lock()
      .write_all(buffer)
      .expect("Falha na escrita do terminal!");

    // println!("STDOUT >> {:?}", buffer);
  }
}

impl IO for Stderr {
  fn read(&mut self, _: &mut [u8]) {
    eprintln!("STDERR é somente escrita!")
  }

  fn write(&mut self, buffer: &[u8]) {
    // std::io::stdout()
    //   .lock()
    //   .write_all(buffer)
    //   .expect("Falha na escrita do terminal!");

    println!("STDERR >> {:?}", buffer);
  }
}
