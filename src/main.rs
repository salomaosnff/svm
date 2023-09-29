use std::{fs::File, time::Instant};

mod lang;

use crate::lang::opcode::{assemble, compile_file};

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

  let mut vm = lang::vm::VM::new();

  vm.program = compile_file(File::open("code.lang").unwrap());

  println!("{:#?}", assemble(&vm.program));

  run(|| vm.run());
}
