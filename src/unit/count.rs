use std::ops::Range;

pub struct HeadCount {
  current: usize,
  max: usize,
}

impl HeadCount {
  pub fn new(max: usize) -> Self {
    HeadCount { current: 0, max }
  }
  pub fn inc(&mut self) {
    self.current += 1;
    if self.current > self.max - 1 {
      panic!("over count!");
    }
  }
  pub fn remain_range(&self) -> Range<usize> {
    let end = self.max - self.current;
    0..end
  }
}
