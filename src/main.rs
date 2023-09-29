use std::{time::Instant, fs::File};

mod lang;

use crate::lang::opcode::{self, disassemble, assemble, Assembler, compile_file};

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

  run(|| vm.run());
}
