use itertools::Itertools;

const OPENING_CHARS: &[char] = &['(', '{', '[', '<'];
const MATCHING_CHARS: &[(char, char)] = &[('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')];
const CORRUPT_CHAR_SCORES: &[(char, usize)] = &[(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
const INCOMPLETE_CHAR_SCORES: &[(char, usize)] = &[(')', 1), (']', 2), ('}', 3), ('>', 4)];

fn main() {
    println!(
        "Part 1 answer: {}",
        include_str!("../../inputs/day10")
            .split('\n')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter(|s| !s.incomplete())
            .map(|s| s.corrupt_score())
            .sum::<usize>()
    );
    let scores = include_str!("../../inputs/day10")
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter(|s| s.incomplete())
        .map(|s| s.incomplete_score())
        .sorted()
        .collect::<Vec<_>>();
    let middle_score = scores[scores.len() / 2];
    println!("Part 2 answer: {}", middle_score);
}

trait Parsing {
    fn incomplete(&self) -> bool;
    fn corrupt_score(&self) -> usize;
    fn corrupt_char(&self) -> Option<char>;
    fn incomplete_sequence(&self) -> Option<String>;
    fn incomplete_score(&self) -> usize;
}

impl<T> Parsing for T
where
    T: AsRef<str>,
{
    fn incomplete(&self) -> bool {
        let mut stack = vec![];
        for c in self.as_ref().chars() {
            if OPENING_CHARS.contains(&c) {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(opening) => {
                        let expected_char = MATCHING_CHARS
                            .iter()
                            .find(|(l, _r)| opening == *l)
                            .map(|(_l, closing)| *closing)
                            .unwrap_or('\0');
                        if c != expected_char {
                            // corrupted - incorrect closing char
                            return false;
                        }
                    }
                    // corrupted - closing char when expecting an opening
                    None => return false,
                }
            }
        }

        !stack.is_empty()
    }

    fn corrupt_char(&self) -> Option<char> {
        let mut stack = vec![];
        for c in self.as_ref().chars() {
            if OPENING_CHARS.contains(&c) {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(opening) => {
                        let expected_char = MATCHING_CHARS
                            .iter()
                            .find(|(l, _r)| opening == *l)
                            .map(|(_l, closing)| *closing)
                            .unwrap_or('\0');
                        if c != expected_char {
                            // corrupted - incorrect closing char
                            return Some(c);
                        }
                    }
                    // corrupted - closing char when expecting an opening
                    None => return Some(c),
                }
            }
        }
        None
    }

    fn corrupt_score(&self) -> usize {
        if let Some(c) = self.corrupt_char() {
            if let Some((_c, score)) = CORRUPT_CHAR_SCORES.iter().find(|(chr, _score)| c == *chr) {
                return *score;
            }
        }
        0
    }

    fn incomplete_sequence(&self) -> Option<String> {
        let mut stack = vec![];
        for c in self.as_ref().chars() {
            if OPENING_CHARS.contains(&c) {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(opening) => {
                        let expected_char = MATCHING_CHARS
                            .iter()
                            .find(|(l, _r)| opening == *l)
                            .map(|(_l, closing)| *closing)
                            .unwrap_or('\0');
                        if c != expected_char {
                            // corrupted - incorrect closing char
                            return None;
                        }
                    }
                    // corrupted - closing char when expecting an opening
                    None => return None,
                }
            }
        }

        Some(
            stack
                .iter()
                .map(|opening| {
                    MATCHING_CHARS
                        .iter()
                        .find(|(o, _c)| opening == o)
                        .map(|(_o, c)| *c)
                        .unwrap()
                })
                .rev()
                .collect(),
        )
    }

    fn incomplete_score(&self) -> usize {
        match self.incomplete_sequence() {
            Some(seq) => seq.chars().fold(0usize, |mut acc, c| {
                acc *= 5;
                if let Some(score) = INCOMPLETE_CHAR_SCORES
                    .iter()
                    .find(|(l, _score)| *l == c)
                    .map(|(_l, score)| *score)
                {
                    acc += score;
                }
                acc
            }),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_corrupt_score() {
        let expected = 26397;
        let actual = TEST_INPUT
            .iter()
            .filter(|s| !s.incomplete())
            .map(|s| s.corrupt_score())
            .sum::<usize>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_incomplete_score() {
        let expected = 288957;
        let scores = TEST_INPUT
            .iter()
            .filter(|s| s.incomplete())
            .map(|s| s.incomplete_score())
            .sorted()
            .collect::<Vec<_>>();
        let actual = scores[scores.len() / 2];
        assert_eq!(expected, actual);
    }
}
