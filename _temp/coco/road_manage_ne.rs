use super::Road;

impl<'a> Road<'a> {
  /** manage north east (turn right) lane of north directed way of the road */
  pub fn manage_ne(&mut self) {
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let autos_clone = self.autos.ne.clone();

    autos_clone.iter().for_each(|auto| {
      if !auto.moving {
        let target = self.way.ne.iter().skip(1).position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
        if target == self.way.ne.len() - 2 {// the hell, it is so muddy -2 because target calculated with skip(1). Perhaps it was not the best idea to use first index as angle of texture rotation
          // end of the way achieved
          pop_first = true;
        } else if target == self.way.ne.len() - 3 { // it must be index of prelast way point, so turbo x2 the auto speed to keep distance between autos. In same time we demonstrate the third auto speed (0, normal, x2)
          let auto_number = self.autos.ne.iter().position(|x| x.to_x == auto.to_x && x.to_y == auto.to_y).unwrap();
            self.autos.ne.iter_mut().nth(auto_number).unwrap().animate_to(
              &self.way.ne[target + 2].map(|x| x.into()),
              2.0,
            );
        } else if self.way_point_is_free(
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