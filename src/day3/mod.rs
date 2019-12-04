use std;
use std::str::{FromStr, Utf8Error};
use std::num::ParseIntError;
use std::error::Error;

mod one;
//mod two;

enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

impl FromStr for Direction {
    type Err = Box<Error>;

    fn from_str(string: &str) -> Result<Direction, Box<Error>> {
        match string {
            "U" => Ok(Direction::UP),
            "L" => Ok(Direction::LEFT),
            "D" => Ok(Direction::DOWN),
            "R" => Ok(Direction::RIGHT),
            _ => panic!("Couldn't convert '{}' into Direction.", string)
        }
    }
}

struct Movement {
    direction: Direction,
    distance: i32
}

impl Movement {
    pub fn new(direction: Direction, distance: i32) -> Self {
        Movement {
            direction,
            distance
        }
    }

}

pub trait ToPoints {
    fn to_points(&self) -> Vec<(i32, i32)>;
}

impl ToPoints for Vec<Movement> {
    fn to_points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec!();

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        points.push((x,y));

        for movement in self.iter() {
            match movement.direction {
                Direction::UP => y += movement.distance,
                Direction::RIGHT => x += movement.distance,
                Direction::DOWN => y -= movement.distance,
                Direction::LEFT => x -= movement.distance,
            };

            points.push((x,y));
        }

        points
    }
}

impl FromStr for Movement {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = Direction::from_str(&s[0..1])?;
        let distance = s[1..].parse::<i32>()?;

        Ok(Movement::new( direction, distance ))
    }
}

fn get_input() -> Vec<Vec<Movement>> {
    let mut input: Vec<Vec<Movement>> = vec!();

    let bytes = include_bytes!("input.txt");
    let input_string = std::str::from_utf8(bytes).unwrap();

    for line in input_string.split("\n") {
        let mut wire: Vec<Movement> = vec!();
        for value in line.split(",") {
            let movement = value.parse::<Movement>().unwrap();
            wire.push(movement);
        }
        input.push(wire);
    }

    input
}
