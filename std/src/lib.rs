use svm_runtime::VM;
mod stdio;

pub fn stdio_plugin(vm: &mut VM) {
  vm.load_plugin(stdio::vm_plugin)
}
