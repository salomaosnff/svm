use std::{fs::File, time::Instant};

mod lexer;
mod parser;

fn main() {
  let now = Instant::now();
  let code = lexer::code::from_file(File::open("code.txt").unwrap());
  let mut lexer = lexer::from_code(code);
  let op = parser::parse(&mut lexer);
  let elapsed = now.elapsed();

  // println!("Tempo: {:?}", elapsed);

  println!("{:#?}", op.unwrap());
  println!("Tempo: {:?}", elapsed);
}
