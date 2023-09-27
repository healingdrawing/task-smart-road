use macroquad::prelude::*;

use crate::config::PXS;
use std::time::Instant;

#[derive(Debug)]
pub struct Auto{
  pub texture:Texture2D, // texture of the car
  pub texture_angle:f32, // angle of texture rotation
  pub init_time:Instant, // time when the car was created
  pub start_time:Instant, // time when the car started moving
  pub sum_dist:f32, // full distance between two way points
  pub dist:f32, // distance car alredy moved between two way points
  pub sign_x:f32, // sign of the x direction -1.0 or 1.0 . 0.0 default
  pub sign_y:f32, // sign of the y direction -1.0 or 1.0 . 0.0 default
  pub from_x:f32, // start point of the way(between two way points)
  pub from_y:f32,
  pub x:f32,
  pub y:f32,
  pub to_x:f32, // target point of the way(between two way points)
  pub to_y:f32,
  pub turbo:f32, // car speed multiplier. 0.0 default
  pub moving:bool, // is the car moving now. when the move completed, the moving will be false. so new move can be started or car can be removed, if the move was the last in the way
}

impl Auto{
  pub fn new(x:f32, y:f32, texture_angle:f32, texture:&Texture2D)->Self{
    Self{
      texture:texture.clone(),
      texture_angle:texture_angle.to_radians(),
      init_time:Instant::now(),
      start_time:Instant::now(),
      sum_dist:0.0,
      dist:0.0,
      sign_x:0.0,
      sign_y:0.0,
      from_x:x,
      from_y:y,
      x,
      y,
      to_x:x,
      to_y:y,
      turbo:0.0,
      moving:false,
    }
  }

  pub fn animate_step(&mut self) {
    if self.moving {
      let elapsed_secs = self.start_time.elapsed().as_secs_f32();
      println!("elapsed_secs: {}", elapsed_secs);
      println!("x: {}, to_x: {}, y: {}, to_y: {}", self.x, self.to_x, self.y, self.to_y);
      // Calculate the Euclidean distance directly
      self.dist = ((self.x - self.from_x).powi(2) + (self.y - self.from_y).powi(2)).sqrt();

      println!("dist: {}, sum_dist: {}", self.dist, self.sum_dist);
      if self.dist < self.sum_dist {
        self.x = self.from_x + elapsed_secs * PXS * self.turbo * self.sign_x;
        println!("x: {}", self.x);
        self.y = self.from_y + elapsed_secs * PXS * self.turbo * self.sign_y;
      } else {
        println!("wtf");
        self.x = self.to_x;
        self.y = self.to_y;
        self.moving = false;
      }
    }
    self.draw();
  }

  fn draw(&self) {
    draw_texture_ex(
      &self.texture,
      self.x,
      self.y,
      WHITE,
      DrawTextureParams {
          rotation: self.texture_angle,
          ..Default::default()
      },
  );
    // Implement your drawing logic here
  }

  pub fn animate_to(&mut self, to_x: f32, to_y: f32, turbo: f32) {
    if !self.moving {
      self.sum_dist = ((self.x - to_x).powi(2) + (self.y - to_y).powi(2)).sqrt();
      self.dist = 0.0;
      self.sign_x = if self.x < to_x { 1.0 } else if self.x > to_x { -1.0 } else { 0.0 };
      self.sign_y = if self.y < to_y { 1.0 } else if self.y > to_y { -1.0 } else { 0.0 };
      self.from_x = self.x;
      self.from_y = self.y;
      self.to_x = to_x;
      self.to_y = to_y;
      self.start_time = std::time::Instant::now();
      self.turbo = turbo;
      self.moving = true;
      // todo check. The texture at the moment aimed to bottom default
      if self.sign_x == 0.0 && self.sign_y != 0.0 {
        self.texture_angle = if self.sign_y == 1.0 { 0.0f32 } else { 180.0f32 }.to_radians()
      } else if self.sign_y == 0.0 && self.sign_x != 0.0 {
        self.texture_angle = (if self.sign_x == 1.0 { 270.0f32 } else { 90.0f32 }).to_radians()
      }
    }
  }

}