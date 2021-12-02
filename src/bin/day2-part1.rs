use anyhow::{bail, Error};
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day2");
    let course = input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<Movement>().ok())
        .collect::<Vec<_>>();
    println!(
        "Answer: {}",
        Position::default().apply_course(&course).product()
    );
}

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("forward ") {
            return Ok(Movement::Forward(num.parse()?));
        }
        if let Some(num) = s.strip_prefix("down ") {
            return Ok(Movement::Down(num.parse()?));
        }
        if let Some(num) = s.strip_prefix("up ") {
            return Ok(Movement::Up(num.parse()?));
        }
        bail!("Invalid input: {:?}", s);
    }
}

#[derive(Debug, Default, PartialEq)]
struct Position {
    horizontal: isize,
    depth: isize,
}

impl Position {
    fn apply_course(mut self, course: &[Movement]) -> Self {
        course.iter().for_each(|movement| self.apply_move(movement));
        self
    }

    fn apply_move(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(num) => self.horizontal += num,
            Movement::Down(num) => self.depth += num,
            Movement::Up(num) => self.depth -= num,
        }
    }

    fn product(&self) -> isize {
        self.horizontal * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = &[
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let expected = &[
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];
        let actual = input
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        assert_eq!(&expected[..], &actual);
    }

    #[test]
    fn test_movement() {
        let input = &[
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];
        let expected = Position {
            horizontal: 15,
            depth: 10,
        };
        let actual = Position::default().apply_course(input);
        assert_eq!(expected, actual);
    }
}
