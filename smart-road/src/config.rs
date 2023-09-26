use macroquad::math::Vec2;
use macroquad::window::Conf;

pub const WINDOW_SIZE: i32 = 1152;

pub const CAR_WIDTH: f32 = 96.0;
pub const CAR_LENGTH: f32 = 96.0;

/// Distance between the car and the middle of the road
pub const CAR_PADDING: f32 = 0.0;
pub const CAR_SAFE_DISTANCE: f32 = 96.0;

// pub const LIGHTS_SIZE: f32 = ROAD_WIDTH / 2.0;

// pub const LIGHTS_TIMEOUT: f32 = 5.0; // seconds

pub const CAR_SPEED: f32 = 3.0;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "task smart-road".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

// Helper constants

pub const STRAIGHT_LENGTH: f32 = 288.0;

// center square corners
pub const TOP_LEFT: Vec2 = Vec2::new(STRAIGHT_LENGTH, STRAIGHT_LENGTH);

// pub const TOP_RIGHT: Vec2 = Vec2::new(WINDOW_SIZE as f32 - STRAIGHT_LENGTH, STRAIGHT_LENGTH);

// pub const BOTTOM_LEFT: Vec2 = Vec2::new(STRAIGHT_LENGTH, WINDOW_SIZE as f32 - STRAIGHT_LENGTH);

pub const BOTTOM_RIGHT: Vec2 = Vec2::new(
    WINDOW_SIZE as f32 - STRAIGHT_LENGTH,
    WINDOW_SIZE as f32 - STRAIGHT_LENGTH,
);

//todo remove later. new code injection

/** cell size of the box along x and y axes. Plan to use in lot of cases.
 * Width of the road(one lane).
 * Size of the car sprite with safe empty pixels around.
 * Distance between cars destination coordinates in the line,
 * before car starts move cross road
 * */
pub const CS:u16 = 96;

/**how many meters in one cell
 * let it be 4 meters(24px per meter). For statistic needs
 */
pub const CM:u16 = 4;

/** pixels per second, slow speed of the car */
pub const PXS:f32 = 96.0;

