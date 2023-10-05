use crate::traffic::Road;
use macroquad::prelude::*;

pub fn handle_input(road: &mut Road) {
  if is_key_pressed(KeyCode::Escape) {
    println!("{}",road.format_stats());
    std::process::exit(0);
    //todo implement first the statistic window appear, then exit after second press
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
    road.try_add_auto_random_directed();
    road.no_spam();
  }

  if is_key_pressed(KeyCode::S) {
    road.switch_spam();
  }

}
