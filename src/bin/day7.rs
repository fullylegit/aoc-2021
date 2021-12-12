use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    println!(
        "Part 1 answer: {}",
        include_str!("../../inputs/day7")
            .parse::<Swarm>()
            .expect("infallible")
            .fuel_to_align_at_least_cost_position_constant()
    );
    println!(
        "Part 2 answer: {}",
        include_str!("../../inputs/day7")
            .parse::<Swarm>()
            .expect("infallible")
            .fuel_to_align_at_least_cost_position_linear()
    );
}

struct Swarm {
    crabs: Vec<isize>,
}

impl FromStr for Swarm {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            crabs: s
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<isize>().ok())
                .collect(),
        })
    }
}

impl Swarm {
    fn fuel_to_align_constant(&self, position: isize) -> isize {
        self.crabs
            .iter()
            .map(|crab_pos| (position - crab_pos).abs())
            .sum()
    }

    fn least_fuel_alignment_position_constant(&self) -> isize {
        let max = self.crabs.iter().max().copied().unwrap_or(0);
        (0..max)
            .map(|pos| (pos, self.fuel_to_align_constant(pos)))
            .min_by_key(|(_pos, fuel)| *fuel)
            .unwrap_or((0, 0))
            .0
    }

    fn fuel_to_align_at_least_cost_position_constant(&self) -> isize {
        self.fuel_to_align_constant(self.least_fuel_alignment_position_constant())
    }

    fn fuel_to_align_linear(&self, position: isize) -> isize {
        self.crabs
            .iter()
            .map(|crab_pos| factorial((position - crab_pos).abs()))
            .sum()
    }

    fn least_fuel_alignment_position_linear(&self) -> isize {
        let max = self.crabs.iter().max().copied().unwrap_or(0);
        (0..max)
            .map(|pos| (pos, self.fuel_to_align_linear(pos)))
            .min_by_key(|(_pos, fuel)| *fuel)
            .unwrap_or((0, 0))
            .0
    }

    fn fuel_to_align_at_least_cost_position_linear(&self) -> isize {
        self.fuel_to_align_linear(self.least_fuel_alignment_position_linear())
    }
}

fn factorial(num: isize) -> isize {
    if num > 0 {
        num + factorial(num - 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_fuel_to_align_constant() {
        let expected = 37;
        let actual = TEST_INPUT
            .parse::<Swarm>()
            .unwrap()
            .fuel_to_align_constant(2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_least_fuel_alignment_position_constant() {
        let expected = 2;
        let actual = TEST_INPUT
            .parse::<Swarm>()
            .unwrap()
            .least_fuel_alignment_position_constant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fuel_to_align_linear() {
        let expected = 168;
        let actual = TEST_INPUT.parse::<Swarm>().unwrap().fuel_to_align_linear(5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_least_fuel_alignment_position_linear() {
        let expected = 5;
        let actual = TEST_INPUT
            .parse::<Swarm>()
            .unwrap()
            .least_fuel_alignment_position_linear();
        assert_eq!(expected, actual);
    }
}
