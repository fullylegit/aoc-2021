use anyhow::{anyhow, Error};
use std::collections::HashMap;

fn main() {
    println!(
        "Part 1 answer: {}",
        include_str!("../../inputs/day8")
            .split('\n')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|s| Line::from_str(s).ok())
            .map(|l| l.count_simple_digits_in_output())
            .sum::<usize>()
    );
    println!(
        "Part 2 answer: {}",
        include_str!("../../inputs/day8")
            .split('\n')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|s| Line::from_str(s).ok())
            .map(|mut l| l.output())
            .sum::<usize>()
    );
}

#[derive(Debug, Eq, Hash, Default, Clone, Copy)]
struct Mapping<'a> {
    string: &'a str,
}

impl PartialEq<str> for Mapping<'_> {
    fn eq(&self, other: &str) -> bool {
        self.string.len() == other.len() && self.string.chars().all(|c| other.contains(c))
    }
}

impl PartialEq<Mapping<'_>> for str {
    fn eq(&self, other: &Mapping) -> bool {
        self.len() == other.string.len() && self.chars().all(|c| other.string.contains(c))
    }
}

impl PartialEq<Mapping<'_>> for &str {
    fn eq(&self, other: &Mapping) -> bool {
        self.len() == other.string.len() && self.chars().all(|c| other.string.contains(c))
    }
}

impl PartialEq for Mapping<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.string.len() == other.string.len()
            && self.string.chars().all(|c| other.string.contains(c))
    }
}

impl<'a> From<&'a str> for Mapping<'a> {
    fn from(string: &'a str) -> Self {
        Self { string }
    }
}

impl<'a> From<Mapping<'a>> for &'a str {
    fn from(mapping: Mapping<'a>) -> Self {
        mapping.string
    }
}

impl<'a> Mapping<'a> {
    fn contains(&self, other: &Mapping) -> bool {
        other.string.chars().all(|c| self.string.contains(c))
    }

    fn contains_partial(&self, other: &Mapping, num: usize) -> bool {
        self.string
            .chars()
            .filter(|c| other.string.contains(*c))
            .count()
            == num
    }
}

struct Line<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
    mapping: HashMap<Mapping<'a>, usize>,
}

impl<'a> Line<'a> {
    fn from_str(s: &'a str) -> Result<Self, Error> {
        match s.split_once(" | ") {
            Some((input, output)) => Ok(Self {
                input: input.split(' ').collect(),
                output: output.split(' ').collect(),
                mapping: Default::default(),
            }),
            None => Err(anyhow!("missing |")),
        }
    }

    fn count_simple_digits_in_output(&self) -> usize {
        self.output
            .iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count()
    }

    fn reverse_mapping(&self, num: usize) -> Option<Mapping<'_>> {
        self.mapping
            .iter()
            .find(|(_k, v)| **v == num)
            .map(|(k, _v)| *k)
    }

    fn compute_mapping(&mut self) {
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .find(|s| s.len() == 2)
        {
            self.mapping.insert((*s).into(), 1);
        }
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .find(|s| s.len() == 3)
        {
            self.mapping.insert((*s).into(), 7);
        }
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .find(|s| s.len() == 4)
        {
            self.mapping.insert((*s).into(), 4);
        }
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .find(|s| s.len() == 7)
        {
            self.mapping.insert((*s).into(), 8);
        }

        let one = self.reverse_mapping(1).unwrap_or_default();
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .filter(|s| s.len() == 5)
            .map(|s| Mapping::from(*s))
            .find(|s| s.contains(&one))
        {
            self.mapping.insert(s, 3);
        }

        let three = self.reverse_mapping(3).unwrap_or_default();
        let four = self.reverse_mapping(4).unwrap_or_default();
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .filter(|s| s.len() == 5 && **s != three)
            .map(|s| Mapping::from(*s))
            .find(|s| s.contains_partial(&four, 2))
        {
            self.mapping.insert(s, 2);
        }

        let three = self.reverse_mapping(3).unwrap_or_default();
        let four = self.reverse_mapping(4).unwrap_or_default();
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .filter(|s| s.len() == 5 && **s != three)
            .map(|s| Mapping::from(*s))
            .find(|s| s.contains_partial(&four, 3))
        {
            self.mapping.insert(s, 5);
        }

        let seven = self.reverse_mapping(7).unwrap_or_default();
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .filter(|s| s.len() == 6)
            .map(|s| Mapping::from(*s))
            .find(|s| s.contains_partial(&seven, 2))
        {
            self.mapping.insert(s, 6);
        }

        let five = self.reverse_mapping(5).unwrap_or_default();
        let six = self.reverse_mapping(6).unwrap_or_default();
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .filter(|s| s.len() == 6 && **s != six)
            .map(|s| Mapping::from(*s))
            .find(|s| s.contains(&five))
        {
            self.mapping.insert(s, 9);
        }

        let six = self.reverse_mapping(6).unwrap_or_default();
        let nine = self.reverse_mapping(9).unwrap_or_default();
        if let Some(s) = self
            .input
            .iter()
            .chain(self.output.iter())
            .filter(|s| s.len() == 6 && **s != six && **s != nine)
            .map(|s| Mapping::from(*s))
            .next()
        {
            self.mapping.insert(s, 0);
        }
    }

    fn get_mapping_by_str(&self, s: &str) -> Option<&usize> {
        self.mapping.iter().find(|(k, _v)| *k == s).map(|(_k, v)| v)
    }

    fn output(&mut self) -> usize {
        self.compute_mapping();
        self.output
            .iter()
            .zip((0..self.output.len() as u32).rev())
            .map(|(s, exp)| {
                self.get_mapping_by_str(s).copied().unwrap_or_default() * (10usize.pow(exp))
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_count_simple_digits_in_output() {
        let expected = 26;
        let actual = TEST_INPUT
            .iter()
            .map(|s| Line::from_str(s).unwrap())
            .map(|l| l.count_simple_digits_in_output())
            .sum::<usize>();
        assert_eq!(expected, actual);
    }

    #[test_case(0, 8394)]
    #[test_case(1, 9781)]
    #[test_case(2, 1197)]
    #[test_case(3, 9361)]
    #[test_case(4, 4873)]
    #[test_case(5, 8418)]
    #[test_case(6, 4548)]
    #[test_case(7, 1625)]
    #[test_case(8, 8717)]
    #[test_case(9, 4315)]
    fn test_decoding_digits(idx: usize, expected: usize) {
        let actual = Line::from_str(TEST_INPUT[idx]).unwrap().output();
        assert_eq!(expected, actual);
    }
}
