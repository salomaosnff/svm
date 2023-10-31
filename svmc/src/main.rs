use std::{env, fs::File, io::Write, process::exit};

use svm_asm::compile;

pub fn open_file(file: String) -> File {
  if let Ok(file) = File::open(file.clone()) {
    file
  } else {
    println!("Could not open file {:?}", file);
    exit(1);
  }
}

pub fn create_file(file: String) -> File {
  if let Ok(file) = File::create(file.clone()) {
    file
  } else {
    println!("Could not create file {:?}", file);
    exit(1);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage: svmc <input> [output]");
    return;
  }

  let input = args.get(1).unwrap().clone();
  let output = args
    .get(2)
    .map(|s| s.clone())
    .unwrap_or(format!("{}.bin", input));

  let file = open_file(input);
  let mut output = create_file(output);

  output
    .write_all(compile(file).to_bytes().as_slice())
    .expect("Could not write to file");
}
