use crate::traffic::way::Way;

pub struct Calc{
  pub way: Way,
}

impl Calc {
    pub fn new() -> Self {
        Self {
            way: Way::new(),
        }
    }

    
}