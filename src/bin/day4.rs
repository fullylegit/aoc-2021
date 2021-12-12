const BOARD_SIZE: usize = 5;

fn main() {
    let input = include_str!("../../inputs/day4");
    let input = input
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let draw_numbers = input[0].split(',').collect::<Vec<_>>();
    let input = &input[1..];
    let boards = {
        let mut boards = Vec::<Board>::new();
        for offset in (0..input.len()).step_by(BOARD_SIZE) {
            match input.get(offset..offset + BOARD_SIZE) {
                Some(input) => boards.push(Board::parse(input)),
                None => break,
            }
        }
        boards
    };

    println!("Part 1 answer: {}", play_bingo(&boards, &draw_numbers));
    println!(
        "Part 2 answer: {}",
        play_bingo_to_lose(&boards, &draw_numbers)
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Board<'a> {
    squares: [[Square<'a>; BOARD_SIZE]; BOARD_SIZE],
}

impl<'a> Board<'a> {
    fn parse(input: &[&'a str]) -> Self {
        assert!(input.len() == BOARD_SIZE);
        let mut this = Board {
            squares: Default::default(),
        };
        for (row, input) in input.iter().enumerate() {
            for (col, input) in input
                .split(' ')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .enumerate()
            {
                this.squares[row][col].number = input;
            }
        }
        this
    }

    fn draw(&mut self, draw_number: &str) {
        for row in self.squares.iter_mut() {
            for col in row {
                if col.number == draw_number {
                    col.marked = true;
                }
            }
        }
    }

    fn finished(&self) -> bool {
        let mut column_results = [true; BOARD_SIZE];
        for row in self.squares {
            if row.iter().all(|s| s.marked) {
                return true;
            }
            for (col_num, col) in row.iter().enumerate() {
                if !col.marked {
                    column_results[col_num] = false;
                }
            }
        }
        column_results.iter().any(|c| *c)
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for row in self.squares {
            for col in row {
                if !col.marked {
                    score += col.number.parse::<usize>().unwrap_or_default();
                }
            }
        }
        score
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Square<'a> {
    number: &'a str,
    marked: bool,
}

#[cfg(test)]
impl<'a> Square<'a> {
    const fn with_number(number: &'a str) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

fn play_bingo(boards: &[Board], draw_numbers: &[&str]) -> usize {
    let mut boards = boards.to_vec();
    for draw_number in draw_numbers {
        boards.iter_mut().for_each(|board| board.draw(draw_number));
        if let Some(board) = boards.iter().find(|b| b.finished()) {
            return board.score() * draw_number.parse::<usize>().unwrap_or_default();
        }
    }
    0
}

fn play_bingo_to_lose(boards: &[Board], draw_numbers: &[&str]) -> usize {
    let mut boards = boards.to_vec();
    for draw_number in draw_numbers {
        boards.iter_mut().for_each(|board| board.draw(draw_number));
        if boards.len() == 1 && boards[0].finished() {
            return boards[0].score() * draw_number.parse::<usize>().unwrap_or_default();
        }
        boards = boards.into_iter().filter(|b| !b.finished()).collect();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BOARDS: [Board; 3] = [
        Board {
            squares: [
                [
                    Square::with_number("22"),
                    Square::with_number("13"),
                    Square::with_number("17"),
                    Square::with_number("11"),
                    Square::with_number("0"),
                ],
                [
                    Square::with_number("8"),
                    Square::with_number("2"),
                    Square::with_number("23"),
                    Square::with_number("4"),
                    Square::with_number("24"),
                ],
                [
                    Square::with_number("21"),
                    Square::with_number("9"),
                    Square::with_number("14"),
                    Square::with_number("16"),
                    Square::with_number("7"),
                ],
                [
                    Square::with_number("6"),
                    Square::with_number("10"),
                    Square::with_number("3"),
                    Square::with_number("18"),
                    Square::with_number("5"),
                ],
                [
                    Square::with_number("1"),
                    Square::with_number("12"),
                    Square::with_number("20"),
                    Square::with_number("15"),
                    Square::with_number("19"),
                ],
            ],
        },
        Board {
            squares: [
                [
                    Square::with_number("3"),
                    Square::with_number("15"),
                    Square::with_number("0"),
                    Square::with_number("2"),
                    Square::with_number("22"),
                ],
                [
                    Square::with_number("9"),
                    Square::with_number("18"),
                    Square::with_number("13"),
                    Square::with_number("17"),
                    Square::with_number("5"),
                ],
                [
                    Square::with_number("19"),
                    Square::with_number("8"),
                    Square::with_number("7"),
                    Square::with_number("25"),
                    Square::with_number("23"),
                ],
                [
                    Square::with_number("20"),
                    Square::with_number("11"),
                    Square::with_number("10"),
                    Square::with_number("24"),
                    Square::with_number("4"),
                ],
                [
                    Square::with_number("14"),
                    Square::with_number("21"),
                    Square::with_number("16"),
                    Square::with_number("12"),
                    Square::with_number("6"),
                ],
            ],
        },
        Board {
            squares: [
                [
                    Square::with_number("14"),
                    Square::with_number("21"),
                    Square::with_number("17"),
                    Square::with_number("24"),
                    Square::with_number("4"),
                ],
                [
                    Square::with_number("10"),
                    Square::with_number("16"),
                    Square::with_number("15"),
                    Square::with_number("9"),
                    Square::with_number("19"),
                ],
                [
                    Square::with_number("18"),
                    Square::with_number("8"),
                    Square::with_number("23"),
                    Square::with_number("26"),
                    Square::with_number("20"),
                ],
                [
                    Square::with_number("22"),
                    Square::with_number("11"),
                    Square::with_number("13"),
                    Square::with_number("6"),
                    Square::with_number("5"),
                ],
                [
                    Square::with_number("2"),
                    Square::with_number("0"),
                    Square::with_number("12"),
                    Square::with_number("3"),
                    Square::with_number("7"),
                ],
            ],
        },
    ];

    #[test]
    fn test_parse_boards() {
        let input = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
        "#;

        let expected = TEST_BOARDS;
        let input = input
            .split('\n')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let actual = {
            let mut boards = Vec::<Board>::new();
            for offset in (0..input.len()).step_by(BOARD_SIZE) {
                match input.get(offset..offset + BOARD_SIZE) {
                    Some(input) => boards.push(Board::parse(input)),
                    None => break,
                }
            }
            boards
        };
        assert_eq!(&expected[..], &actual);
    }

    #[test]
    fn test_play_bingo() {
        let draw_numbers = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
            .split(',')
            .map(str::trim)
            .collect::<Vec<_>>();

        let expected = 4512;
        let actual = play_bingo(&TEST_BOARDS, &draw_numbers);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_play_bingo_to_lose() {
        let draw_numbers = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
            .split(',')
            .map(str::trim)
            .collect::<Vec<_>>();

        let expected = 1924;
        let actual = play_bingo_to_lose(&TEST_BOARDS, &draw_numbers);
        assert_eq!(expected, actual);
    }
}
