use macroquad::prelude::*;

use crate::config::PXS;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Auto{
  pub texture:Texture2D, // texture of the auto
  pub texture_angle:f32, // angle of texture rotation
  pub init_time:Instant, // time when the auto was created
  pub start_time:Instant, // time when the auto started moving
  pub sum_dist:f32, // full distance between two way points
  pub dist:f32, // distance auto alredy moved between two way points
  pub sign_x:f32, // sign of the x direction -1.0 or 1.0 . 0.0 default
  pub sign_y:f32, // sign of the y direction -1.0 or 1.0 . 0.0 default
  pub from_x:f32, // start point of the way(between two way points)
  pub from_y:f32,
  pub x:f32,
  pub y:f32,
  pub to_x:f32, // target point of the way(between two way points)
  pub to_y:f32,
  pub turbo:f32, // auto speed multiplier. 0.0 default
  pub moving:bool, // is the auto moving now. when the move completed, the moving will be false. so new move can be started or auto can be removed, if the move was the last in the way
}

impl Auto{
  /** 
   * xy is left upper corner of texture position on screen
   * texture angle (degrees) in the initial moment when auto added on screen.
   Positive direction cw (microquad rule, looks like creator hates school math)
   */
  pub fn new(xy:&[f32;2], texture_angle:f32, texture:&Texture2D)->Self{
    Self{
      texture:texture.clone(),
      texture_angle:texture_angle.to_radians(),
      init_time:Instant::now(),
      start_time:Instant::now(),
      sum_dist:0.0,
      dist:0.0,
      sign_x:0.0,
      sign_y:0.0,
      from_x:xy[0],
      from_y:xy[1],
      x:xy[0],
      y:xy[1],
      to_x:xy[0],
      to_y:xy[1],
      turbo:0.0,
      moving:false,
    }
  }

  pub fn animate_step(&mut self) {
    if self.moving {
      let elapsed_secs = self.start_time.elapsed().as_secs_f32();
      self.dist = ((self.x - self.from_x).powi(2) + (self.y - self.from_y).powi(2)).sqrt();

      if self.dist < self.sum_dist {
        self.x = self.from_x + elapsed_secs * PXS * self.turbo * self.sign_x;
        self.y = self.from_y + elapsed_secs * PXS * self.turbo * self.sign_y;
      } else {
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
  }

  pub fn animate_to(&mut self, xy:&[f32;2], turbo: f32) {
    if !self.moving {
      self.sum_dist = ((self.x - xy[0]).powi(2) + (self.y - xy[1]).powi(2)).sqrt();
      self.dist = 0.0;
      self.sign_x = if self.x < xy[0] { 1.0 } else if self.x > xy[0] { -1.0 } else { 0.0 };
      self.sign_y = if self.y < xy[1] { 1.0 } else if self.y > xy[1] { -1.0 } else { 0.0 };
      self.from_x = self.x;
      self.from_y = self.y;
      self.to_x = xy[0];
      self.to_y = xy[1];
      self.start_time = Instant::now();
      self.turbo = turbo;
      self.moving = true;
      
      if self.sign_x == 0.0 && self.sign_y != 0.0 {
        self.texture_angle = if self.sign_y == 1.0 { 0.0f32 } else { 180.0f32 }.to_radians()
      } else if self.sign_y == 0.0 && self.sign_x != 0.0 {
        self.texture_angle = (if self.sign_x == 1.0 { 270.0f32 } else { 90.0f32 }).to_radians()
      }
      
    }
  }

}