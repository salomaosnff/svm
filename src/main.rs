use crate::parser::nodes::unary_operation;
use std::{fs::File, time::Instant};

mod lexer;
mod parser;

fn main() {
  let code = lexer::code::from_file(File::open("code.txt").unwrap());
  let mut lexer = lexer::from_code(code);
  let now = Instant::now();
  let op = unary_operation::signal_operation(&mut lexer);

  // println!("Tempo: {:?}", elapsed);

  let elapsed = now.elapsed();
  println!("{:#?}", op.unwrap());
  println!("Tempo: {:?}", elapsed);
  // for ch in lexer {
  // }
  // let mut lexer = lexer::Lexer::new(lexer::code::from_file(File::open("code.txt").unwrap()));
  // let ast = parser::parse(&lexer);

  // println!("{:?}", ast);
  // println!("FIM")

  // loop {
  //   match code.read() {
  //     Some(c) => println!("{}", c),
  //     _ => break,
  //   }
  // }
}
