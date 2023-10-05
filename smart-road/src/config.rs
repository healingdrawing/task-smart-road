use macroquad::window::Conf;

pub const WINDOW_SIZE: i32 = 1152;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "task smart-road".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

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

