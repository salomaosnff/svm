use std::{fs::File, time::Instant};

use crate::runner::{value::{NativeFunction, Value}, Eval};

mod lexer;
mod parser;
mod runner;

fn run<U, T: FnOnce() -> U>(f: T) -> U {
  let now = Instant::now();
  let result = f();
  let elapsed = now.elapsed();

  println!("Tempo: {:?}", elapsed);

  return result;
}

fn main() {
  // Clear the terminal
  println!("\x1bc");
  let code = lexer::code::from_file(File::open("code.txt").unwrap());
  let mut lexer = lexer::from_code(code);
  let program = parser::parse(&mut lexer).expect("Failed to parse program");
  let scope = runner::scope::Scope::new();

  scope
    .as_ref()
    .borrow_mut()
    .set("print", runner::value::Value::NativeFunction(NativeFunction::new(|args| {
      let output = args
        .iter()
        .map(|arg| format!("{arg}"))
        .collect::<Vec<String>>()
        .join(" ");

      println!("{output}");
      return Value::String(output);
    })));

  let result = run(|| program.eval(&scope));

  println!("{result:?}");
}
