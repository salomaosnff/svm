use std::{fs::File, time::Instant};

use crate::runner::{run::Run, scope::Scope};

mod lexer;
mod parser;
mod runner;

fn main() {
  // Clear the terminal
  let now = Instant::now();
  let code = lexer::code::from_file(File::open("code.txt").unwrap());
  let mut lexer = lexer::from_code(code);
  let program = parser::parse(&mut lexer);
  let mut scope = Scope::new();
  let result = program.map(|p| p.run(&mut scope));
  print!("{:#?}", scope.declarations);
  let elapsed = now.elapsed();

  // println!("Tempo: {:?}", elapsed);

  println!("Tempo: {:?}", elapsed);
}
