use macroquad::window::Conf;

pub const WINDOW_SIZE: i32 = 1152;

pub const WINDOW_TITLE : &str = "Task \"smart-road\" . Grit:lab Åland Islands 2023 . Controls: [S] - on/off spam vehicles, [R] - random vehicles, [Esc] - statistics -> exit";

pub fn window_conf() -> Conf {
  Conf {
    window_title: WINDOW_TITLE.to_owned(),
    window_width: WINDOW_SIZE,
    window_height: WINDOW_SIZE,
    window_resizable: false,
    ..Default::default()
  }
}

/** cell size of the box along x and y axes. Plan to use in lot of cases.
 * Width of the road(one lane).
 * Size of the vehicle sprite with safe empty pixels around.
 * Distance between vehicles destination coordinates in the line,
 * before vehicle starts move cross road
 * */
pub const CS:u16 = 96;

/**how many meters in one cell
 * let it be 4 meters(24px per meter). For statistic needs
 */
pub const CM:u16 = 4;

/** pixels per second, slow speed of the vehicle */
pub const PXS:f32 = 96.0;

