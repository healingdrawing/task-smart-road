use crate::config::{CAR_LENGTH, CAR_SAFE_DISTANCE};
use crate::traffic::{Car, To, Turn, Path};

#[derive(Debug, Eq, PartialEq)]
pub enum Light {
    Red,
    Green,
}

#[derive(Debug)]
pub struct Line {
    pub coming_from: To,

    pub cars: Vec<Car>,
    pub light: Light,

    pub paths: [Path; 3],
}

fn get_path(paths: &[Path; 3], going: Turn) -> &Path {
    match going {
        Turn::No => &paths[0],
        Turn::L => &paths[1],
        Turn::R => &paths[2],
    }
}

impl Line {
    pub fn new(coming_from: To, light: Light) -> Self {
        Line {
            coming_from,
            light,

            cars: Vec::new(),

            paths: [
                Path::new(coming_from, Turn::No),
                Path::new(coming_from, Turn::L),
                Path::new(coming_from, Turn::R),
            ],
        }
    }

    pub fn switch(&mut self) {
        self.light = match self.light {
            Light::Red => Light::Green,
            Light::Green => Light::Red,
        }
    }

    pub fn update(&mut self) {
        let mut prev_car = None;

        for car in self.cars.iter_mut() {
            let path = get_path(&self.paths, car.turn);

            car.update(path, prev_car, &self.light);

            prev_car = Some(car);
        }

        self.cleanup_cars();
    }

    pub fn can_add_car(&self) -> bool {
        let prev_car = self
            .cars
            .iter()
            .rfind(|c| c.going_to == self.coming_from);

        if prev_car.is_none() {
            return true;
        }

        let prev_car = prev_car.unwrap();

        prev_car.border_distance() >= CAR_LENGTH + CAR_SAFE_DISTANCE
    }

    pub fn add_car(&mut self) {
        if !self.can_add_car() {
            return;
        }
        let car = Car::new(self.coming_from);

        self.cars.push(car);
    }

    pub fn cleanup_cars(&mut self) {
        self.cars.retain(|car| {
            let path = get_path(&self.paths, car.turn);
            !car.is_done(path)
        })
    }
}
