use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

mod movement;
mod ship;

use movement::Movement;
use ship::{BasicShip, Ship, WaypointedShip};

#[derive(Debug)]
pub struct Solution {
    movements: Vec<Movement>,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let movements = parse_file(input_file)?;

        Ok(Self { movements })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        12
    }

    fn solve(&self, part: Part) -> String {
        let mut basic = BasicShip::new();
        let mut waypointed = WaypointedShip::new();

        let ship: &mut dyn Ship = match part {
            Part::One => &mut basic,
            Part::Two => &mut waypointed,
        };

        for movement in &self.movements {
            ship.process_movement(movement);
        }

        format!("ship distance from zero is: {}", ship.manhattan_distance())
    }
}
