use crate::draw::Textures;
use crate::traffic::way::Way;
use crate::traffic::autos::Autos;

pub struct Calc<'a> {
  pub way: Way,
  pub autos: Autos,
  pub textures: &'a Textures,
}

impl<'a> Calc<'a> {
  pub fn new(textures: &'a Textures) -> Self {
    Self {
      way: Way::new(),
      autos: Autos::new(),
      textures,
    }
  }

  pub fn update(&mut self) {
    self.autos.ss.iter_mut().for_each(|auto| auto.animate_step());
    println!("update");
  }

  /** input handler call this method */
  pub fn try_add_auto_random(&mut self) {
    // self.autos.add_auto_random();
    println!("add_auto_random calc.rs");
  }
}
