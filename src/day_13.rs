use std::collections::HashSet;
use std::fmt::Debug;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
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
enum Instruction {
    FoldAlongX(i16),
    FoldAlongY(i16),
}

impl Instruction {
    const fn apply(self, point: Point) -> Point {
        match self {
            Self::FoldAlongX(x) if point.x > x => Point {
                x: 2 * x - point.x,
                ..point
            },
            Self::FoldAlongY(y) if point.y > y => Point {
                y: 2 * y - point.y,
                ..point
            },
            _ => point,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once('=').ok_or(ParseError::SyntaxError)?;
        Ok(match key {
            "fold along x" => Self::FoldAlongX(value.parse()?),
            "fold along y" => Self::FoldAlongY(value.parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManualPage {
    points: Vec<Point>,
    instructions: Vec<Instruction>,
}

impl FromStr for ManualPage {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let points = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let instructions = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self {
            points,
            instructions,
        })
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<ManualPage, ParseError> {
    input.parse()
}

#[aoc(day13, part1)]
fn part_1(manual_page: &ManualPage) -> usize {
    let first = manual_page.instructions[0];
    let mut points = HashSet::new();
    for &point in &manual_page.points {
        points.insert(first.apply(point));
    }
    points.len()
}

#[aoc(day13, part2)]
fn part_2(manual_page: &ManualPage) -> String {
    let mut points = HashSet::new();
    let (mut min_x, mut max_x) = (i16::MAX, i16::MIN);
    let (mut min_y, mut max_y) = (i16::MAX, i16::MIN);
    for &point in &manual_page.points {
        let folded = manual_page
            .instructions
            .iter()
            .fold(point, |pt, instr| instr.apply(pt));
        min_x = min_x.min(folded.x);
        max_x = max_x.max(folded.x);
        min_y = min_y.min(folded.y);
        max_y = max_y.max(folded.y);
        points.insert(folded);
    }
    let mut result = String::new();
    for y in (min_y..=max_y).step_by(2) {
        result.push('\n');
        for x in min_x..=max_x {
            result.push(
                match (
                    points.contains(&Point { x, y }),
                    points.contains(&Point { x, y: y + 1 }),
                ) {
                    (true, true) => '█',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (false, false) => ' ',
                },
            );
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        6,10\n\
        0,14\n\
        9,10\n\
        0,3\n\
        10,4\n\
        4,11\n\
        6,0\n\
        6,12\n\
        4,1\n\
        0,13\n\
        10,12\n\
        3,4\n\
        3,0\n\
        8,4\n\
        1,10\n\
        2,14\n\
        8,10\n\
        9,0\n\
        \n\
        fold along y=7\n\
        fold along x=5\
    ";

    #[test]
    fn test_parse() {
        macro_rules! pt {
            ($x:expr,$y:expr) => {
                Point { x: $x, y: $y }
            };
        }
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result.points,
            [
                pt!(6, 10),
                pt!(0, 14),
                pt!(9, 10),
                pt!(0, 3),
                pt!(10, 4),
                pt!(4, 11),
                pt!(6, 0),
                pt!(6, 12),
                pt!(4, 1),
                pt!(0, 13),
                pt!(10, 12),
                pt!(3, 4),
                pt!(3, 0),
                pt!(8, 4),
                pt!(1, 10),
                pt!(2, 14),
                pt!(8, 10),
                pt!(9, 0)
            ]
        );
        assert_eq!(
            result.instructions,
            [Instruction::FoldAlongY(7), Instruction::FoldAlongX(5)]
        );
    }

    #[test]
    fn test_part_1() {
        let manual_page = parse(EXAMPLE).unwrap();
        let result = part_1(&manual_page);
        assert_eq!(result, 17);
    }

    #[test]
    fn test_part_2() {
        let manual_page = parse(EXAMPLE).unwrap();
        let result = part_2(&manual_page);
        assert_eq!(
            result,
            "\n\
            █▀▀▀█\n\
            █   █\n\
            ▀▀▀▀▀\
            "
        );
    }
}
