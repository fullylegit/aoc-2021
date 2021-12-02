fn main() {
    let input = include_str!("../../inputs/day1");
    let depths = input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();
    println!("Part 1 answer: {}", count_increases_part_1(&depths));
    println!("Part 2 answer: {}", count_increases_part_2(&depths));
}

fn count_increases_part_1(depths: &[usize]) -> usize {
    depths
        .windows(2)
        .filter(|window| match window {
            [left, right] => right > left,
            _ => false,
        })
        .count()
}

fn count_increases_part_2(depths: &[usize]) -> usize {
    let window_sums = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<usize>>();
    window_sums
        .windows(2)
        .filter(|window| match window {
            [left, right] => right > left,
            _ => false,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases_part_1() {
        let input = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        let actual = count_increases_part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_increases_part_2() {
        let input = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;
        let actual = count_increases_part_2(input);
        assert_eq!(expected, actual);
    }
}
