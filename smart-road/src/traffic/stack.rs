use std::collections::VecDeque;

pub struct LimitedStack<T> {
  items: VecDeque<T>,
  capacity: usize,
}

impl<T> LimitedStack<T> {
  pub fn new(capacity: usize) -> Self {
      LimitedStack {
          items: VecDeque::with_capacity(capacity),
          capacity,
      }
  }

  pub fn push(&mut self, item: T) -> bool {
      if self.items.len() >= self.capacity {
          return false; // Return false if the stack is full.
      }
      self.items.push_back(item);
      true // Return true if pushed successfully.
  }

  pub fn pop(&mut self) -> Option<T> {
      self.items.pop_front()
  }

  pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<T> {
      self.items.iter_mut()
  }

  pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
    self.items.iter()
  }

  pub fn len(&self) -> usize {
      self.items.len()
  }
}
