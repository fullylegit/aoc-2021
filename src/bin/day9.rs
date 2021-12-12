use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    println!(
        "Part 1 answer: {}",
        include_str!("../../inputs/day9")
            .parse::<Heightmap>()
            .unwrap()
            .total_risk()
    );
    println!(
        "Part 2 answer: {}",
        include_str!("../../inputs/day9")
            .parse::<Heightmap>()
            .unwrap()
            .basin_score()
    );
}

#[derive(Debug, Default)]
struct Basin {
    points: Vec<(usize, usize)>,
}

impl Basin {
    fn size(&self) -> usize {
        self.points.len()
    }

    fn add_point(&mut self, x: usize, y: usize) -> &mut Self {
        if !self.points.contains(&(x, y)) {
            self.points.push((x, y));
        }
        self
    }

    fn add_points(&mut self, points: &[(usize, usize)]) -> &mut Self {
        points.iter().for_each(|(x, y)| {
            self.add_point(*x, *y);
        });
        self
    }
}

#[derive(Debug)]
struct Heightmap {
    positions: Vec<Vec<usize>>,
}

impl FromStr for Heightmap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            positions: s
                .split('\n')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.chars()
                        .map(|c| c as usize - '0' as usize)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        })
    }
}

impl Heightmap {
    fn total_risk(&self) -> usize {
        self.holes()
            .into_iter()
            .filter_map(|(x, y)| self.positions.get(x).map(|row| row.get(y)).flatten())
            .map(|height| *height + 1)
            .sum()
    }

    fn holes(&self) -> Vec<(usize, usize)> {
        (0..self.positions.len())
            .cartesian_product(0..self.positions.get(0).map(Vec::len).unwrap_or_default())
            .filter(|(x, y)| self.is_hole(*x, *y))
            .collect()
    }

    fn is_hole(&self, x: usize, y: usize) -> bool {
        let height = match self.positions.get(x).map(|row| row.get(y)).flatten() {
            Some(height) => *height,
            None => panic!("out of bounds"),
        };
        let height_above = if x > 0 {
            self.positions
                .get(x - 1)
                .map(|row| row.get(y))
                .flatten()
                .copied()
        } else {
            None
        };
        let height_below = if x < self.positions.len() - 1 {
            self.positions
                .get(x + 1)
                .map(|row| row.get(y))
                .flatten()
                .copied()
        } else {
            None
        };
        let height_left = self
            .positions
            .get(x)
            .map(|row| if y > 0 { row.get(y - 1) } else { None })
            .flatten()
            .copied();
        let height_right = self
            .positions
            .get(x)
            .map(|row| {
                if y < row.len() - 1 {
                    row.get(y + 1)
                } else {
                    None
                }
            })
            .flatten()
            .copied();

        height < height_above.unwrap_or(usize::MAX)
            && height < height_below.unwrap_or(usize::MAX)
            && height < height_left.unwrap_or(usize::MAX)
            && height < height_right.unwrap_or(usize::MAX)
    }

    fn adjacent_basin_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut points = vec![];

        // above
        if x > 0 {
            if let Some(height) = self.positions.get(x - 1).map(|row| row.get(y)).flatten() {
                if *height != 9 {
                    points.push((x - 1, y));
                }
            }
        }

        // below
        if x < self.positions.len() - 1 {
            if let Some(height) = self.positions.get(x + 1).map(|row| row.get(y)).flatten() {
                if *height != 9 {
                    points.push((x + 1, y));
                }
            }
        }

        // left
        if y > 0 {
            if let Some(height) = self.positions.get(x).map(|row| row.get(y - 1)).flatten() {
                if *height != 9 {
                    points.push((x, y - 1));
                }
            }
        }

        // right
        if y < self
            .positions
            .get(x)
            .map(|row| row.len() - 1)
            .unwrap_or_default()
        {
            if let Some(height) = self.positions.get(x).map(|row| row.get(y + 1)).flatten() {
                if *height != 9 {
                    points.push((x, y + 1));
                }
            }
        }

        points
    }

    fn basins(&self) -> Vec<Basin> {
        let mut basins = vec![];
        for (x, y) in self.holes() {
            let mut basin = Basin::default();
            basin.add_point(x, y);
            // this is a super lazy way to solve this problem
            loop {
                let size_before = basin.size();

                let mut new_points = vec![];
                for (x, y) in basin.points.iter() {
                    new_points.extend(self.adjacent_basin_points(*x, *y));
                }
                basin.add_points(&new_points);

                if basin.size() == size_before {
                    break;
                }
            }
            basins.push(basin);
        }
        basins
    }

    fn basin_score(&self) -> usize {
        let mut basins = self.basins();
        basins.sort_by_key(|b| b.size());
        basins.reverse();
        basins[..3.min(basins.len())]
            .iter()
            .map(Basin::size)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn test_risk() {
        let expected = 15;
        let actual = TEST_INPUT.parse::<Heightmap>().unwrap().total_risk();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_basins() {
        let expected = 1134;
        let actual = TEST_INPUT.parse::<Heightmap>().unwrap().basin_score();
        assert_eq!(expected, actual);
    }
}
