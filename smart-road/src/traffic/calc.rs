use crate::traffic::way::Way;

pub struct Calc{
  way: Way,
}

impl Calc {
    pub fn new() -> Self {
        Self {
            way: Way::new(),
        }
    }
}