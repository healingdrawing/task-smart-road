
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
  
  fn next_way_point_is_free(&self, oldxy:&[u16;2], xy:&[u16;2], lane_autos:&LimitedStack<Auto>) -> bool {
    let free =
    lane_autos.iter().all(|auto| auto.to_x != xy[0].into() || auto.to_y != xy[1].into() );
    // println!("free: {}", free);
    // println!("free: {}, oldxy {:#?}, xy {:#?} , autos {:#?}", free, oldxy, xy, lane_autos); //todo hide
    free
  }

  /** calculate the behavior of autos to allow them properly move through cross road */
  fn manage_autos(&mut self) {
    //todo
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let mut autos_ne_clone = self.autos.ne.clone();  // Clone self.autos.ne

    autos_ne_clone.iter_mut().for_each(|auto| {
      if !auto.moving {
          let target = self.way.ne.iter().skip(1).position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
          if target == self.way.ne.len() - 2 {// the hell, it is so muddy -2 because target calculated with skip(1). Perhaps it was not the best idea to use first index as angle of texture rotation
              // end of the way achieved
              pop_first = true;
          } else if target == self.way.ne.len() - 3 { // it must be prelast way point
            let auto_number = self.autos.ne.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();
              self.autos.ne.iter_mut().nth(auto_number).unwrap().animate_to(
                &self.way.ne[target + 2].map(|x| x.into()),
                 2.0,
              );
          }else if self.next_way_point_is_free(
              &self.way.ne[target+1],
              &self.way.ne[target + 2],
              &self.autos.ne,
          ) {
              // next point is free, so move to it
              let auto_number = self.autos.ne.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();
              self.autos.ne.iter_mut().nth(auto_number).unwrap().animate_to(
                &self.way.ne[target + 2].map(|x| x.into()),
                 1.0,
              );  
          } 
      }
    });

    if pop_first {
      self.autos.ne.pop();
    }
  }

  


}
