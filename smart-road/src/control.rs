use std::time::Instant;

use crate::traffic::Road;
use macroquad::prelude::*;

pub fn handle_input(road: &mut Road) {
  if is_key_pressed(KeyCode::Escape) {
    if road.show_stats{
      std::process::exit(0);
    } else {
      road.simulation_end_time = Instant::now();
      road.show_stats = true;
    }
    
  }

  if is_key_pressed(KeyCode::Up) {
    road.try_add_auto_north_directed();
    road.no_spam();
  }

  if is_key_pressed(KeyCode::Down) {
    road.try_add_auto_south_directed();
    road.no_spam();
  }

  if is_key_pressed(KeyCode::Right) {
    road.try_add_auto_east_directed();
    road.no_spam();
  }

  if is_key_pressed(KeyCode::Left) {
    road.try_add_auto_west_directed();
    road.no_spam();
  }

  if is_key_pressed(KeyCode::R) {
    road.random_car_generation = !road.random_car_generation; // Toggle the flag
  
  }

  if is_key_pressed(KeyCode::S) {
    road.switch_spam();
  }

}
