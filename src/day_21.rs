use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
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
struct Input {
    player1: u8,
    player2: u8,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let player1 = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_prefix("Player 1 starting position: ")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        let player2 = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_prefix("Player 2 starting position: ")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        if lines.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self { player1, player2 })
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Input, ParseError> {
    input.parse()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct DeterministicDie {
    face: u8,
    rolls: u64,
}

impl DeterministicDie {
    const fn roll(&mut self) -> u8 {
        self.face = self.face % 100 + 1;
        self.rolls += 1;
        self.face
    }
}

#[aoc(day21, part1)]
fn part_1(input: &Input) -> u64 {
    let mut score1 = 0;
    let mut score2 = 0;
    let mut pos1 = input.player1 - 1;
    let mut pos2 = input.player2 - 1;
    let mut die = DeterministicDie::default();
    while score2 < 1000 {
        let steps = die.roll() % 10 + die.roll() % 10 + die.roll() % 10;
        (pos1, pos2) = (pos2, (pos1 + steps) % 10);
        (score1, score2) = (score2, score1 + u64::from(pos2 + 1));
    }
    die.rolls * score1
}

#[derive(Debug, Clone)]
struct Grid<const D: usize, T> {
    data: Vec<T>,
    size: [usize; D],
    stride: [usize; D],
}

impl<const D: usize, T> Grid<D, T> {
    fn new(size: [usize; D]) -> Self
    where
        T: Default,
    {
        let mut stride = [0; D];
        let mut prod = 1;
        for (st, &sz) in stride.iter_mut().zip(&size) {
            *st = prod;
            prod *= sz;
        }
        Self {
            data: (0..prod).map(|_| T::default()).collect(),
            size,
            stride,
        }
    }
}

impl<const D: usize, T> Index<[usize; D]> for Grid<D, T> {
    type Output = T;

    fn index(&self, indexes: [usize; D]) -> &Self::Output {
        let index = self
            .stride
            .iter()
            .zip(&self.size)
            .zip(indexes)
            .map(|((&st, &sz), x)| (x < sz).then_some(st * x))
            .sum::<Option<usize>>()
            .expect("Index out of range");
        &self.data[index]
    }
}

impl<const D: usize, T> IndexMut<[usize; D]> for Grid<D, T> {
    fn index_mut(&mut self, indexes: [usize; D]) -> &mut Self::Output {
        let index = self
            .stride
            .iter()
            .zip(&self.size)
            .zip(indexes)
            .map(|((&st, &sz), x)| (x < sz).then_some(st * x))
            .sum::<Option<usize>>()
            .expect("Index out of range");
        &mut self.data[index]
    }
}

#[aoc(day21, part2)]
fn part_2(input: &Input) -> u64 {
    let mut state = Grid::new([10, 10, 21, 21]);
    let mut next = Grid::new([10, 10, 21, 21]);
    state[[input.player1 as usize - 1, input.player2 as usize - 1, 0, 0]] = 1;
    let mut wins1 = 0;
    let mut wins2 = 0;
    let steps = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut any_change = true;
    while any_change {
        any_change = false;
        next.data.fill(0);
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                for score1 in 0..21 {
                    for score2 in 0..21 {
                        let cnt = state[[pos1 - 1, pos2 - 1, score1, score2]];
                        if cnt == 0 {
                            continue;
                        }
                        any_change = true;
                        for (step, mult) in steps {
                            let next_pos1 = (pos1 - 1 + step) % 10 + 1;
                            let next_score1 = score1 + next_pos1;
                            if next_score1 >= 21 {
                                wins1 += cnt * mult;
                            } else {
                                next[[pos2 - 1, next_pos1 - 1, score2, next_score1]] += cnt * mult;
                            }
                        }
                    }
                }
            }
        }
        (state, next) = (next, state);
        (wins1, wins2) = (wins2, wins1);
    }
    wins1.max(wins2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        Player 1 starting position: 4\n\
        Player 2 starting position: 8\
    ";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 739_785);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 444_356_092_776_315);
    }
}
