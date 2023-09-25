use crate::config::CS;

/// First element of each way includes the initial angle of texture rotation, and the texture rotation angle on the way to last point of the path, when the car accelerated after turn.
/// South(screen bottom) direction is 0 degrees.
/// Other elements is the coordinates of the path points x and y respectively.
/**
 * SS means start to south then turn to south(no turn).
 * SE means start to south then turn to east(turn left).
 */
pub struct Ways {
    //controllable using queue and switcher
    //forward
    pub SS: [[u16;2]; 5],
    pub NN: [[u16;2]; 5],
    pub WW: [[u16;2]; 5],
    pub EE: [[u16;2]; 5],
    //turn left
    pub SE: [[u16;2]; 6],
    pub NW: [[u16;2]; 6],
    pub WS: [[u16;2]; 6],
    pub EN: [[u16;2]; 6],
    //turn right. controllable using queue. always open for cars
    pub SW: [[u16;2]; 6],
    pub NE: [[u16;2]; 6],
    pub ES: [[u16;2]; 6],
    pub WN: [[u16;2]; 6],
}

impl Ways {
    pub fn new() -> Ways {
        Ways {
            SS: [[0,0], [4*CS,0],[4*CS,1*CS],[4*CS,2*CS],[4*CS,11*CS]],
            NN: [[180,180], [7*CS,11],[7*CS,10*CS],[7*CS,9*CS],[7*CS,0]],
            WW: [[270,270], [11*CS, 4*CS],[10*CS, 4*CS],[9*CS, 4*CS],[0, 4*CS]],
            EE: [[90,90], [0,7*CS],[1*CS,7*CS],[2*CS,7*CS],[11*CS,7*CS]],
            SE: [[0,90], [5*CS,0],[5*CS,1*CS],[5*CS,2*CS],[5*CS,6*CS],[11*CS,6*CS]],
            NW: [[180,270], [6*CS,11*CS],[6*CS,10*CS],[6*CS,9*CS],[6*CS,5*CS],[0,5*CS]],
            WS: [[270,0], [11*CS,5*CS],[10*CS,5*CS],[9*CS,5*CS],[5*CS,5*CS],[5*CS,11*CS]],
            EN: [[90,180], [0,6*CS],[1*CS,6*CS],[2*CS,6*CS],[6*CS,6*CS],[6*CS,0]],
            SW: [[0,270], [3*CS,0],[3*CS,1*CS],[3*CS,2*CS],[3*CS,3*CS],[0,3*CS]],
            NE: [[180,90], [8*CS,11*CS],[8*CS,10*CS],[8*CS,9*CS],[8*CS,8*CS],[11*CS,8*CS]],
            ES: [[90,0], [0,8*CS],[1*CS,8*CS],[2*CS,8*CS],[3*CS,8*CS],[3*CS,11*CS]],
            WN: [[270,180], [11*CS,3*CS],[10*CS,3*CS],[9*CS,3*CS],[8*CS,3*CS],[8*CS,0]],
        }
    }
}