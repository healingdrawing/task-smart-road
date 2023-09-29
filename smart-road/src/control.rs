use crate::traffic::{Direction, TrafficState, Calc};
use macroquad::prelude::*;

pub fn handle_input(traffic_state: &mut TrafficState, calc: &mut Calc) {
  if is_key_pressed(KeyCode::Escape) {
    std::process::exit(0);
    //todo implement first the statistic window appear, then exit after second press
  }

  if is_key_pressed(KeyCode::Up) {
    traffic_state.add_car(Direction::S);
    calc.try_add_auto_north_directed();
  }

  if is_key_pressed(KeyCode::Down) {
    traffic_state.add_car(Direction::N);
    calc.try_add_auto_south_directed();
  }

  if is_key_pressed(KeyCode::Right) {
    traffic_state.add_car(Direction::W);
    calc.try_add_auto_east_directed();
  }

  if is_key_pressed(KeyCode::Left) {
    traffic_state.add_car(Direction::E);
    calc.try_add_auto_west_directed();
  }

  if is_key_pressed(KeyCode::R) {
    traffic_state.add_car_random();
    calc.try_add_auto_random_directed();
  }
}
