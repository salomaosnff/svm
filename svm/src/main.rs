use std::{fs::File, process::exit};

use svm_lang::Program;
use svm_vm::VM;

fn open_file(file: String) -> File {
  if let Ok(file) = File::open(file.clone()) {
    file
  } else {
    println!("Could not open file {:?}", file);
    exit(1);
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: svm <input>");
    return;
  }

  let mut vm = VM::new();

  vm.program = Program::from_file(open_file(args.get(1).unwrap().clone()));

  vm.run();

  println!("Program terminated with stack: {:?}", vm.stack.data);
}
