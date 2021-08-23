use crate::Direction::*;
use crate::Turn::*;

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
impl Direction {
    fn adjust(&mut self, turn: Turn, degrees: u32) {
        let direction = self;
        if turn == Right {
            for _ in 0..degrees / 90 {
                *direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
            }
        } else {
            for _ in 0..degrees / 90 {
                *direction = match direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                };
            }
        }
    }
}

struct Ship {
    x: i32,
    y: i32,
    facing: Direction,
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
        match self.facing {
            Direction::North => self.y += dist,
            Direction::South => self.y -= dist,
            Direction::East => self.x += dist,
            Direction::West => self.x -= dist,
        }
    }
    fn turn(&mut self, turn: Turn, degrees: u32) {
        if degrees % 90 != 0 {
            panic!("degrees must be a multiple of 90");
        };
        self.facing.adjust(turn, degrees);
    }
    fn new(facing: Direction) -> Ship {
        Ship {
            x: 0,
            y: 0,
            facing,
        }
    }
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn execute(self, command: &str) {
    }
}

fn main() {
    println!("Hello, world!");
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
        assert_eq!(ship.facing, North);
    }
    #[test]
    fn test_turn_2() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 90);
        assert_eq!(ship.facing, East);
    }
    #[test]
    fn test_turn_3() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 180);
        assert_eq!(ship.facing, South);
    }
    #[test]
    fn test_turn_4() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 270);
        assert_eq!(ship.facing, West);
    }
    #[test]
    fn test_turn_5() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 360);
        assert_eq!(ship.facing, North);
    }
    #[test]
    #[should_panic]
    fn test_turn_6() {
        let mut ship = Ship::new(North);
        ship.turn(Right, 99);
    }
}
