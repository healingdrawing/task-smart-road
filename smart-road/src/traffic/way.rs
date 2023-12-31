use crate::config::{CS,CM};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum To {
    /** North*/ N,
    /** East */ E,
    /** South*/ S,
    /** West */ W,
}

/// First element of each way includes the initial angle of texture rotation, and the texture rotation angle on the way to last point of the path, when the vehicle accelerated after turn(not used in code at the moment).
/// South(screen bottom) direction of sprite/ auto texture orientation is 0 degrees.
/// Other elements is the coordinates of the path points x and y respectively.
/**
 * ss means start to south then turn to south(no turn).
 * se means start to south then turn to east(turn left).
 * 
 * sum_ss means the full distance of the ss way in meters(converted from pixels)
 */
#[derive(Debug)]
 pub struct Way {
  //controllable using queue and switcher
  //forward
  pub ss: [[u16;2]; 5],
  pub nn: [[u16;2]; 5],
  pub ww: [[u16;2]; 5],
  pub ee: [[u16;2]; 5],
  //turn left
  pub se: [[u16;2]; 6],
  pub nw: [[u16;2]; 6],
  pub ws: [[u16;2]; 6],
  pub en: [[u16;2]; 6],
  //turn right. controllable using queue. always open for vehicles
  pub sw: [[u16;2]; 6],
  pub ne: [[u16;2]; 6],
  pub es: [[u16;2]; 6],
  pub wn: [[u16;2]; 6],
  
  // full distances for each way in meters(based on converted pixels)
  pub sum_ss: u16,
  pub sum_nn: u16,
  pub sum_ww: u16,
  pub sum_ee: u16,
  pub sum_se: u16,
  pub sum_nw: u16,
  pub sum_ws: u16,
  pub sum_en: u16,
  pub sum_sw: u16,
  pub sum_ne: u16,
  pub sum_es: u16,
  pub sum_wn: u16,
}

impl Way {
  pub fn new() -> Way {
  Way {
    //move forward
    ss: [[0,0], [4*CS,0],[4*CS,1*CS],[4*CS,2*CS],[4*CS,11*CS]],
    nn: [[180,180], [7*CS,11*CS],[7*CS,10*CS],[7*CS,9*CS],[7*CS,0]],
    ww: [[90,270], [11*CS, 4*CS],[10*CS, 4*CS],[9*CS, 4*CS],[0, 4*CS]],
    ee: [[270,90], [0,7*CS],[1*CS,7*CS],[2*CS,7*CS],[11*CS,7*CS]],
    //turn left
    se: [[0,90], [5*CS,0],[5*CS,1*CS],[5*CS,2*CS],[5*CS,6*CS],[11*CS,6*CS]],
    nw: [[180,270], [6*CS,11*CS],[6*CS,10*CS],[6*CS,9*CS],[6*CS,5*CS],[0,5*CS]],
    ws: [[90,0], [11*CS,5*CS],[10*CS,5*CS],[9*CS,5*CS],[5*CS,5*CS],[5*CS,11*CS]],
    en: [[270,180], [0,6*CS],[1*CS,6*CS],[2*CS,6*CS],[6*CS,6*CS],[6*CS,0]],
    //turn right
    sw: [[0,270], [3*CS,0],[3*CS,1*CS],[3*CS,2*CS],[3*CS,3*CS],[0,3*CS]],
    ne: [[180,90], [8*CS,11*CS],[8*CS,10*CS],[8*CS,9*CS],[8*CS,8*CS],[11*CS,8*CS]],
    es: [[270,0], [0,8*CS],[1*CS,8*CS],[2*CS,8*CS],[3*CS,8*CS],[3*CS,11*CS]],
    wn: [[90,180], [11*CS,3*CS],[10*CS,3*CS],[9*CS,3*CS],[8*CS,3*CS],[8*CS,0]],
    // full distances for each way in meters(based on converted pixels)
    // forward ways
    sum_ss: 11*CM,
    sum_nn: 11*CM,
    sum_ww: 11*CM,
    sum_ee: 11*CM,
    // turn left ways
    sum_se: 12*CM,
    sum_nw: 12*CM,
    sum_ws: 12*CM,
    sum_en: 12*CM,
    // turn right ways
    sum_sw: 6*CM,
    sum_ne: 6*CM,
    sum_es: 6*CM,
    sum_wn: 6*CM,
  }
}
}