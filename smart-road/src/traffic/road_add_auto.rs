use macroquad::texture::Texture2D;
use rand::Rng;

use super::{Road, way::To, Auto};

impl<'a> Road<'a> {
  //todo extend to random texture
  /** random auto texture */
  fn auto_texture(&self) -> &Texture2D { &self.textures.random_auto() }

  /** texture angle for initial auto position */
  fn texture_angle(&self, road:To) -> f32 {
    match road {
      To::N => self.way.nn[0][0] as f32,
      To::S => self.way.ss[0][0] as f32,
      To::W => self.way.ww[0][0] as f32,
      _ => self.way.ee[0][0] as f32,
    }
  }

  /** texture position for the new auto in the moment of appearing on the screen */
  fn auto_texture_position_xy(&self, road: To, lane_number: u8) -> &[f32; 2] {
    let result: Box<[f32; 2]> = match road {
        To::N => match lane_number {
            0 => Box::new([self.way.ne[1][0] as f32, self.way.ne[1][1] as f32]),
            1 => Box::new([self.way.nn[1][0] as f32, self.way.nn[1][1] as f32]),
            _ => Box::new([self.way.nw[1][0] as f32, self.way.nw[1][1] as f32]),
        },
        To::S => match lane_number {
            0 => Box::new([self.way.sw[1][0] as f32, self.way.sw[1][1] as f32]),
            1 => Box::new([self.way.ss[1][0] as f32, self.way.ss[1][1] as f32]),
            _ => Box::new([self.way.se[1][0] as f32, self.way.se[1][1] as f32]),
        },
        To::W => match lane_number {
            0 => Box::new([self.way.wn[1][0] as f32, self.way.wn[1][1] as f32]),
            1 => Box::new([self.way.ww[1][0] as f32, self.way.ww[1][1] as f32]),
            _ => Box::new([self.way.ws[1][0] as f32, self.way.ws[1][1] as f32]),
        },
        _ => match lane_number {
            0 => Box::new([self.way.es[1][0] as f32, self.way.es[1][1] as f32]),
            1 => Box::new([self.way.ee[1][0] as f32, self.way.ee[1][1] as f32]),
            _ => Box::new([self.way.en[1][0] as f32, self.way.en[1][1] as f32]),
        },
    };
    Box::leak(result)
}

  fn add_auto(&self, road:To, lane_number: u8) -> Auto {
    Auto::new(
      self.auto_texture_position_xy(road, lane_number),
      self.texture_angle(road),
      self.auto_texture(),
    )
  }

  /** input handler call this method, to try add auto from south to north */
  pub fn try_add_auto_north_directed(&mut self) {
    match self.random_free_lane(To::N) {
      0 => {self.autos.ne.push(self.add_auto(To::N, 0));},
      1 => {self.autos.nn.push(self.add_auto(To::N, 1));},
      2 => {self.autos.nw.push(self.add_auto(To::N, 2));},
      _ => {} // should not happen
    }
  }

  /** input handler call this method, to try add auto from north to south */
  pub fn try_add_auto_south_directed(&mut self) {
    match self.random_free_lane(To::S) {
      0 => {self.autos.sw.push(self.add_auto(To::S, 0));},
      1 => {self.autos.ss.push(self.add_auto(To::S, 1));},
      2 => {self.autos.se.push(self.add_auto(To::S, 2));},
      _ => {} // should not happen
    }
  }

  /** input handler call this method, to try add auto from east to west */
  pub fn try_add_auto_west_directed(&mut self) {
    match self.random_free_lane(To::W) {
      0 => {self.autos.wn.push(self.add_auto(To::W, 0));},
      1 => {self.autos.ww.push(self.add_auto(To::W, 1));},
      2 => {self.autos.ws.push(self.add_auto(To::W, 2));},
      _ => {} // should not happen
    }
  }

  /** input handler call this method, to try add auto from west to east */
  pub fn try_add_auto_east_directed(&mut self) {
    match self.random_free_lane(To::E) {
      0 => {self.autos.es.push(self.add_auto(To::E, 0));},
      1 => {self.autos.ee.push(self.add_auto(To::E, 1));},
      2 => {self.autos.en.push(self.add_auto(To::E, 2));},
      _ => {} // should not happen
    }
  }

  /** input handler call this method */
  pub fn try_add_auto_random_directed(&mut self) {
    match rand::thread_rng().gen_range(0..4) {
      0 => {self.try_add_auto_north_directed();},
      1 => {self.try_add_auto_south_directed();},
      2 => {self.try_add_auto_west_directed();},
      _ => {self.try_add_auto_east_directed();},
    }
  }

  /** when self.spam is true, fill the road as possible */
  pub fn spam_autos(&mut self) {
    self.try_add_auto_north_directed();
    self.try_add_auto_south_directed();
    self.try_add_auto_west_directed();
    self.try_add_auto_east_directed();
  }

  pub fn switch_spam(&mut self) {
    self.spam_autos = !self.spam_autos;
  }

  pub fn no_spam(&mut self) {
    self.spam_autos = false;
  }

  pub fn spam(&mut self) {
    self.spam_autos = true;
  }

}