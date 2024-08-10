#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Id(u64);

#[derive(Default)]
pub struct IdProvider {
  last: u64,
}

impl IdProvider {
  pub fn next(&mut self) -> Id {
    self.last = self.last.checked_add(1).unwrap();
    Id(self.last)
  }
}
