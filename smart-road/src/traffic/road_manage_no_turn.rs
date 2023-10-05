// rust is awful
use super::{Road,way::To, road_::Free};

impl<'a> Road<'a> {
  pub fn manage_no_turn(&mut self, to: To) {
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let autos_clone = match to{
      To::N => self.autos.nn.clone(),
      To::S => self.autos.ss.clone(),
      To::W => self.autos.ww.clone(),
      To::E => self.autos.ee.clone(),
    };

    autos_clone.iter().for_each(|auto| {
      if !auto.moving {
        let way = match to {
          To::N => &self.way.nn,
          To::S => &self.way.ss,
          To::W => &self.way.ww,
          To::E => &self.way.ee,
        };

        let mut way_iter = match to {
          To::N => self.way.nn.iter().skip(1),
          To::S => self.way.ss.iter().skip(1),
          To::W => self.way.ww.iter().skip(1),
          To::E => self.way.ee.iter().skip(1),
        };

        let target = way_iter.position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
        let way_len = way.len();
        if target == way_len - 2 {// the hell, it is so muddy -2 because target calculated with skip(1). Perhaps it was not the best idea to use first index as angle of texture rotation
          // end of the way achieved
          pop_first = true;
        } else if target == way_len - 3 { // it must be index of prelast way point, so turbo x2 the auto speed, to keep distance between autos. In same time we demonstrate the third auto speed (0, normal, x2)
          let auto_number = autos_clone.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();

          let mut autos_iter = match to {
            To::N => self.autos.nn.iter_mut(),
            To::S => self.autos.ss.iter_mut(),
            To::W => self.autos.ww.iter_mut(),
            To::E => self.autos.ee.iter_mut(),
          };

          let one_auto_allowed = match to {
            To::N => self.nn_free,
            To::S => self.ss_free,
            To::W => self.ww_free,
            To::E => self.ee_free,
          };
          
          if (one_auto_allowed) &&
            (
            ((to == To::N || to == To::S) && self.free == Free::VERTICAL)
            || ((to == To::W || to == To::E) && self.free == Free::HORIZONTAL)
            )
          {
            match to {
              To::N => self.nn_free = false,
              To::S => self.ss_free = false,
              To::W => self.ww_free = false,
              To::E => self.ee_free = false,
            }
            
            autos_iter.nth(auto_number).unwrap().animate_to(
              &way[target + 2].map(|x| x.into()),
              2.0,
            );  
          }

          // autos_iter.nth(auto_number).unwrap().animate_to(
          //   &way[target + 2].map(|x| x.into()),
          //   2.0,
          // );
        } else if self.way_point_is_free(
          &way[target + 2],
          match to{
            To::N => &self.autos.nn,
            To::S => &self.autos.ss,
            To::W => &self.autos.ww,
            To::E => &self.autos.ee,
          },
        ) {
          // next point is free, so move to it
          let auto_number = autos_clone.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();

          let mut autos_iter = match to {
            To::N => self.autos.nn.iter_mut(),
            To::S => self.autos.ss.iter_mut(),
            To::W => self.autos.ww.iter_mut(),
            To::E => self.autos.ee.iter_mut(),
          };

          autos_iter.nth(auto_number).unwrap().animate_to(
            &way[target + 2].map(|x| x.into()),
            1.0,
          );  
        } 
      }
    });

    if pop_first {
      
      let auto_passed_crossroad = match to{
        To::N => self.autos.nn.pop(),
        To::S => self.autos.ss.pop(),
        To::W => self.autos.ww.pop(),
        To::E => self.autos.ee.pop(),
      }.unwrap();

      let sum_way = match to{
        To::N => self.way.sum_nn,
        To::S => self.way.sum_ss,
        To::W => self.way.sum_ww,
        To::E => self.way.sum_ee,
      };

      self.update_stats(auto_passed_crossroad, sum_way);
      
    }
  }

}