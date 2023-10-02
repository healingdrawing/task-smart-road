use super::Road;

impl<'a> Road<'a> {
  /** manage south west (turn right) lane of south directed way of the road */
  pub fn manage_sw(&mut self) {
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let mut autos_clone = self.autos.sw.clone();

    autos_clone.iter_mut().for_each(|auto| {
      if !auto.moving {
        let target = self.way.sw.iter().skip(1).position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
        if target == self.way.sw.len() - 2 {// the hell, it is so muddy -2 because target calculated with skip(1). Perhaps it was not the best idea to use first index as angle of texture rotation
          // end of the way achieved
          pop_first = true;
        } else if target == self.way.sw.len() - 3 { // it must be index of prelast way point, so turbo x2 the auto speed to keep distance between autos. In same time we demonstrate the third auto speed (0, normal, x2)
          let auto_number = self.autos.sw.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();
            self.autos.sw.iter_mut().nth(auto_number).unwrap().animate_to(
              &self.way.sw[target + 2].map(|x| x.into()),
              2.0,
            );
        } else if self.next_way_point_is_free(
          &self.way.sw[target+1],
          &self.way.sw[target + 2],
          &self.autos.sw,
        ) {
          // next point is free, so move to it
          let auto_number = self.autos.sw.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();
          self.autos.sw.iter_mut().nth(auto_number).unwrap().animate_to(
            &self.way.sw[target + 2].map(|x| x.into()),
            1.0,
          );  
        } 
      }
    });

    if pop_first {
      self.autos.sw.pop();
    }
  }
  
}