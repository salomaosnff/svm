use std::{fs::File, time::Instant, io::Write, env, hash::Hash, collections::HashMap};

mod lang;

use crate::lang::{
  vm, assembler::{StackValue, DataType, Bytecode, opcodes::OpCode},
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

  vm.program = vec![
    OpCode::PUSH(StackValue::U8(1)),
    OpCode::PUSH(StackValue::U8(2)),
    OpCode::ADD(DataType::U8),
  ];
  // let file = File::open("example/code.svm").expect("Could not open file");
  // let mut output = File::create("example/code.bin").expect("Could not create file");

  // let bytecode = assembler::compile(file);

  // output.write_all(&bytecode.data).expect("Could not write to file");
  
  // vm.program = assembler::parse(bytecode.data);

  // // println!("{:?}", vm.program);

  run("Run", || vm.run());

  println!("{:?}", vm.stack);
}
