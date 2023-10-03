// rust is awful
use super::{Road,way::To};

impl<'a> Road<'a> {
  pub fn manage_turn_right(&mut self, to: To) {
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let autos_clone = match to{
      To::N => self.autos.ne.clone(),
      To::S => self.autos.sw.clone(),
      To::W => self.autos.wn.clone(),
      To::E => self.autos.es.clone(),
    };

    autos_clone.iter().for_each(|auto| {
      if !auto.moving {
        let way = match to {
          To::N => &self.way.ne,
          To::S => &self.way.sw,
          To::W => &self.way.wn,
          To::E => &self.way.es,
        };

        let mut way_iter = match to {
          To::N => self.way.ne.iter().skip(1),
          To::S => self.way.sw.iter().skip(1),
          To::W => self.way.wn.iter().skip(1),
          To::E => self.way.es.iter().skip(1),
        };

        let target = way_iter.position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
        let way_len = way.len();
        if target == way_len - 2 {// the hell, it is so muddy -2 because target calculated with skip(1). Perhaps it was not the best idea to use first index as angle of texture rotation
          // end of the way achieved
          pop_first = true;
        } else if target == way_len - 3 { // it must be index of prelast way point, so turbo x2 the auto speed to keep distance between autos. In same time we demonstrate the third auto speed (0, normal, x2)
          let auto_number = autos_clone.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();

          let mut autos_iter = match to {
            To::N => self.autos.ne.iter_mut(),
            To::S => self.autos.sw.iter_mut(),
            To::W => self.autos.wn.iter_mut(),
            To::E => self.autos.es.iter_mut(),
          };

          autos_iter.nth(auto_number).unwrap().animate_to(
            &way[target + 2].map(|x| x.into()),
            2.0,
          );
        } else if self.next_way_point_is_free(
          &way[target+1],
          &way[target + 2],
          match to{
            To::N => &self.autos.ne,
            To::S => &self.autos.sw,
            To::W => &self.autos.wn,
            To::E => &self.autos.es,
          },
        ) {
          // next point is free, so move to it
          let auto_number = autos_clone.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();

          let mut autos_iter = match to {
            To::N => self.autos.ne.iter_mut(),
            To::S => self.autos.sw.iter_mut(),
            To::W => self.autos.wn.iter_mut(),
            To::E => self.autos.es.iter_mut(),
          };

          autos_iter.nth(auto_number).unwrap().animate_to(
            &way[target + 2].map(|x| x.into()),
            1.0,
          );  
        } 
      }
    });

    if pop_first {
      
      if to == To::N { self.autos.ne.pop();}
      else if to == To::S { self.autos.sw.pop();}
      else if to == To::W { self.autos.wn.pop();}
      else if to == To::E { self.autos.es.pop();}
      
    }
  }

}