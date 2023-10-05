mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use config::window_conf;

use control::handle_input;

use crate::draw::{draw_road, Textures};
use crate::traffic::Road;

#[macroquad::main(window_conf)]
async fn main() {
  let textures = Textures::load().await;

  let mut road = Road::new(&textures);

  loop {
    handle_input(&mut road);

    clear_background(BLACK); //todo delete, not sure it needed, because road has no transparency

    draw_road(&textures);
    
    road.update();
    
    next_frame().await
  }
}
