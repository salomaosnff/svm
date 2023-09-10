pub struct Consumer<T: PartialEq> {
  reader: Box<dyn Iterator<Item = T> + 'static>,
  queue: Vec<T>,
}

impl<T: PartialEq> Consumer<T> {
  pub fn new(reader: impl Iterator<Item = T> + 'static) -> Self {
    let mut instance = Self {
      reader: Box::new(reader),
      queue: Vec::new(),
    };

    instance.read_to_queue();

    return instance;
  }

  fn read_to_queue(&mut self) {
    match self.reader.next() {
      Some(ch) => self.queue.push(ch),
      _ => (),
    }
  }

  pub fn lookahead(&self) -> Option<&T> {
    return self.queue.get(0);
  }

  pub fn lookahead_is<U: FnOnce(&T) -> bool>(&mut self, condition: U) -> bool {
    match self.lookahead() {
      Some(ch) if condition(ch) => true,
      _ => false,
    }
  }

  pub fn consume(&mut self) -> Option<T> {
    self.read_to_queue();

    if self.queue.len() > 0 {
      return Some(self.queue.remove(0));
    }

    return None;
  }

  pub fn consume_equal(&mut self, expected: T) -> Option<T> {
    return self.consume_if(|c| *c == expected);
  }

  pub fn consume_if<U: FnOnce(&T) -> bool>(&mut self, condition: U) -> Option<T> {
    if self.lookahead_is(condition) {
      return self.consume();
    }

    return None;
  }

  pub fn consume_while<M: Fn(&T) -> bool>(&mut self, matcher: M) -> Vec<T> {
    let mut result = Vec::new();

    loop {
      match self.consume_if(&matcher) {
        Some(c) if matcher(&c) => {
          result.push(c);
        }
        _ => return result,
      };
    }
  }
}

impl<T: PartialEq> Iterator for Consumer<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    return self.consume();
  }
}
