// rust is awful
use super::{Road,way::To, road_::Free};

impl<'a> Road<'a> {
  pub fn manage_turn_left(&mut self, to: To) {
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let autos_clone = match to{
      To::N => self.autos.nw.clone(),
      To::S => self.autos.se.clone(),
      To::W => self.autos.ws.clone(),
      To::E => self.autos.en.clone(),
    };

    autos_clone.iter().for_each(|auto| {
      if !auto.moving {
        let way = match to {
          To::N => &self.way.nw,
          To::S => &self.way.se,
          To::W => &self.way.ws,
          To::E => &self.way.en,
        };

        let mut way_iter = match to {
          To::N => self.way.nw.iter().skip(1),
          To::S => self.way.se.iter().skip(1),
          To::W => self.way.ws.iter().skip(1),
          To::E => self.way.en.iter().skip(1),
        };

        let target = way_iter.position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
        let way_len = way.len();
        if target == way_len - 2 {// the hell, it is so muddy -2 because target calculated with skip(1). Perhaps it was not the best idea to use first index as angle of texture rotation
          // end of the way achieved
          pop_first = true;
        } else if target == way_len - 3 { // it must be index of prelast way point, so turbo x2 the auto speed to keep distance between autos. In same time we demonstrate the third auto speed (0, normal, x2)
          let auto_number = autos_clone.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();

          let mut autos_iter = match to {
            To::N => self.autos.nw.iter_mut(),
            To::S => self.autos.se.iter_mut(),
            To::W => self.autos.ws.iter_mut(),
            To::E => self.autos.en.iter_mut(),
          };

          autos_iter.nth(auto_number).unwrap().animate_to(
            &way[target + 2].map(|x| x.into()),
            2.0,
          );
        } else if self.way_point_is_free(
          &way[target + 2],
          match to{
            To::N => &self.autos.nw,
            To::S => &self.autos.se,
            To::W => &self.autos.ws,
            To::E => &self.autos.en,
          },
        ) {
          // next point is free, so move to it
          let auto_number = autos_clone.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();

          let mut autos_iter = match to {
            To::N => self.autos.nw.iter_mut(),
            To::S => self.autos.se.iter_mut(),
            To::W => self.autos.ws.iter_mut(),
            To::E => self.autos.en.iter_mut(),
          };

          if target != 2{ 
            // common case, not a point with index 3 minus first item with angle
            autos_iter.nth(auto_number).unwrap().animate_to(
              &way[target + 2].map(|x| x.into()),
              1.0,
            );  
          } else { // point controlled by road control
            let one_auto_allowed = match to {
              To::N => self.nw_free,
              To::S => self.se_free,
              To::W => self.ws_free,
              To::E => self.en_free,
            };
            
            if (one_auto_allowed) &&
              (
              ((to == To::N || to == To::S) && self.free == Free::VERTICAL)
              || ((to == To::W || to == To::E) && self.free == Free::HORIZONTAL)
              )
            {
              match to {
                To::N => self.nw_free = false,
                To::S => self.se_free = false,
                To::W => self.ws_free = false,
                To::E => self.en_free = false,
              }
              
              autos_iter.nth(auto_number).unwrap().animate_to(
                &way[target + 2].map(|x| x.into()),
                1.0,
              );  
            }
          }
        } 
      }
    });

    if pop_first {
      
      if to == To::N { self.autos.nw.pop();}
      else if to == To::S { self.autos.se.pop();}
      else if to == To::W { self.autos.ws.pop();}
      else if to == To::E { self.autos.en.pop();}
      
    }
  }

}