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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, dist) = s.split_once(' ').ok_or(ParseError::SyntaxError)?;
        Ok(match command {
            "forward" => Self::Forward(dist.parse()?),
            "up" => Self::Up(dist.parse()?),
            "down" => Self::Down(dist.parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Command>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day2, part1)]
fn part_1(commands: &[Command]) -> u64 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    for &command in commands {
        match command {
            Command::Forward(dist) => horizontal += dist,
            Command::Up(dist) => depth = depth.saturating_sub(dist),
            Command::Down(dist) => depth += dist,
        }
    }
    u64::from(horizontal) * u64::from(depth)
}

#[aoc(day2, part2)]
fn part_2(commands: &[Command]) -> u64 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    for &command in commands {
        match command {
            Command::Forward(dist) => {
                horizontal += dist;
                depth += aim * dist;
            }
            Command::Up(dist) => aim = aim.saturating_sub(dist),
            Command::Down(dist) => aim += dist,
        }
    }
    u64::from(horizontal) * u64::from(depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let commands = parse(EXAMPLE).unwrap();
        let result = part_1(&commands);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_part_2() {
        let commands = parse(EXAMPLE).unwrap();
        let result = part_2(&commands);
        assert_eq!(result, 900);
    }
}
