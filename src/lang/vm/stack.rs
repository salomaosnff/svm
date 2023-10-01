use super::util::vm_panic;

const STACK_ITEM_SIZE: usize = 4;

#[derive(Debug)]
pub struct Stack {
  pub data: Vec<u8>,
  pub size: usize,
  pub sp: usize,
}

impl Stack {
  pub fn new(size: usize) -> Self {
    Self {
      data: vec![],
      size,
      sp: 0,
    }
  }

  pub fn peek(&self) -> [u8; STACK_ITEM_SIZE] {
    return self.get(self.sp - 1);
  }

  pub fn push(&mut self, value: [u8; STACK_ITEM_SIZE]) {
    if self.sp >= self.size {
      vm_panic("StackOverflow", "Maximum stack size exceeded!");
    }

    for i in 0..STACK_ITEM_SIZE {
      self.data.insert(self.sp * STACK_ITEM_SIZE + i, value[i]);
    }

    self.sp += 1;
  }

  pub fn pop(&mut self) -> [u8; STACK_ITEM_SIZE] {
    if self.sp == 0 {
      vm_panic("StackUnderflow", "Cannot pop from empty stack!");
    }

    let mut value = [0; STACK_ITEM_SIZE];

    self.sp -= 1;

    let index = self.sp * STACK_ITEM_SIZE;

    for i in 0..STACK_ITEM_SIZE {
      value[i] = self.data[index];
      self.data.remove(index);
    }

    return value;
  }

  pub fn set_sp(&mut self, offset: usize) {
    if offset > self.size {
      vm_panic("StackOverflow", "Stack pointer out of bounds!");
    }

    self.sp = offset;
  }

  pub fn get_sp(&self) -> usize {
    return self.sp;
  }

  pub fn get(&self, offset: usize) -> [u8; STACK_ITEM_SIZE] {
    if offset >= self.size {
      vm_panic("StackOverflow", "Stack pointer out of bounds!");
    }

    let mut value = [0; STACK_ITEM_SIZE];

    for i in 0..STACK_ITEM_SIZE {
      value[i] = self.data[offset * STACK_ITEM_SIZE + i];
    }

    return value;
  }

  pub fn set(&mut self, offset: usize, value: [u8; STACK_ITEM_SIZE]) {
    if offset >= self.size {
      vm_panic("StackOverflow", "Stack pointer out of bounds!");
    }

    for i in 0..STACK_ITEM_SIZE {
      self.data[offset * STACK_ITEM_SIZE + i] = value[i];
    }
  }

  pub fn get_size(&self) -> usize {
    return self.size;
  }

  pub fn dump(&self) -> &Vec<u8> {
    return &self.data;
  }
}
