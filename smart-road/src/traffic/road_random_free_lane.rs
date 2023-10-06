use rand::Rng;

use super::{Road, stack::LimitedStack, Auto, way::To};

impl<'a> Road<'a> {

  fn auto_targeted_to_point_does_not_move(&self, point:&[u16; 2], lane_autos:&LimitedStack<Auto>) -> bool {
    lane_autos.iter().any(|auto| auto.to_x == point[0] as f32 && auto.to_y == point[1] as f32 && auto.moving == false)
  }

  fn no_autos_targeted_to_point(&self, point:&[u16; 2], lane_autos:&LimitedStack<Auto>) -> bool {
    lane_autos.iter().all(|auto| auto.to_x != point[0] as f32 || auto.to_y != point[1] as f32)
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

  /** lane is free to add new auto to first point of the lane */
  fn lane_is_free(&self, road_direction:To, lane_number: u8) -> bool {
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

    let first_free = self.no_autos_targeted_to_lane_first_point(road_direction, lane_number, lane_autos);
    let second_free = self.no_autos_targeted_to_lane_second_point(road_direction, lane_number, lane_autos);
    let second_done = self.auto_targeted_to_second_point_does_not_move(road_direction, lane_number, lane_autos);

    first_free && (second_free || second_done)

  }

  

  /** random_free_lane choose randomly the free lane: 
   * 0 - turn right
   * 1 - no turn
   * 2 - turn left
   
   of the road way or

   returns 3 as fail value */
  pub fn random_free_lane(&self, road:To) -> u8 {
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

}