use macroquad::texture::Texture2D;
use rand::Rng;

use crate::draw::Textures;
use crate::traffic::way::{Way, To};
use crate::traffic::autos::Autos;
use crate::traffic::auto::Auto;

use super::stack::LimitedStack;

pub struct Calc<'a> {
  pub way: Way,
  pub autos: Autos,
  pub textures: &'a Textures,
}

impl<'a> Calc<'a> {
  pub fn new(textures: &'a Textures) -> Self {
    Self {
      way: Way::new(),
      autos: Autos::new(),
      textures,
    }
  }

  pub fn update(&mut self) {
    self.manage_autos();
    self.animate_step();
    // println!("update"); //todo hide
  }
  
  fn next_way_point_is_free(&self, oldxy:&[u16;2], xy:&[u16;2], lane_autos:&LimitedStack<Auto>) -> bool {
    let free =
    lane_autos.iter().all(|auto| auto.to_x != xy[0].into() || auto.to_y != xy[1].into() );
    // println!("free: {}, oldxy {:#?}, xy {:#?} , autos {:#?}", free, oldxy, xy, lane_autos); //todo hide
    free
  }

  /** calculate the behavior of autos to allow them properly move through cross road */
  fn manage_autos(&mut self) {
    //todo
    let mut pop_first: bool = false; // if some auto done the way,remove it from the stack
    let mut autos_ne_clone = self.autos.ne.clone();  // Clone self.autos.ne

    autos_ne_clone.iter_mut().for_each(|auto| {
      if !auto.moving {
          let target = self.way.ne.iter().skip(1).position(|point| point[0] as f32 == auto.to_x && point[1] as f32 == auto.to_y).unwrap();
          if target == self.way.ne.len() - 1 {
              // end of the way achieved
              pop_first = true;
          } else if self.next_way_point_is_free(
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

  /** animate autos */
  fn animate_step(&mut self) {
    self.autos.ne.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.nn.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.nw.iter_mut().for_each(|auto| auto.animate_step());
    
    self.autos.sw.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ss.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.se.iter_mut().for_each(|auto| auto.animate_step());

    self.autos.wn.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ww.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ws.iter_mut().for_each(|auto| auto.animate_step());

    self.autos.es.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.ee.iter_mut().for_each(|auto| auto.animate_step());
    self.autos.en.iter_mut().for_each(|auto| auto.animate_step());
  }

  //todo extend to random texture
  /** random auto texture */
  fn a_tex(&self) -> &Texture2D { &self.textures.auto }

  /** texture angle for initial auto position */
  fn t_ang(&self, road:To) -> f32 {
    match road {
      To::N => self.way.nn[0][0] as f32,
      To::S => self.way.ss[0][0] as f32,
      To::W => self.way.ww[0][0] as f32,
      _ => self.way.ee[0][0] as f32,
    }
  }

  //todo very muddy. 
  fn a_xy(&self, road: To, lane_number: u8) -> &[f32; 2] {
    let result: Box<[f32; 2]> = match road {
        To::N => match lane_number {
            0 => Box::new([self.way.ne[1][0] as f32, self.way.ne[1][1] as f32]),
            1 => Box::new([self.way.nn[1][0] as f32, self.way.nn[1][1] as f32]),
            _ => Box::new([self.way.nw[1][0] as f32, self.way.nw[1][1] as f32]),
        },
        To::S => match lane_number {
            0 => Box::new([self.way.sw[1][0] as f32, self.way.sw[1][1] as f32]),
            1 => Box::new([self.way.ss[1][0] as f32, self.way.ss[1][1] as f32]),
            _ => Box::new([self.way.se[1][0] as f32, self.way.se[1][1] as f32]),
        },
        To::W => match lane_number {
            0 => Box::new([self.way.wn[1][0] as f32, self.way.wn[1][1] as f32]),
            1 => Box::new([self.way.ww[1][0] as f32, self.way.ww[1][1] as f32]),
            _ => Box::new([self.way.ws[1][0] as f32, self.way.ws[1][1] as f32]),
        },
        _ => match lane_number {
            0 => Box::new([self.way.es[1][0] as f32, self.way.es[1][1] as f32]),
            1 => Box::new([self.way.ee[1][0] as f32, self.way.ee[1][1] as f32]),
            _ => Box::new([self.way.en[1][0] as f32, self.way.en[1][1] as f32]),
        },
    };
    Box::leak(result)
}

  fn add_auto(&self, road:To, lane_number: u8) -> Auto {
    Auto::new(self.a_xy(road, lane_number),self.t_ang(road),self.a_tex())
  }

  fn auto_targeted_to_point_does_not_move(&self, point:&[u16; 2], lane_autos:&LimitedStack<Auto>) -> bool {
    lane_autos.iter().any(|auto| auto.to_x == point[0] as f32 && auto.to_y == point[1] as f32 && auto.moving == false)
  }

  fn no_autos_targeted_to_point(&self, point:&[u16; 2], lane_autos:&LimitedStack<Auto>) -> bool {
    lane_autos.iter().all(|auto| auto.to_x != point[0] as f32 && auto.to_y != point[1] as f32)
  }

  fn no_autos_targeted_to_lane_first_point(
    &self,
    road_direction: To,
    lane_number: u8,
    lane_autos:&LimitedStack<Auto>,
  ) -> bool {
    // todo
    match road_direction {
      To::N => match lane_number {
        // take the first point of the lane, which is under index 1
        0 => { self.no_autos_targeted_to_point(&self.way.ne[1], lane_autos) }/*turn right*/ 
        1 => { self.no_autos_targeted_to_point(&self.way.nn[1], lane_autos) }/*no turn*/
        _ => { self.no_autos_targeted_to_point(&self.way.nw[1], lane_autos) }/*turn left*/
      }
      To::S => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.sw[1], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ss[1], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.se[1], lane_autos) }
      }
      To::W => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.wn[1], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ww[1], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.ws[1], lane_autos) }
      }
      _ => match lane_number { // To::E , east
        0 => { self.no_autos_targeted_to_point(&self.way.es[1], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ee[1], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.en[1], lane_autos) }
      }
    }
    
  }

  fn no_autos_targeted_to_lane_second_point(
    &self,
    road_direction: To,
    lane_number: u8,
    lane_autos:&LimitedStack<Auto>,
  ) -> bool {
    // todo
    match road_direction {
      To::N => match lane_number {
        // take the second point of the lane, which is under index 2
        0 => { self.no_autos_targeted_to_point(&self.way.ne[2], lane_autos) }/*turn right*/ 
        1 => { self.no_autos_targeted_to_point(&self.way.nn[2], lane_autos) }/*no turn*/
        _ => { self.no_autos_targeted_to_point(&self.way.nw[2], lane_autos) }/*turn left*/
      }
      To::S => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.sw[2], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ss[2], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.se[2], lane_autos) }
      }
      To::W => match lane_number {
        0 => { self.no_autos_targeted_to_point(&self.way.wn[2], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ww[2], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.ws[2], lane_autos) }
      }
      _ => match lane_number { // To::E , east
        0 => { self.no_autos_targeted_to_point(&self.way.es[2], lane_autos) }
        1 => { self.no_autos_targeted_to_point(&self.way.ee[2], lane_autos) }
        _ => { self.no_autos_targeted_to_point(&self.way.en[2], lane_autos) }
      }
    }
    
  }

  fn auto_targeted_to_second_point_does_not_move(&self, road_direction: To, lane_number: u8, lane_autos:&LimitedStack<Auto>) -> bool {
    match road_direction {
      To::N => match lane_number {
        // take the second point of the lane, which is under index 2
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.ne[2], lane_autos) }/*turn right*/
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.nn[2], lane_autos) }/*no turn*/
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.nw[2], lane_autos) }/*turn left*/
      }
      To::S => match lane_number {
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.sw[2], lane_autos) }
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.ss[2], lane_autos) }
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.se[2], lane_autos) }
      }
      To::W => match lane_number {
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.wn[2], lane_autos) }
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.ww[2], lane_autos) }
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.ws[2], lane_autos) }
      }
      _ => match lane_number { // To::E , east
        0 => { self.auto_targeted_to_point_does_not_move(&self.way.es[2], lane_autos) }
        1 => { self.auto_targeted_to_point_does_not_move(&self.way.ee[2], lane_autos) }
        _ => { self.auto_targeted_to_point_does_not_move(&self.way.en[2], lane_autos) }
      }
    }
  }

  fn lane_is_free(&self, road_direction:To, lane_number: u8) -> bool {
    // todo
    let lane_autos = match road_direction {
      To::N => match lane_number {
        0 => &self.autos.ne, // turn right
        1 => &self.autos.nn, // no turn
        _ => &self.autos.nw, // turn left
      }
      To::S => match lane_number {
        0 => &self.autos.sw,
        1 => &self.autos.ss,
        _ => &self.autos.se,
      }
      To::W => match lane_number {
        0 => &self.autos.wn,
        1 => &self.autos.ww,
        _ => &self.autos.ws,
      }
      _ => match lane_number {
        0 => &self.autos.es,
        1 => &self.autos.ee,
        _ => &self.autos.en,
      }
    };

    //todo implement mulitstatement, then remove comments
    /*no autos targeted to lane first point and no targeted to second point OR
    no autos targeted to lane first point and there is auto targeted to second point, but this auto does not move, so this auto already in destination point (on second point), so no collision in add moment
     */
    self.no_autos_targeted_to_lane_first_point(road_direction, lane_number, lane_autos)
    && (
      self.no_autos_targeted_to_lane_second_point(road_direction, lane_number, lane_autos)
      ||
      self.auto_targeted_to_second_point_does_not_move(road_direction, lane_number, lane_autos)
    ) //todo check it twice, does not look clear
  }

  

  /** generate random number from 0 to 2 to choose the lane */
  fn random_lane(&self, road:To) -> u8 {
    let mut rng = rand::thread_rng();
    let mut choices = vec![0, 1, 2];

    while !choices.is_empty() {
      let index = rng.gen_range(0..choices.len());
      let lane_number = choices.remove(index);

      if self.lane_is_free(road, lane_number) {
        println!("road: {:?} lane_number: {}", road, lane_number); //todo hide
        return lane_number;
      }
    }

    // If all choices are taken, return the (3) as a fail value
    3
  }

  /** input handler call this method, to try add auto from south to north */
  pub fn try_add_auto_north_directed(&mut self) {
    println!("try_add_auto_north_directed calc.rs"); //todo hide
    match self.random_lane(To::N) {
      0 => {self.autos.ne.push(self.add_auto(To::N, 0));},
      1 => {self.autos.nn.push(self.add_auto(To::N, 1));},
      2 => {self.autos.nw.push(self.add_auto(To::N, 2));},
      _ => println!("lane error"), //keep it, it should not happen
    }
  }

  /** input handler call this method, to try add auto from north to south */
  pub fn try_add_auto_south_directed(&mut self) {
    println!("try_add_auto_south_directed calc.rs"); //todo hide
    match self.random_lane(To::S) {
      0 => {self.autos.sw.push(self.add_auto(To::S, 0));},
      1 => {self.autos.ss.push(self.add_auto(To::S, 1));},
      2 => {self.autos.se.push(self.add_auto(To::S, 2));},
      _ => println!("lane error"), //keep it, it should not happen
    }
  }

  /** input handler call this method, to try add auto from east to west */
  pub fn try_add_auto_west_directed(&mut self) {
    println!("try_add_auto_west_directed calc.rs"); //todo hide
    match self.random_lane(To::W) {
      0 => {self.autos.wn.push(self.add_auto(To::W, 0));},
      1 => {self.autos.ww.push(self.add_auto(To::W, 1));},
      2 => {self.autos.ws.push(self.add_auto(To::W, 2));},
      _ => println!("lane error"), //keep it, it should not happen
    }
  }

  /** input handler call this method, to try add auto from west to east */
  pub fn try_add_auto_east_directed(&mut self) {
    println!("try_add_auto_east_directed calc.rs"); //todo hide
    match self.random_lane(To::E) {
      0 => {self.autos.es.push(self.add_auto(To::E, 0));},
      1 => {self.autos.ee.push(self.add_auto(To::E, 1));},
      2 => {self.autos.en.push(self.add_auto(To::E, 2));},
      _ => println!("lane error"), //keep it, it should not happen
    }
  }

  /** input handler call this method */
  pub fn try_add_auto_random_directed(&mut self) {
    println!("try_add_auto_random_directed calc.rs"); //todo hide
    match rand::thread_rng().gen_range(0..4) {
      0 => {self.try_add_auto_north_directed();},
      1 => {self.try_add_auto_south_directed();},
      2 => {self.try_add_auto_west_directed();},
      _ => {self.try_add_auto_east_directed();},
    }
  }
}
