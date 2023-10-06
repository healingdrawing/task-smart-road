use crate::{draw::Textures, traffic::Road};
use macroquad::prelude::*;

pub fn draw_road(textures: &Textures) {
    let texture =  &textures.road;

    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            ..Default::default()
        },
    );
}

pub fn draw_stats(road: &Road) {
    let stats = road.format_stats();
    let font_size = 64.0;

    let lines: Vec<&str> = stats.lines().collect();
    let line_height = font_size + 2.0; // Adjust the spacing between lines if needed

    for (i, line) in lines.iter().enumerate() {
        draw_text(
            line,
            100f32,
            100f32 + i as f32 * line_height,
            font_size,
            Color::new(245f32, 245f32, 245f32, 1f32),
        );
    }
}