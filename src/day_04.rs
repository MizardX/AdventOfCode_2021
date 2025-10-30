use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bingo {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl FromStr for Bingo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let numbers = parts
            .next()
            .ok_or(ParseError::SyntaxError)?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let boards = parts.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { numbers, boards })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Board {
    grid: [u8; 25],
    marks: u32,
}

impl Board {
    fn mark(&mut self, num: u8) {
        if let Some(ix) = self.grid.iter().position(|&x| x == num) {
            self.marks |= 1 << ix;
        }
    }

    const fn has_bingo(&self) -> bool {
        const COL: u32 = 0b00001_00001_00001_00001_00001;
        const ROW: u32 = 0b11111;
        let m = self.marks;
        ((m >> 4) & (m >> 3) & (m >> 2) & (m >> 1) & m & COL) != 0
            || ((m >> 20) & (m >> 15) & (m >> 10) & (m >> 5) & m & ROW) != 0
    }

    fn sum_unmarked(&self) -> u32 {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(ix, &val)| ((self.marks & (1 << ix)) == 0).then_some(u32::from(val)))
            .sum()
    }
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [0; 25];
        for (y, line) in s.lines().enumerate() {
            for (x, cell) in line.split_ascii_whitespace().enumerate() {
                grid[y * 5 + x] = cell.parse()?;
            }
        }
        Ok(Self { grid, marks: 0 })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                let ix = 5 * y + x;
                let val = self.grid[ix];
                if (self.marks & (1 << ix)) != 0 {
                    write!(f, "\x1b[97m{val:2}\x1b[0m ")?;
                } else {
                    write!(f, "\x1b[90m{val:2}\x1b[0m ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Bingo, ParseError> {
    input.parse()
}

#[aoc(day4, part1)]
fn part_1(bingo: &Bingo) -> u32 {
    let mut boards = bingo.boards.clone();
    for &num in &bingo.numbers {
        for board in &mut boards {
            board.mark(num);
            if board.has_bingo() {
                return board.sum_unmarked() * u32::from(num);
            }
        }
    }
    0
}

#[aoc(day4, part2)]
fn part_2(bingo: &Bingo) -> u32 {
    let mut boards = bingo.boards.clone();
    for &num in &bingo.numbers {
        let final_board = boards.len() == 1;
        for board in &mut boards {
            board.mark(num);
            if final_board && board.has_bingo() {
                return boards[0].sum_unmarked() * u32::from(num);
            }
        }
        boards.retain(|b| !b.has_bingo());
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
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
 2  0 12  3  7\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(&result.numbers[..5], [7, 4, 9, 5, 11]);
        assert_eq!(&result.boards[1].grid[0..5], [3, 15, 0, 2, 22]);
    }

    #[test]
    fn test_part_1() {
        let bingo = parse(EXAMPLE).unwrap();
        let result = part_1(&bingo);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part_2() {
        let bingo = parse(EXAMPLE).unwrap();
        let result = part_2(&bingo);
        assert_eq!(result, 1924);
    }
}
