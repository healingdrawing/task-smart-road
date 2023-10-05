use macroquad::prelude::Texture2D;

pub struct Textures {
    pub road: Texture2D,
    pub auto: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        Self {
            road: macroquad::texture::load_texture("assets/road.png")
                .await
                .unwrap(),
            auto: macroquad::texture::load_texture("assets/car.png")
                .await
                .unwrap(),
            
        }
    }
}
