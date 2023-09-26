use crate::traffic::auto::Auto;
use crate::traffic::stack::LimitedStack;

/**the limited stack of autos for each way. Plan to use it for iteration
 * capacity for each stack is length of the way minus one,
 * because the first element of the way is not a point
 * but rotation angles for texture
 */
#[derive(Debug)]
 pub struct Autos{
  pub ss:LimitedStack<Auto>,
  pub nn:LimitedStack<Auto>,
  pub ww:LimitedStack<Auto>,
  pub ee:LimitedStack<Auto>,

  pub se:LimitedStack<Auto>,
  pub nw:LimitedStack<Auto>,
  pub ws:LimitedStack<Auto>,
  pub en:LimitedStack<Auto>,

  pub sw:LimitedStack<Auto>,
  pub ne:LimitedStack<Auto>,
  pub es:LimitedStack<Auto>,
  pub wn:LimitedStack<Auto>,
}

impl Autos{
   fn new()->Autos{
    Autos{
      ss:LimitedStack::new(4),
      nn:LimitedStack::new(4),
      ww:LimitedStack::new(4),
      ee:LimitedStack::new(4),

      se:LimitedStack::new(5),
      nw:LimitedStack::new(5),
      ws:LimitedStack::new(5),
      en:LimitedStack::new(5),

      sw:LimitedStack::new(5),
      ne:LimitedStack::new(5),
      es:LimitedStack::new(5),
      wn:LimitedStack::new(5),
    }
  }
}