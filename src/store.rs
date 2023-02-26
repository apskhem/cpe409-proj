use std::fmt::Debug;
use num_traits::Num;

pub trait Numeric:
  Num +
  Debug +
  Copy {}

impl<T:
  Num +
  Debug +
  Copy> Numeric for T {}

pub struct Store<T: Numeric> {
  arr: Vec<T>
}

impl<T: Numeric> Store<T> {
  pub fn new(size: usize) -> Self {
    Self { arr: vec![T::zero(); size] }
  }

  pub fn at(&self, idx: usize) -> Result<T, String> {
    self.arr
      .get(idx)
      .ok_or_else(|| "Index out of bounds".into())
      .cloned()
  }

  pub fn write(self, idx: usize, value: T) -> Self {
    let (left, right) = self.arr.split_at(idx);
    let mem = vec![left, &[value], &right[1..]].concat();

    Self { arr: mem }
  }

  pub fn write_all(self, idx: usize, values: &[T]) -> Self {
    let (left, right) = self.arr.split_at(idx);
    let mem = vec![left, values, &right[values.len()..]].concat();

    Self { arr: mem }
  }
}