#[derive(Debug)]
pub struct Heap {
  pub data: Vec<u8>,
  pub size: usize,
}

impl Heap {
  pub fn new(size: usize) -> Self {
    return Heap {
      data: vec![0; size],
      size: size,
    };
  }

  pub fn load(address: usize) -> u8 {
    return self.data[address];
  }
}
