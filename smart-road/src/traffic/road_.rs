
use std::time::Instant;

use crate::draw::Textures;
use crate::traffic::way::Way;
use crate::traffic::autos::Autos;
use crate::traffic::auto::Auto;

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
  /*statistic section */
  /** time stamp from the start of the simulation */
  pub simulation_init_time: Instant,
  /** time stamp of the moment when statistics was collected */
  pub simulation_end_time: Instant,
  /** how many autos passed the crossroad */
  pub autos_passed: u128,
  /** max speed achieved by auto on the full way */
  pub max_speed: f32,
  /** min speed achieved by auto on the full way */
  pub min_speed: f32,
  /** max time seconds of auto on the full way */
  pub max_time: f32,
  /** min time seconds of auto on the full way */
  pub min_time: f32,
  /** close calls, when autos pass each other with violation of safe distance.
   * Again bullshit from 01-edu. In example they demonstrated in task description
   * one car hit another, but finally they show 0 close calls in statistics.
   */
  pub close_calls: u128,
  /** show statistics on screen, after first Esc press */
  pub show_stats: bool,
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
      simulation_init_time: Instant::now(),
      simulation_end_time: Instant::now(),
      autos_passed: 0,
      max_speed: 0f32,
      min_speed: f32::MAX,
      max_time: 0f32,
      min_time: f32::MAX,
      close_calls: 0,
      show_stats: false,
    }
  }

  pub fn update(&mut self) {

    if self.spam {
      self.spam_autos();
    }

    self.manage_autos();
    self.animate_step();
  }

  /** update_stats update statistics according to task requirements */
  pub fn update_stats(&mut self, auto: Auto, sum_way: u16) {
    self.autos_passed += 1;
    let time = Instant::now().duration_since(auto.init_time).as_secs_f32();
    self.max_time = self.max_time.max(time);
    self.min_time = self.min_time.min(time);
    let speed = sum_way as f32 / time;
    self.max_speed = self.max_speed.max(speed);
    self.min_speed = self.min_speed.min(speed);
  }

  pub fn format_stats(&self) -> String {
    format!(
      "Simulation time: {:.2}\nAutos passed: {}\nMax speed: {:.2} m/s\nMin speed: {:.2} m/s\nMax time: {:.2} s\nMin time: {:.2} s\nClose calls: {} \nCollisions: {}\nPress Esc to exit",
      self.simulation_end_time.duration_since(self.simulation_init_time).as_secs_f32(),
      self.autos_passed,
      self.max_speed,
      if self.min_speed == f32::MAX {0f32} else {self.min_speed},
      self.max_time,
      if self.min_time == f32::MAX {0f32} else {self.min_time},
      self.close_calls, //idiots from 01-edu in task require close calls number, but in audit they also require collisions number
      self.close_calls,
    )
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
