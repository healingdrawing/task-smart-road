
use crate::draw::Textures;
use crate::traffic::way::Way;
use crate::traffic::autos::Autos;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Free{
  /** vertical */ VERTICAL,
  /** locked after vertical step */ LOCK_AFTER_VERTICAL,
  /** horizontal */ HORIZONTAL,
  /** locked after horizontal step */ LOCK_AFTER_HORIZONTAL,
}

pub struct Road<'a> {
  pub way: Way,
  pub autos: Autos,
  pub textures: &'a Textures,
  pub spam: bool,
  pub free: Free,
}

impl<'a> Road<'a> {
  pub fn new(textures: &'a Textures) -> Self {
    Self {
      way: Way::new(),
      autos: Autos::new(),
      textures,
      spam: false,
      free: Free::VERTICAL,
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

  pub fn switch_free(&mut self) {
    match self.free {
      Free::VERTICAL => self.free = Free::LOCK_AFTER_VERTICAL,
      Free::LOCK_AFTER_VERTICAL => self.free = Free::HORIZONTAL,
      Free::HORIZONTAL => self.free = Free::LOCK_AFTER_HORIZONTAL,
      Free::LOCK_AFTER_HORIZONTAL => self.free = Free::VERTICAL,
    }
  }

}
