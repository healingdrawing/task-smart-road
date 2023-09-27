mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use config::window_conf;

use control::handle_input;

use crate::draw::{/* draw_light, */ draw_path, draw_roads, Textures};
use crate::traffic::TrafficState;
use crate::traffic::Autos;
use crate::traffic::Auto;
use draw::draw_car;

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();
    let textures = Textures::load().await;

    let mut autos = Autos::new();
    autos.ss.push(Auto::new(100f32,100f32,0f32,&textures.auto));
    autos.ss.push(Auto::new(200f32,100f32,0f32,&textures.auto));
    
    autos.ss.iter_mut().for_each(|auto|
        if auto.x == 100f32 {auto.animate_to(100f32, 300f32, 1f32)}
        else{auto.animate_to(100f32, 100f32, 1f32)}
    );



    loop {
        handle_input(&mut traffic_state);

        traffic_state.update();

        clear_background(BLACK); //todo delete, not sure it needed, because road has no transparency

        draw_roads(&textures);
        autos.ss.iter_mut().for_each(|auto| auto.animate_step());

        for line in traffic_state.lines.iter() {
            // draw_light(line, &textures);

            for path in line.paths.iter() {
                draw_path(path);
            }
        }

        for line in traffic_state.lines.iter() {
            for car in line.cars.iter() {
                draw_car(car);
            }
        }

        next_frame().await
    }
}
