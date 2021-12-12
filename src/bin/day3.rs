fn main() {
    let input = include_str!("../../inputs/day3");
    let diagnostics = input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    println!(
        "Part 1 answer: {}",
        calculate_epsilon_rate(&diagnostics) * calculate_gamma_rate(&diagnostics)
    );
    println!(
        "Part 2 answer: {}",
        calculate_oxygen_generator_rating(&diagnostics)
            * calculate_co2_scrubber_rating(&diagnostics)
    );
}

enum BinaryNumber {
    Zero,
    One,
}

#[derive(Debug, Default, Clone, Copy)]
struct Column {
    zeros: usize,
    ones: usize,
}

impl Column {
    fn with_initial_value(c: char) -> Self {
        let mut this = Self::default();
        this.add(c);
        this
    }

    fn most_common(&self) -> BinaryNumber {
        if self.zeros > self.ones {
            BinaryNumber::Zero
        } else {
            BinaryNumber::One
        }
    }

    fn least_common(&self) -> BinaryNumber {
        if self.zeros > self.ones {
            BinaryNumber::One
        } else {
            BinaryNumber::Zero
        }
    }

    fn add(&mut self, c: char) {
        if c == '0' {
            self.zeros += 1;
        } else if c == '1' {
            self.ones += 1;
        }
    }
}

fn parse(input: &[&str]) -> Vec<Column> {
    let mut columns = Vec::<Column>::new();
    input.iter().for_each(|input| {
        input
            .char_indices()
            .for_each(|(idx, c)| match columns.get_mut(idx) {
                Some(col) => col.add(c),
                None => columns.insert(idx, Column::with_initial_value(c)),
            })
    });
    columns
}

fn parse_binary(input: &str) -> usize {
    input.chars().fold(0, |acc, item| match item {
        '0' => acc << 1,
        '1' => (acc << 1) | 1,
        _ => acc,
    })
}

fn calculate_gamma_rate(input: &[&str]) -> usize {
    parse(input)
        .iter()
        .fold(0, |acc, item| match item.most_common() {
            BinaryNumber::Zero => acc << 1,
            BinaryNumber::One => (acc << 1) | 1,
        })
}

fn calculate_epsilon_rate(input: &[&str]) -> usize {
    parse(input)
        .iter()
        .fold(0, |acc, item| match item.least_common() {
            BinaryNumber::Zero => acc << 1,
            BinaryNumber::One => (acc << 1) | 1,
        })
}

fn calculate_oxygen_generator_rating(input: &[&str]) -> usize {
    let mut parsed: Vec<Column>;
    let mut input = input.to_vec();
    for idx in 0..input.get(0).map(|s| s.len()).unwrap_or_default() {
        parsed = parse(&input);
        if parsed[idx].ones >= parsed[idx].zeros {
            input = input
                .into_iter()
                .filter(|s| s.chars().nth(idx).unwrap_or_default() == '1')
                .collect::<Vec<_>>();
        } else {
            input = input
                .into_iter()
                .filter(|s| s.chars().nth(idx).unwrap_or_default() == '0')
                .collect::<Vec<_>>();
        }
        if input.len() == 1 {
            return parse_binary(input.into_iter().next().unwrap_or_default());
        }
    }
    0
}

fn calculate_co2_scrubber_rating(input: &[&str]) -> usize {
    let mut parsed: Vec<Column>;
    let mut input = input.to_vec();
    for idx in 0..input.get(0).map(|s| s.len()).unwrap_or_default() {
        parsed = parse(&input);
        if parsed[idx].zeros <= parsed[idx].ones {
            input = input
                .into_iter()
                .filter(|s| s.chars().nth(idx).unwrap_or_default() == '0')
                .collect::<Vec<_>>();
        } else {
            input = input
                .into_iter()
                .filter(|s| s.chars().nth(idx).unwrap_or_default() == '1')
                .collect::<Vec<_>>();
        }
        if input.len() == 1 {
            return parse_binary(input.into_iter().next().unwrap_or_default());
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_gamma() {
        let expected = 22;
        let actual = calculate_gamma_rate(INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_epsilon() {
        let expected = 9;
        let actual = calculate_epsilon_rate(INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_oxygen() {
        let expected = 23;
        let actual = calculate_oxygen_generator_rating(INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scrubber() {
        let expected = 10;
        let actual = calculate_co2_scrubber_rating(INPUT);
        assert_eq!(expected, actual);
    }
}
