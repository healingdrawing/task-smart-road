use crate::traffic::{To, TrafficState};
use macroquad::prelude::*;

pub fn handle_input(traffic_state: &mut TrafficState) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if is_key_pressed(KeyCode::Up) {
        traffic_state.add_car(To::S);
    }

    if is_key_pressed(KeyCode::Down) {
        traffic_state.add_car(To::N);
    }

    if is_key_pressed(KeyCode::Right) {
        traffic_state.add_car(To::W);
    }

    if is_key_pressed(KeyCode::Left) {
        traffic_state.add_car(To::E);
    }

    if is_key_pressed(KeyCode::R) {
        traffic_state.add_car_random();
    }
}
