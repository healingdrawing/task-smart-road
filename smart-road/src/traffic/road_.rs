
use crate::draw::Textures;
use crate::traffic::way::Way;
use crate::traffic::autos::Autos;
use crate::traffic::auto::Auto;

use super::stack::LimitedStack;

pub struct Road<'a> {
  pub way: Way,
  pub autos: Autos,
  pub textures: &'a Textures,
}

impl<'a> Road<'a> {
  pub fn new(textures: &'a Textures) -> Self {
    Self {
      way: Way::new(),
      autos: Autos::new(),
      textures,
    }
  }

  pub fn update(&mut self) {
    self.manage_autos();
    self.animate_step();
    // println!("update"); //todo hide
  }



}
