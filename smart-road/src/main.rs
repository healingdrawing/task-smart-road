mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use config::window_conf;

use control::handle_input;

use crate::draw::{draw_road, draw_stats, Textures};
use crate::traffic::Road;

#[macroquad::main(window_conf)]
async fn main() {
  let textures = Textures::load().await;

  let mut road = Road::new(&textures);

  loop {
    handle_input(&mut road);

    clear_background(BLACK);

    if road.show_stats {
      draw_stats(&road);
    } else {
      draw_road(&textures); // draw background
      road.update(); // calculate changes
    }
    
    next_frame().await
  }
}
