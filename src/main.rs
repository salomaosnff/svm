use std::{fs::File, time::Instant};

use crate::runner::Eval;

mod lexer;
mod parser;
mod runner;

fn main() {
  // Clear the terminal
  println!("\x1bc");
  let code = lexer::code::from_file(File::open("code.txt").unwrap());
  let mut lexer = lexer::from_code(code);
  let program = parser::parse(&mut lexer).expect("Failed to parse program");
  let scope = runner::scope::Scope::new();
  let now = Instant::now();
  let result = program.eval(&scope);
  let elapsed = now.elapsed();

  println!("{}", result);

  println!("Tempo: {:?}", elapsed);
}
