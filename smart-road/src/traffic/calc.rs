use rand::Rng;

use crate::draw::Textures;
use crate::traffic::way::{Way, To};
use crate::traffic::autos::Autos;
use crate::traffic::auto::Auto;

use super::stack::LimitedStack;

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
    // println!("update");
  }
  
  fn auto_targeted_to_point_does_not_move(&self, point:&[u16; 2], lane_autos:&LimitedStack<Auto>) -> bool {
    lane_autos.iter().any(|auto| auto.to_x == point[0] as f32 && auto.to_y == point[1] as f32 && auto.moving == false)
  }

  fn no_autos_targeted_to_point(&self, point:&[u16; 2], lane_autos:&LimitedStack<Auto>) -> bool {
    lane_autos.iter().all(|auto| auto.to_x != point[0] as f32 && auto.to_y != point[1] as f32)
  }

  fn no_autos_targeted_to_lane_first_point(
    &self,
    road_direction: To,
    lane_number: u8,
    lane_autos:&LimitedStack<Auto>,
  ) -> bool {
    // todo
    match road_direction {
      To::N => match lane_number {
        // take the first point of the lane, which is under index 1
        0 => { self.no_autos_targeted_to_point(&self.way.ne[1], lane_autos) }/*turn right*/ 
        1 => { self.no_autos_targeted_to_point(&self.way.nn[1], lane_autos) }/*no turn*/
        _ => { self.no_autos_targeted_to_point(&self.way.nw[1], lane_autos) }/*turn left*/
      }
      To::S => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.sw[1], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ss[1], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.se[1], lane_autos) }
      }
      To::W => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.wn[1], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ww[1], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.ws[1], lane_autos) }
      }
      _ => match lane_number { // To::E , east
        0 => { self.no_autos_targeted_to_point(&self.way.es[1], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ee[1], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.en[1], lane_autos) }
      }
    }
    
  }

  fn no_autos_targeted_to_lane_second_point(
    &self,
    road_direction: To,
    lane_number: u8,
    lane_autos:&LimitedStack<Auto>,
  ) -> bool {
    // todo
    match road_direction {
      To::N => match lane_number {
        // take the second point of the lane, which is under index 2
        0 => { self.no_autos_targeted_to_point(&self.way.ne[2], lane_autos) }/*turn right*/ 
        1 => { self.no_autos_targeted_to_point(&self.way.nn[2], lane_autos) }/*no turn*/
        _ => { self.no_autos_targeted_to_point(&self.way.nw[2], lane_autos) }/*turn left*/
      }
      To::S => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.sw[2], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ss[2], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.se[2], lane_autos) }
      }
      To::W => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.wn[2], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ww[2], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.ws[2], lane_autos) }
      }
      _ => match lane_number { // To::E , east
        0 => { self.no_autos_targeted_to_point(&self.way.es[2], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ee[2], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.en[2], lane_autos) }
      }
    }
    
  }

  fn auto_targeted_to_second_point_does_not_move(&self, road_direction: To, lane_number: u8, lane_autos:&LimitedStack<Auto>) -> bool {
    match road_direction {
      To::N => match lane_number {
        // take the second point of the lane, which is under index 2
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.ne[2], lane_autos) }/*turn right*/
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.nn[2], lane_autos) }/*no turn*/
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.nw[2], lane_autos) }/*turn left*/
      }
      To::S => match lane_number {
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.sw[2], lane_autos) }
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.ss[2], lane_autos) }
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.se[2], lane_autos) }
      }
      To::W => match lane_number {
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.wn[2], lane_autos) }
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.ww[2], lane_autos) }
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.ws[2], lane_autos) }
      }
      _ => match lane_number { // To::E , east
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.es[2], lane_autos) }
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.ee[2], lane_autos) }
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.en[2], lane_autos) }
      }
    }
  }

  fn lane_is_free(&self, road_direction:To, lane_number: u8) -> bool {
    // todo
    let lane_autos = match road_direction {
      To::N => match lane_number {
        0 => &self.autos.ne, // turn right
        1 => &self.autos.nn, // no turn
        _ => &self.autos.nw, // turn left
      }
      To::S => match lane_number {
        0 => &self.autos.sw,
        1 => &self.autos.ss,
        _ => &self.autos.se,
      }
      To::W => match lane_number {
        0 => &self.autos.wn,
        1 => &self.autos.ww,
        _ => &self.autos.ws,
      }
      _ => match lane_number {
        0 => &self.autos.es,
        1 => &self.autos.ee,
        _ => &self.autos.en,
      }
    };

    //todo implement mulitstatement, then remove comments
    /*no autos targeted to lane first point and no targeted to second point OR
    no autos targeted to lane first point and there is auto targeted to second point, but this auto does not move, so this auto already in destination point (on second point), so no collision in add moment
     */
    self.no_autos_targeted_to_lane_first_point(road_direction, lane_number, lane_autos)
    && (
      self.no_autos_targeted_to_lane_second_point(road_direction, lane_number, lane_autos)
      ||
      self.auto_targeted_to_second_point_does_not_move(road_direction, lane_number, lane_autos)
    ) //todo check it twice, does not look clear
  }

  

  /** generate random number from 0 to 2 to choose the lane */
  fn random_lane(&self, road:To) -> u8 {
    let mut rng = rand::thread_rng();
    let mut choices = vec![0, 1, 2];

    while !choices.is_empty() {
        let index = rng.gen_range(0..choices.len());
        let lane_number = choices.remove(index);

        if self.lane_is_free(road, lane_number) {
            return lane_number;
        }
    }

    // If all choices are taken, return the (3) as a fail value
    3
  }

  /** input handler call this method, to try add auto from south to north */
  pub fn try_add_auto_north_directed(&mut self) {
    // self.autos.add_auto_north();
    println!("try_add_auto_north_directed calc.rs");
    // generate random number from 0 to 2 to choose the lane
    match self.random_lane(To::N) {
      0 => println!("lane 0"),
      1 => println!("lane 1"),
      2 => println!("lane 2"),
      _ => println!("lane error"),
    }
  }

  /** input handler call this method, to try add auto from north to south */
  pub fn try_add_auto_south_directed(&mut self) {
    // self.autos.add_auto_south();
    println!("try_add_auto_south_directed calc.rs");
  }

  /** input handler call this method, to try add auto from west to east */
  pub fn try_add_auto_east_directed(&mut self) {
    // self.autos.add_auto_east();
    println!("try_add_auto_east_directed calc.rs");
  }

  /** input handler call this method, to try add auto from east to west */
  pub fn try_add_auto_west_directed(&mut self) {
    // self.autos.add_auto_west();
    println!("try_add_auto_west_directed calc.rs");
  }

  /** input handler call this method */
  pub fn try_add_auto_random_directed(&mut self) {
    // self.autos.add_auto_random();
    println!("try_add_auto_random_directed calc.rs");
  }
}
