use anyhow::{anyhow, Error};
use either::Either;
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day5");
    let input = input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|input| input.parse::<Line>().ok())
        .collect::<Vec<_>>();
    println!(
        "Part 1 answer: {}",
        count_dangerous_points(
            &input
                .iter()
                .copied()
                .filter(|line| line.horizontal() || line.vertical())
                .collect::<Vec<_>>()
        )
    );
    println!("Part 2 answer: {}", count_dangerous_points(&input));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x, y)) => Ok(Point {
                x: x.parse()?,
                y: y.parse()?,
            }),
            None => Err(anyhow!("missing ,")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" -> ") {
            Some((start_point, end_point)) => Ok(Line {
                start: start_point.parse()?,
                end: end_point.parse()?,
            }),
            None => Err(anyhow!("missing ->")),
        }
    }
}

impl Line {
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn covers_points(&self) -> Vec<Point> {
        if self.horizontal() || self.vertical() {
            let mut points = vec![];
            for x in self.start.x.min(self.end.x)..=self.end.x.max(self.start.x) {
                for y in self.start.y.min(self.end.y)..=self.end.y.max(self.start.y) {
                    points.push(Point { x, y })
                }
            }
            points
        } else {
            // (unenforced) invariant: diagonal lines are always 45deg
            let x_iter = if self.start.x <= self.end.x {
                Either::Left(self.start.x..=self.end.x)
            } else {
                Either::Right((self.end.x..=self.start.x).rev())
            };
            let y_iter = if self.start.y <= self.end.y {
                Either::Left(self.start.y..=self.end.y)
            } else {
                Either::Right((self.end.y..=self.start.y).rev())
            };
            x_iter.zip(y_iter).map(|(x, y)| Point { x, y }).collect()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    points: Vec<Point>,
}

impl FromIterator<Point> for Map {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Self {
            points: iter.into_iter().collect(),
        }
    }
}

impl Map {
    fn count_points_covered(&self, threshold: usize) -> usize {
        self.points
            .iter()
            .into_group_map_by(|point| *point)
            .iter()
            .filter(|(_k, v)| v.len() >= threshold)
            .count()
    }
}

fn count_dangerous_points(input: &[Line]) -> usize {
    input
        .iter()
        .flat_map(|line| line.covers_points())
        .collect::<Map>()
        .count_points_covered(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_any_order {
        ($left:expr, $right:expr) => {
            assert_eq!($left.len(), $right.len());
            assert!($left
                .iter()
                .all(|left| $right.iter().any(|right| left == right)));
        };
    }

    const TEST_INPUT: &[&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    impl Line {
        fn from_coords(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
            Line {
                start: Point { x: x1, y: y1 },
                end: Point { x: x2, y: y2 },
            }
        }
    }

    #[test]
    fn test_parsing() {
        let expected = [
            Line::from_coords(0, 9, 5, 9),
            Line::from_coords(8, 0, 0, 8),
            Line::from_coords(9, 4, 3, 4),
            Line::from_coords(2, 2, 2, 1),
            Line::from_coords(7, 0, 7, 4),
            Line::from_coords(6, 4, 2, 0),
            Line::from_coords(0, 9, 2, 9),
            Line::from_coords(3, 4, 1, 4),
            Line::from_coords(0, 0, 8, 8),
            Line::from_coords(5, 5, 8, 2),
        ];
        let actual = TEST_INPUT
            .iter()
            .filter_map(|input| input.parse::<Line>().ok())
            .collect::<Vec<_>>();
        assert_eq!(&expected[..], &actual);
    }

    #[test]
    fn covers_points_horizontal() {
        let input = TEST_INPUT[0].parse::<Line>().unwrap();
        assert!(input.horizontal());
        let expected = (0..=5).map(|x| Point { x, y: 9 }).collect::<Vec<_>>();
        let actual = input.covers_points();
        assert_eq!(expected, actual);
    }

    #[test]
    fn covers_points_vertical() {
        let input = TEST_INPUT[4].parse::<Line>().unwrap();
        assert!(input.vertical());
        let expected = (0..=4).map(|y| Point { x: 7, y }).collect::<Vec<_>>();
        let actual = input.covers_points();
        assert_eq!(expected, actual);
    }

    #[test]
    fn covers_points_diagonal() {
        let input = TEST_INPUT[5].parse::<Line>().unwrap();
        let expected = (2..=6)
            .rev()
            .zip((0..=4).rev())
            .map(|(x, y)| Point { x, y })
            .collect::<Vec<_>>();
        let actual = input.covers_points();
        assert_eq_any_order!(expected, actual);
    }

    #[test]
    fn covers_points_backwards_horizontal() {
        let input = TEST_INPUT[2].parse::<Line>().unwrap();
        assert!(input.horizontal());
        let expected = (3..=9).map(|x| Point { x, y: 4 }).collect::<Vec<_>>();
        let actual = input.covers_points();
        assert_eq_any_order!(expected, actual);
    }

    #[test]
    fn covers_points_backwards_vertical() {
        let input = TEST_INPUT[3].parse::<Line>().unwrap();
        assert!(input.vertical());
        let expected = (1..=2).map(|y| Point { x: 2, y }).collect::<Vec<_>>();
        let actual = input.covers_points();
        assert_eq_any_order!(expected, actual);
    }

    #[test]
    fn dangerous_points_horizontal_vertical() {
        let expected = 5;
        let actual = count_dangerous_points(
            &TEST_INPUT
                .iter()
                .filter_map(|input| input.parse::<Line>().ok())
                .filter(|line| line.horizontal() || line.vertical())
                .collect::<Vec<_>>(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn dangerous_points() {
        let expected = 12;
        let actual = count_dangerous_points(
            &TEST_INPUT
                .iter()
                .filter_map(|input| input.parse::<Line>().ok())
                .collect::<Vec<_>>(),
        );
        assert_eq!(expected, actual);
    }
}
