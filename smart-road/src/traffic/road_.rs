
use crate::draw::Textures;
use crate::traffic::way::Way;
use crate::traffic::autos::Autos;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Free{
  /** vertical */ VERTICAL,
  /** locked after vertical */ AFTER_VERTICAL,
  /** horizontal */ HORIZONTAL,
  /** locked after horizontal */ AFTER_HORIZONTAL,
}

pub struct Road<'a> {
  pub way: Way,
  pub autos: Autos,
  pub textures: &'a Textures,
  pub spam: bool,
  pub free: Free,
  /*turn left section to allow one car per turn */
  pub nw_free: bool,
  pub se_free: bool,
  pub ws_free: bool,
  pub en_free: bool,
  /*no turn section to allow one car per turn */
  pub nn_free: bool,
  pub ss_free: bool,
  pub ww_free: bool,
  pub ee_free: bool,

}

impl<'a> Road<'a> {
  pub fn new(textures: &'a Textures) -> Self {
    Self {
      way: Way::new(),
      autos: Autos::new(),
      textures,
      spam: false,
      free: Free::VERTICAL,
      nw_free: true,
      se_free: true,
      ws_free: false,
      en_free: false,
      nn_free: true,
      ss_free: true,
      ww_free: false,
      ee_free: false,
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

  pub fn reset_free(&mut self) {
    self.nw_free = false;
    self.se_free = false;
    self.ws_free = false;
    self.en_free = false;
    self.nn_free = false;
    self.ss_free = false;
    self.ww_free = false;
    self.ee_free = false;
  }

  fn switch_to_vertical(&mut self) {
    // println!("switch_to_vertical"); //todo hide
    self.nw_free = true;
    self.se_free = true;
    self.ws_free = false;
    self.en_free = false;
    self.nn_free = true;
    self.ss_free = true;
    self.ww_free = false;
    self.ee_free = false;
  }

  fn switch_to_horizontal(&mut self) {
    // println!("switch_to_horizontal"); //todo hide
    self.nw_free = false;
    self.se_free = false;
    self.ws_free = true;
    self.en_free = true;
    self.nn_free = false;
    self.ss_free = false;
    self.ww_free = true;
    self.ee_free = true;
  }

  pub fn switch_free(&mut self) {
    match self.free {
      Free::VERTICAL => self.free = Free::AFTER_VERTICAL,
      Free::AFTER_VERTICAL => {
        self.free = Free::HORIZONTAL;
        self.switch_to_horizontal();
      },
      Free::HORIZONTAL => self.free = Free::AFTER_HORIZONTAL,
      Free::AFTER_HORIZONTAL => {
        self.free = Free::VERTICAL;
        self.switch_to_vertical();
      },
    }
  }

}
