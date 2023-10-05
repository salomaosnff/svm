use std::{time::Instant, env, fs::File, io::Write};

mod lang;

use crate::lang::{
  vm, assembler::{StackValue, DataType, self, Bytecode},
};

fn run<U, T: FnOnce() -> U>(name: &str, f: T) -> U {
  let now = Instant::now();
  let result = f();
  let elapsed = now.elapsed();

  println!("{name}: {:?}", elapsed);

  return result;
}

fn main() {
  env::set_var("RUST_BACKTRACE", "1");
  // Clear the terminal
  println!("\x1bc");

  let mut vm = vm::VM::new();

  let file = File::open("example/code.svm").expect("Could not open file");
  let mut output = File::create("example/code.bin").expect("Could not create file");

  let bytecode = assembler::compile(file);
  
  output.write_all(&bytecode.data).expect("Could not write to file");
  
  vm.program = assembler::parse(bytecode.data);

  run("Run", || vm.run());

  println!("{:?}", vm.stack);
}
