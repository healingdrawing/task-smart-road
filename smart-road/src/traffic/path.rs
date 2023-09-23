use crate::config::{CAR_PADDING, STRAIGHT_LENGTH, WINDOW_SIZE};
use crate::traffic::curve::quadratic_curve;
use crate::traffic::{To, Turn};
use macroquad::math::Vec2;
use std::ops::{Mul, Sub};

#[derive(Debug)]
pub struct Path {
    pub to: To,
    pub turn: Turn,

    points: Vec<Vec2>,
}

//                         North
//                     |  ↓  |  ↑  |
//                     |  ↓  |  ↑  |
//                     |     |     |
//                     |     |     |
//                     |     |     |
//                     |     |     |
//      _______________|     |     |_______________
//      ← ←                                     ← ←
// East ---------------             --------------- West
//      A →            B                        → →
//      _______________    C        _______________
//                     |   | |     |
//                     |   | |     |
//                     |   | |     |
//                     |   | |     |
//                     |   | |     |
//                     |   ↓ |  ↑  |
//                     |   D |  ↑  |
//                         South
// Path(East, Right) =  [A -> B -> C -> D]
// A - border_point(East, true)
// D - border_point(South, false)
// B - straight_point(East, A, STRAIGHT_LENGTH)
// C - straight_point(South, D, STRAIGHT_LENGTH)

//todo need refactor, because x3 roads now
/// Returns the point on the border where the car should appear or disappear
fn border_point(to: To, right_side: bool) -> Vec2 {
    let car_padding = if right_side {
        CAR_PADDING
    } else {
        -CAR_PADDING
    };
    match to {
        To::N => Vec2::new(WINDOW_SIZE as f32 / 2.0 - car_padding, 0.0),
        To::E => Vec2::new(WINDOW_SIZE as f32, WINDOW_SIZE as f32 / 2.0 - car_padding),
        To::S => Vec2::new(WINDOW_SIZE as f32 / 2.0 + car_padding, WINDOW_SIZE as f32),
        To::W => Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 + car_padding),
    }
}

/// Returns the point in center associated with the border point
fn straight_point(direction: To, border_point: Vec2) -> Vec2 {
    match direction {
        To::N => Vec2::new(border_point.x, border_point.y + STRAIGHT_LENGTH),
        To::E => Vec2::new(border_point.x - STRAIGHT_LENGTH, border_point.y),
        To::S => Vec2::new(border_point.x, border_point.y - STRAIGHT_LENGTH),
        To::W => Vec2::new(border_point.x + STRAIGHT_LENGTH, border_point.y),
    }
}

impl Path {
    pub fn new(going_to: To, turn: Turn) -> Self {
        let destination = going_to.destination(turn);

        let start_point = border_point(going_to, true);
        let end_point = border_point(destination, false);

        match turn {
            Turn::No => Self {
                to: going_to,
                turn,

                points: vec![start_point, end_point],
            },
            Turn::L | Turn::R => {
                let curve_start_point = straight_point(going_to, start_point);
                let curve_end_point = straight_point(destination, end_point);

                let center = Vec2::new(WINDOW_SIZE as f32 / 2.0, WINDOW_SIZE as f32 / 2.0);

                let control_point = match turn {
                    Turn::L => center,
                    Turn::R => {
                        // make curve radius smaller by half

                        // vector between curve_start_point and curve_end_point
                        let line = curve_start_point.sub(curve_end_point);

                        // perpendicular vector from center to line
                        let radial_vector = Vec2::new(-line.y, line.x);

                        center.sub(radial_vector.mul(0.5))
                    }
                    _ => unreachable!(),
                };

                let curve = quadratic_curve(curve_start_point, control_point, curve_end_point);

                Self {
                    to: going_to,
                    turn,

                    points: [start_point, curve_start_point]
                        .into_iter()
                        .chain(curve)
                        .chain([curve_end_point, end_point])
                        .collect(),
                }
            }
        }
    }

    pub fn points(&self) -> &Vec<Vec2> {
        &self.points
    }

    pub fn point(&self, index: usize) -> Option<Vec2> {
        self.points.get(index).copied()
    }
}
