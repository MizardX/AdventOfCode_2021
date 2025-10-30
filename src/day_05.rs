use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    const fn is_axis_aligned(self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

impl IntoIterator for Line {
    type Item = Point;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            pos: self.start,
            dx: (self.end.x.cast_signed() - self.start.x.cast_signed()).signum(),
            dy: (self.end.y.cast_signed() - self.start.y.cast_signed()).signum(),
            remaining: self
                .start
                .x
                .abs_diff(self.end.x)
                .max(self.start.y.abs_diff(self.end.y)),
            first: true,
        }
    }
}

struct LineIterator {
    pos: Point,
    dx: i16,
    dy: i16,
    remaining: u16,
    first: bool,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.pos);
        }
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        self.pos.x = self.pos.x.checked_add_signed(self.dx).unwrap();
        self.pos.y = self.pos.y.checked_add_signed(self.dy).unwrap();
        Some(self.pos)
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<Line>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day5, part1)]
fn part_1(lines: &[Line]) -> usize {
    let mut counts = HashMap::<Point, u16>::new();
    for line in lines {
        if line.is_axis_aligned() {
            for point in line.into_iter() {
                *counts.entry(point).or_default() += 1;
            }
        }
    }
    counts.values().filter(|&&c| c > 1).count()
}

#[aoc(day5, part2)]
fn part_2(lines: &[Line]) -> usize {
    let mut counts = HashMap::<Point, u16>::new();
    for line in lines {
        for point in line.into_iter() {
            *counts.entry(point).or_default() += 1;
        }
    }
    counts.values().filter(|&&c| c > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2\
    ";

    #[test]
    fn test_parse() {
        macro_rules! pt {
            ($x:expr,$y:expr) => {
                Point { x: $x, y: $y }
            };
        }
        macro_rules! line {
            ($start:expr, $end:expr) => {
                Line {
                    start: $start,
                    end: $end,
                }
            };
        }
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                line!(pt!(0, 9), pt!(5, 9)),
                line!(pt!(8, 0), pt!(0, 8)),
                line!(pt!(9, 4), pt!(3, 4)),
                line!(pt!(2, 2), pt!(2, 1)),
                line!(pt!(7, 0), pt!(7, 4)),
                line!(pt!(6, 4), pt!(2, 0)),
                line!(pt!(0, 9), pt!(2, 9)),
                line!(pt!(3, 4), pt!(1, 4)),
                line!(pt!(0, 0), pt!(8, 8)),
                line!(pt!(5, 5), pt!(8, 2)),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let lines = parse(EXAMPLE).unwrap();
        let result = part_1(&lines);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_2() {
        let lines = parse(EXAMPLE).unwrap();
        let result = part_2(&lines);
        assert_eq!(result, 12);
    }
}
