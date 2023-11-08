pub fn vm_panic(code: &str, error: &str) {
  panic!("VmError [{code}]: {error}");
  // std::process::exit(1);
}
