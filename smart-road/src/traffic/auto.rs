use std::collections::VecDeque;

pub struct Auto{
  pub start_x:u16, // start point of the way(between two way points)
  pub start_y:u16,
  pub target_x:u16, // target point of the way(between two way points)
  pub target_y:u16,
  pub x:u16,
  pub y:u16,
  pub speed:u16,
  pub sum_distance:u16, // used to compare the distance car moved with the distance of the way
  pub moving:bool, // is the car moving now. when the move completed, the moving will be false. so new move can be started or car can be removed, if the move was the last in the way
}

pub struct LimitedStack<T> {
  items: VecDeque<T>,
  capacity: usize,
}

impl<T> LimitedStack<T> {
  fn new(capacity: usize) -> Self {
      LimitedStack {
          items: VecDeque::with_capacity(capacity),
          capacity,
      }
  }

  fn push(&mut self, item: T) -> bool {
      if self.items.len() >= self.capacity {
          return false; // Return false if the stack is full.
      }
      self.items.push_back(item);
      true // Return true if pushed successfully.
  }

  fn pop(&mut self) -> Option<T> {
      self.items.pop_front()
  }

  fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<T> {
      self.items.iter_mut()
  }

  fn iter(&self) -> std::collections::vec_deque::Iter<T> {
    self.items.iter()
  }

  fn len(&self) -> usize {
      self.items.len()
  }
}

/**the limited stack of autos for each way. Plan to use it for iteration */
pub struct Autos{
  pub ss:LimitedStack<Auto>,
  pub nn:LimitedStack<Auto>,
  pub ww:LimitedStack<Auto>,
  pub ee:LimitedStack<Auto>,

  pub se:LimitedStack<Auto>,
  pub nw:LimitedStack<Auto>,
  pub ws:LimitedStack<Auto>,
  pub en:LimitedStack<Auto>,

  pub sw:LimitedStack<Auto>,
  pub ne:LimitedStack<Auto>,
  pub es:LimitedStack<Auto>,
  pub wn:LimitedStack<Auto>,
}

/** capacity for each stack is length of the way minus one,
 * because the first element of the way is not a point
 * but rotation angles for texture*/
impl Autos{
  pub fn new()->Autos{
    Autos{
      ss:LimitedStack::new(4),
      nn:LimitedStack::new(4),
      ww:LimitedStack::new(4),
      ee:LimitedStack::new(4),

      se:LimitedStack::new(5),
      nw:LimitedStack::new(5),
      ws:LimitedStack::new(5),
      en:LimitedStack::new(5),

      sw:LimitedStack::new(5),
      ne:LimitedStack::new(5),
      es:LimitedStack::new(5),
      wn:LimitedStack::new(5),
    }
  }
}