use rand::Rng;

use macroquad::prelude::Texture2D;

pub struct Textures {
    pub road: Texture2D,
    pub auto: Texture2D,
    pub auto_blue: Texture2D,
    pub auto_green: Texture2D,
    pub auto_grey: Texture2D,
    pub auto_red: Texture2D,
    pub excavator: Texture2D,
    pub garbage_truck: Texture2D,
    pub ambulance: Texture2D,
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
      auto_blue: macroquad::texture::load_texture("assets/car_blue.png")
        .await
        .unwrap(),
      auto_green: macroquad::texture::load_texture("assets/car_green.png")
        .await
        .unwrap(),
      auto_grey: macroquad::texture::load_texture("assets/car_grey.png")
        .await
        .unwrap(),
      auto_red: macroquad::texture::load_texture("assets/car_red.png")
        .await
        .unwrap(),
      excavator: macroquad::texture::load_texture("assets/excavator.png")
        .await
        .unwrap(),
      garbage_truck: macroquad::texture::load_texture("assets/garbage_truck.png")
        .await
        .unwrap(),
      ambulance: macroquad::texture::load_texture("assets/ambulance.png")
        .await
        .unwrap(),
        
    }
  }

  pub fn random_auto(&self) -> &Texture2D {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..8);
    match r {
      1 => &self.auto_blue,
      2 => &self.auto_green,
      3 => &self.auto_grey,
      4 => &self.auto_red,
      5 => &self.excavator,
      6 => &self.garbage_truck,
      7 => &self.ambulance,
      _ => &self.auto,
    }
  }
}
