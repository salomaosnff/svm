use svm_lang::Value;
use svm_runtime::VM;

fn print(vm: &mut VM) -> Option<Value> {
  let item_type = vm.stack.pop_type();
  let value = vm.stack.pop_value(&item_type);

  match value {
    Value::Bool(value) => print!("{}", value),
    Value::Bytes(value) => print!("Bytes {}", String::from_utf8(value).unwrap()),
    Value::F32(value) => print!("{}", value),
    Value::F64(value) => print!("{}", value),
    Value::I16(value) => print!("{}", value),
    Value::I32(value) => print!("{}", value),
    Value::I64(value) => print!("{}", value),
    Value::I8(value) => print!("{}", value),
    Value::Isize(value) => print!("{}", value),
    Value::U16(value) => print!("{}", value),
    Value::U32(value) => print!("{}", value),
    Value::U64(value) => print!("{}", value),
    Value::U8(value) => print!("{}", value),
    Value::Usize(value) => print!("{}", value),
    Value::String(value) => print!("{}", value),
  }

  None
}

pub fn vm_plugin(vm: &mut VM) {
  vm.register_extern(0, &print);
}
