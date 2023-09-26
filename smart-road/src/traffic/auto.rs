
pub struct Auto{
  pub start_x:u16, // start point of the way(between two way points)
  pub start_y:u16,
  pub target_x:u16, // target point of the way(between two way points)
  pub target_y:u16,
  pub x:u16,
  pub y:u16,
  pub speed:u16,
  pub sum_distance:u16, // used to compare the distance car moved with the distance of the way
  pub moving:bool, // is the car moving now. when the move completed, the moving will be false. so new move can be started or car can be removed, if the move was the last in the way
}

