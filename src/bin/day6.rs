use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day6");
    println!(
        "Part 1 answer: {}",
        input
            .parse::<School>()
            .expect("infallible")
            .advance_days(80)
            .count()
    );
    println!(
        "Part 2 answer: {}",
        input
            .parse::<School>()
            .expect("infallible")
            .advance_days(256)
            .count()
    );
}

struct School {
    fish_timers: HashMap<usize, usize>,
}

impl FromStr for School {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            fish_timers: s
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<usize>().ok())
                .fold(HashMap::<usize, usize>::new(), |mut acc, timer| {
                    if let Some(timer) = acc.get_mut(&timer) {
                        *timer += 1;
                    } else {
                        acc.insert(timer, 1);
                    }
                    acc
                }),
        })
    }
}

impl School {
    fn advance_days(&mut self, days: usize) -> &mut Self {
        (0..days).for_each(|_day| self.advance_day());
        self
    }

    fn advance_day(&mut self) {
        let t0 = self.fish_timers.remove(&0).unwrap_or(0);
        for timer in 1..=8 {
            if let Some(num_fish) = self.fish_timers.remove(&timer) {
                self.fish_timers.insert(timer - 1, num_fish);
            }
        }
        self.fish_timers
            .insert(6, self.fish_timers.get(&6).copied().unwrap_or(0) + t0);
        // babies
        self.fish_timers.insert(8, t0);
    }

    fn count(&self) -> usize {
        self.fish_timers.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn eighteen_days() {
        let expected = 26;
        let actual = TEST_INPUT
            .parse::<School>()
            .unwrap()
            .advance_days(18)
            .count();
        assert_eq!(expected, actual);
    }

    #[test]
    fn eighty_days() {
        let expected = 5934;
        let actual = TEST_INPUT
            .parse::<School>()
            .unwrap()
            .advance_days(80)
            .count();
        assert_eq!(expected, actual);
    }

    #[test]
    fn two_fifty_six_days() {
        let expected = 26_984_457_539;
        let actual = TEST_INPUT
            .parse::<School>()
            .unwrap()
            .advance_days(256)
            .count();
        assert_eq!(expected, actual);
    }
}
