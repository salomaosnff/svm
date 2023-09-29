pub fn vm_panic(code: &str, error: &str) {
  println!("VmError [{code}]: {error}");
  std::process::exit(1);
}
