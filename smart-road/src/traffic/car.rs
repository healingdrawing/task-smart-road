use crate::config::{CAR_LENGTH, CAR_SAFE_DISTANCE, CAR_SPEED, STRAIGHT_LENGTH, WINDOW_SIZE};
use crate::traffic::{Light, Path};
use macroquad::math::Vec2;
use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum To {
    /** North*/ N,
    /** East */ E,
    /** South*/ S,
    /** West */ W,
}

impl To {
    pub fn destination(&self, turn: Turn) -> To {
        match (self, turn) {
            (To::N, Turn::No) => To::N,
            (To::N, Turn::L) => To::W,
            (To::N, Turn::R) => To::E,

            (To::E, Turn::No) => To::E,
            (To::E, Turn::L) => To::N,
            (To::E, Turn::R) => To::S,

            (To::S, Turn::No) => To::S,
            (To::S, Turn::L) => To::E,
            (To::S, Turn::R) => To::W,

            (To::W, Turn::No) => To::W,
            (To::W, Turn::L) => To::S,
            (To::W, Turn::R) => To::N,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Turn {
    No,
    R,
    L,
}

pub const TURN: [Turn; 3] = [Turn::No, Turn::R, Turn::L];

#[derive(Debug)]
pub struct Car {
    pub going_to: To,
    pub turn: Turn,

    /// Position of the front left corner of the car
    pub pos: Vec2,

    /// Rotation of the car in radians, 0 is facing right
    pub rotation: f32,

    point_index: usize,
}

impl Car {
    pub fn new(going_to: To) -> Car {
        let turn = TURN[rand::thread_rng().gen_range(0..TURN.len())];

        let path = Path::new(going_to, turn);

        let first_point = path.point(0).unwrap();

        Self {
            going_to,
            turn,
            point_index: 0,

            pos: first_point,
            rotation: 0.0,
        }
    }

    pub fn is_in_queue(&self) -> bool {
        self.point_index == 0
    }

    pub fn border_distance(&self) -> f32 {
        match self.going_to {
            To::N => self.pos.y,
            To::E => WINDOW_SIZE as f32 - self.pos.x,
            To::S => WINDOW_SIZE as f32 - self.pos.y,
            To::W => self.pos.x,
        }
    }

    pub fn update(&mut self, path: &Path, next_car: Option<&Car>, light: &Light) {
        let next_point = path.point(self.point_index + 1);

        if next_point.is_none() {
            return;
        }

        if let Some(next_car) = next_car {
            let distance = (next_car.pos - self.pos).length() - CAR_LENGTH;

            if distance < CAR_SAFE_DISTANCE {
                return;
            }
        }

        let vector = next_point.unwrap() - self.pos;

        if *light == Light::Red && self.is_in_queue()
        // the car is at the first straight part of the path
        {
            // TODO: refactor
            let border_distance = self.border_distance();
            if border_distance < STRAIGHT_LENGTH
                && border_distance + CAR_SPEED * 1.0 + CAR_SAFE_DISTANCE > STRAIGHT_LENGTH
            {
                return;
            }
        }

        if vector.length() < CAR_SPEED * 1.0 {
            self.point_index += 1;
            self.update(path, next_car, light);
            return;
        }

        self.rotation = vector.y.atan2(vector.x);

        let vector = vector.normalize();

        self.pos += vector * CAR_SPEED;
    }

    pub fn is_done(&self, path: &Path) -> bool {
        path.point(self.point_index + 1).is_none()
    }
}