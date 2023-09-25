use crate::config::CS;

/// First element of each way includes the initial angle of texture rotation, and the texture rotation angle on the way to last point of the path, when the car accelerated after turn.
/// South(screen bottom) direction is 0 degrees.
/// Other elements is the coordinates of the path points x and y respectively.
/**
 * SS means start to south then turn to south(no turn).
 * SE means start to south then turn to east(turn left).
 * 
 * free_SS means the array of [bool,bool] for each point in SS, to manage place
 * for car moving. The first element with index zero includes array
 * of two boolean, where the free_SS[0][0] means means the boolean
 * for the control the global condition of the SS line, is the SS opens for
 * car moving over cross road, or another way turn now. The second element of the
 * SS[0][0] does not used at the moment.
 * The free_SS parameter condition will be mutated by the smart road.
 */
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
    //turn right. controllable using queue. always open for cars
    pub sw: [[u16;2]; 6],
    pub ne: [[u16;2]; 6],
    pub es: [[u16;2]; 6],
    pub wn: [[u16;2]; 6],
    
    // now declare the arrays which represent the every point of each way, to check is it free
    pub free_ss: [[bool;2]; 5],
    pub free_nn: [[bool;2]; 5],
    pub free_ww: [[bool;2]; 5],
    pub free_ee: [[bool;2]; 5],
    pub free_se: [[bool;2]; 6],
    pub free_nw: [[bool;2]; 6],
    pub free_ws: [[bool;2]; 6],
    pub free_en: [[bool;2]; 6],
    pub free_sw: [[bool;2]; 6],
    pub free_ne: [[bool;2]; 6],
    pub free_es: [[bool;2]; 6],
    pub free_wn: [[bool;2]; 6],
}

impl Way {
    pub fn new() -> Way {
        Way {
            //move forward
            ss: [[0,0], [4*CS,0],[4*CS,1*CS],[4*CS,2*CS],[4*CS,11*CS]],
            nn: [[180,180], [7*CS,11],[7*CS,10*CS],[7*CS,9*CS],[7*CS,0]],
            ww: [[270,270], [11*CS, 4*CS],[10*CS, 4*CS],[9*CS, 4*CS],[0, 4*CS]],
            ee: [[90,90], [0,7*CS],[1*CS,7*CS],[2*CS,7*CS],[11*CS,7*CS]],
            //turn left
            se: [[0,90], [5*CS,0],[5*CS,1*CS],[5*CS,2*CS],[5*CS,6*CS],[11*CS,6*CS]],
            nw: [[180,270], [6*CS,11*CS],[6*CS,10*CS],[6*CS,9*CS],[6*CS,5*CS],[0,5*CS]],
            ws: [[270,0], [11*CS,5*CS],[10*CS,5*CS],[9*CS,5*CS],[5*CS,5*CS],[5*CS,11*CS]],
            en: [[90,180], [0,6*CS],[1*CS,6*CS],[2*CS,6*CS],[6*CS,6*CS],[6*CS,0]],
            //turn right
            sw: [[0,270], [3*CS,0],[3*CS,1*CS],[3*CS,2*CS],[3*CS,3*CS],[0,3*CS]],
            ne: [[180,90], [8*CS,11*CS],[8*CS,10*CS],[8*CS,9*CS],[8*CS,8*CS],[11*CS,8*CS]],
            es: [[90,0], [0,8*CS],[1*CS,8*CS],[2*CS,8*CS],[3*CS,8*CS],[3*CS,11*CS]],
            wn: [[270,180], [11*CS,3*CS],[10*CS,3*CS],[9*CS,3*CS],[8*CS,3*CS],[8*CS,0]],
            // in the initial state first two ways are open for cars, let is say it is both side vertical ways
            free_ss: [[true,false],[true,true],[true,true],[true,true],[true,true]],
            free_nn: [[true,false],[true,true],[true,true],[true,true],[true,true]],
            free_ww: [[false,false],[true,true],[true,true],[true,true],[true,true]],
            free_ee: [[false,false],[true,true],[true,true],[true,true],[true,true]],
            // in the initial state first two ways are open for cars, let is say it is both side vertical + turn left
            free_se: [[true,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            free_nw: [[true,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            free_ws: [[false,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            free_en: [[false,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            // always open for cars, but still controllable using queue, because turn right is always open for cars
            free_sw: [[true,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            free_ne: [[true,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            free_es: [[true,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
            free_wn: [[true,false],[true,true],[true,true],[true,true],[true,true],[true,true]],
        }
    }
}