use std::ops::Range;

/// The counter for unit members.
/// the number must be between 2 and 5.
pub struct HeadCount {
  current: usize,
  max: usize,
}

impl HeadCount {
  /// # Panics
  /// when The passed is'nt between 2 and 5.
  pub fn new(max: usize) -> Self {
    match max {
      2..=5 => HeadCount { current: 0, max },
      _ => {
        panic!("the number of unit is not available.")
      }
    }
  }
  /// # Panics
  /// when to add member than 5.
  /// Usually, it is occurred by library(this).
  /// So it is bug, maybe.
  pub fn inc(&mut self) {
    self.current += 1;
    if self.current > self.max - 1 {
      panic!("over count!");
    }
  }
  /// A value needed to fill unit with chicks.
  pub fn remain_range(&self) -> Range<usize> {
    let end = self.max - self.current;
    0..end
  }
}
