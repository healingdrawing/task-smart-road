use crate::draw::Textures;
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
