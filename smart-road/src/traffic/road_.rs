
use crate::draw::Textures;
use crate::traffic::way::Way;
use crate::traffic::autos::Autos;


pub struct Road<'a> {
  pub way: Way,
  pub autos: Autos,
  pub textures: &'a Textures,
  pub spam: bool,
}

impl<'a> Road<'a> {
  pub fn new(textures: &'a Textures) -> Self {
    Self {
      way: Way::new(),
      autos: Autos::new(),
      textures,
      spam: false,
    }
  }

  pub fn update(&mut self) {

    if self.spam {
      self.spam_autos();
    }

    self.manage_autos();
    self.animate_step();
    // println!("update"); //todo hide
  }



}
