use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Direction::*;
use crate::Turn::*;

fn main() {
    let file = File::open("./src/input.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut ship = Ship::new(East);

    reader.lines().map(Result::unwrap).for_each(|line| {
        ship.parse_command(line);
    });
    println!("{}", ship.get_manhattan_distance());

    println!("Hello, world!");
}

#[derive(PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Ship {
    x: i32,
    y: i32,
    bearing: VecDeque<Direction>,
}
struct Waypoint {
    x: i32,
    y: i32,
}
impl Ship {
    fn mv(&mut self, direction: Direction, dist: i32) {
        match direction {
            Direction::North => self.y += dist,
            Direction::South => self.y -= dist,
            Direction::East => self.x += dist,
            Direction::West => self.x -= dist,
        }
    }
    fn fwd(&mut self, dist: i32) {
        match self.is_facing() {
            Direction::North => self.y += dist,
            Direction::South => self.y -= dist,
            Direction::East => self.x += dist,
            Direction::West => self.x -= dist,
        }
    }
    fn is_facing(&self) -> &Direction {
        &self.bearing[0]
    }
    fn turn(&mut self, turn: Turn, degrees: u32) {
        if degrees % 90 != 0 {
            panic!("degrees must be a multiple of 90");
        };
        if turn == Right {
            self.bearing.rotate_left((degrees / 90) as usize);
        } else {
            self.bearing.rotate_right((degrees / 90) as usize);
        }
    }
    fn new(facing: Direction) -> Ship {
        let mut bearing = VecDeque::new();
        bearing.push_back(North);
        bearing.push_back(East);
        bearing.push_back(South);
        bearing.push_back(West);

        let mut direction = &bearing[0];
        while *direction != facing {
            bearing.rotate_left(1);
            direction = &bearing[0];
        }
        Ship {
            x: 0,
            y: 0,
            bearing,
        }
    }
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn get_manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn parse_command(&mut self, line: String) {
        let g = &line[0..1];
        let h = str::parse::<i32>(&line[1..line.len()]).unwrap();

        match g {
            "N" => self.mv(North, h),
            "S" => self.mv(South, h),
            "E" => self.mv(East, h),
            "W" => self.mv(West, h),
            "L" => self.turn(Left, h as u32),
            "R" => self.turn(Right, h as u32),
            "F" => self.fwd(h),
            _ => panic!("Invalid command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mv_1() {
        let mut ship = Ship::new(North);
        ship.mv(North, 111);
        assert_eq!(ship.get_pos(), (0, 111));
    }

    #[test]
    fn test_mv_2() {
        let mut ship = Ship::new(North);
        ship.mv(South, 111);
        assert_eq!(ship.get_pos(), (0, -111));
    }

    #[test]
    fn test_mv_3() {
        let mut ship = Ship::new(North);
        ship.mv(East, 111);
        assert_eq!(ship.get_pos(), (111, 0));
    }

    #[test]
    fn test_mv_4() {
        let mut ship = Ship::new(North);
        ship.mv(West, 111);
        assert_eq!(ship.get_pos(), (-111, 0));
    }

    #[test]
    fn test_turn_1() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 0);
        assert_eq!(ship.is_facing(), &North);
    }
    #[test]
    fn test_turn_2() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 90);
        assert_eq!(ship.is_facing(), &East);
    }
    #[test]
    fn test_turn_3() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 180);
        assert_eq!(ship.is_facing(), &South);
    }
    #[test]
    fn test_turn_4() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 270);
        assert_eq!(ship.is_facing(), &West);
    }
    #[test]
    fn test_turn_5() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 360);
        assert_eq!(ship.is_facing(), &North);
    }
    #[test]
    #[should_panic]
    fn test_turn_6() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 99);
    }
    #[test]
    fn test_case() {
        let mut ship = Ship::new(East);
        ship.fwd(10);
        assert_eq!(ship.get_pos(), (10, 0));
        ship.mv(North, 3);
        assert_eq!(ship.get_pos(), (10, 3));
        ship.fwd(7);
        assert_eq!(ship.get_pos(), (17, 3));
        ship.turn(Right, 90);
        assert_eq!(ship.get_pos(), (17, 3));
        ship.fwd(11);
        assert_eq!(ship.get_pos(), (17, -8));
        assert_eq!(ship.get_manhattan_distance(), 25);
    }
}
