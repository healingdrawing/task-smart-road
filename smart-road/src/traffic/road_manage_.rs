use super::{Road, stack::LimitedStack, Auto};

impl<'a> Road<'a> {
    
  pub fn next_way_point_is_free(&self, oldxy:&[u16;2], xy:&[u16;2], lane_autos:&LimitedStack<Auto>) -> bool {
    let free =
    lane_autos.iter().all(|auto| auto.to_x != xy[0].into() || auto.to_y != xy[1].into() );
    // println!("free: {}", free);
    // println!("free: {}, oldxy {:#?}, xy {:#?} , autos {:#?}", free, oldxy, xy, lane_autos); //todo hide
    free
  }

  /** calculate the behavior of autos to allow them properly move through cross road */
  pub fn manage_autos(&mut self) {
    self.manage_ne();
    self.manage_sw();
    self.manage_wn();
    self.manage_es();
  }

  
}