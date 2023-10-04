use super::{Road, stack::LimitedStack, Auto, way::To, road_::Free};

impl<'a> Road<'a> {
    
  pub fn way_point_is_free(&self, oldxy:&[u16;2], xy:&[u16;2], lane_autos:&LimitedStack<Auto>) -> bool {
    let free =
    lane_autos.iter().all(|auto| auto.to_x != xy[0].into() || auto.to_y != xy[1].into() );
    // println!("free: {}", free);
    // println!("free: {}, oldxy {:#?}, xy {:#?} , autos {:#?}", free, oldxy, xy, lane_autos); //todo hide
    free
  }

  /** all turn left prelast way points not targeted by autos
   * so time for autos to turn left and no turn, according to the road control
   */
  pub fn road_free(&self) -> bool {
    let mut free = true;
    let ways_prelast_points= [
      self.way.nw[self.way.nw.len()-2],
      self.way.se[self.way.se.len()-2],
      self.way.ws[self.way.ws.len()-2],
      self.way.en[self.way.en.len()-2],
    ];
    let autos = [
      &self.autos.nw, &self.autos.se, &self.autos.ws, &self.autos.en,
    ];
    for (way_prelast_point, autos) in ways_prelast_points.iter().zip(autos.iter()) {
      if !self.way_point_is_free( &[0,0], way_prelast_point, autos) {
        free = false;
        break;
      }
    }
    free
  }

  /** calculate the behavior of autos to allow them properly move through cross road */
  pub fn manage_autos(&mut self) {
    
    if self.road_free() {
      self.switch_free();
    }

    // self.manage_turn_right(To::N);
    // self.manage_turn_right(To::S);
    // self.manage_turn_right(To::W);
    // self.manage_turn_right(To::E);
    
    // self.manage_no_turn(To::N);
    // self.manage_no_turn(To::S);
    // self.manage_no_turn(To::W);
    // self.manage_no_turn(To::E);

    self.manage_turn_left(To::N);
    self.manage_turn_left(To::S);
    self.manage_turn_left(To::W);
    self.manage_turn_left(To::E);

    // self.manage_ne();
    // self.manage_sw();
    // self.manage_wn();
    // self.manage_es();
  }

  
}