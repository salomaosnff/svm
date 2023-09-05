use crate::parser::nodes::{unary_expression, ternary_expression};
use std::{fs::File, time::Instant};

mod lexer;
mod parser;

fn main() {
  let code = lexer::code::from_file(File::open("code.txt").unwrap());
  let mut lexer = lexer::from_code(code);
  let now = Instant::now();
  // let op = ternary_expression::parse(&mut lexer);

  // println!("Tempo: {:?}", elapsed);

  let elapsed = now.elapsed();

  // println!("{:#?}", op.unwrap());
  println!("Tempo: {:?}", elapsed);
  for ch in lexer {
    print!("{:#?}", ch)
  }
}