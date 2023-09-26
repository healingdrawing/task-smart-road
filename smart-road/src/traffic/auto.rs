
#[derive(Debug)]
pub struct Auto{
  pub sum_dist:f32, // full distance between two way points
  pub dist:f32, // distance car alredy moved between two way points
  pub init_time:f32, // time when the car was created
  pub start_time:f32, // time when the car started moving
  pub target_time:f32, // time when the car will reach the end of the way

  pub start_x:f32, // start point of the way(between two way points)
  pub start_y:f32,
  pub target_x:f32, // target point of the way(between two way points)
  pub target_y:f32,
  pub x:f32,
  pub y:f32,
  pub speed:f32,
  pub moving:bool, // is the car moving now. when the move completed, the moving will be false. so new move can be started or car can be removed, if the move was the last in the way

}

