use std::str::FromStr;

use smallvec::SmallVec;
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Wires(u8);

impl FromStr for Wires {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0;
        for ch in s.bytes() {
            bits |= 1 << (ch - b'a');
        }
        Ok(Self(bits))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SegmentDisplay {
    digits: [Wires; 10],
    output: [Wires; 4],
}

impl SegmentDisplay {
    fn find_mapping(self) -> [u8; 10] {
        let one = self
            .digits
            .iter()
            .find(|w| w.0.count_ones() == 2)
            .copied()
            .unwrap();
        let four = self
            .digits
            .iter()
            .find(|w| w.0.count_ones() == 4)
            .copied()
            .unwrap();
        self.digits.map(|d| {
            match (
                (d.0 ^ one.0).count_ones(),
                (d.0 ^ four.0).count_ones(),
                (d.0 ^ one.0 ^ four.0).count_ones(),
            ) {
                (0, _, _) => 1,
                (1, _, _) => 7,
                (2, _, _) => 4,
                (3, _, _) => 3,
                (6, _, _) => 6,
                (_, 2, _) => 9,
                (_, 4, _) => 0,
                (_, 5, _) => 2,
                (_, _, 3) => 5,
                (_, _, 5) => 8,
                _ => unreachable!(),
            }
        })
    }

    fn decode_output(&self) -> u32 {
        let mapping = self.find_mapping();
        self.output
            .iter()
            .map(|o| {
                self.digits
                    .iter()
                    .zip(&mapping)
                    .find_map(|(d, &v)| (d == o).then_some(v))
                    .unwrap()
            })
            .fold(0, |s, d| s * 10 + u32::from(d))
    }
}

impl FromStr for SegmentDisplay {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (digits, output) = s.split_once(" | ").ok_or(ParseError::SyntaxError)?;
        let digits = digits
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<SmallVec<[Wires; 10]>, _>>()?
            .into_inner()
            .map_err(|_| ParseError::SyntaxError)?;
        let output = output
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<SmallVec<[Wires; 4]>, _>>()?
            .into_inner()
            .map_err(|_| ParseError::SyntaxError)?;
        Ok(Self { digits, output })
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Vec<SegmentDisplay>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day8, part1)]
fn part_1(displays: &[SegmentDisplay]) -> usize {
    displays
        .iter()
        .flat_map(|d| &d.output)
        .filter(|d| matches!(d.0.count_ones(), 2..=4 | 7))
        .count()
}

#[aoc(day8, part2)]
fn part_2(displays: &[SegmentDisplay]) -> u32 {
    displays.iter().map(SegmentDisplay::decode_output).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "\
        acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\
    ";

    const EXAMPLE2: &str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(
            result,
            [SegmentDisplay {
                digits: [
                    Wires(0b111_1111),
                    Wires(0b011_1110),
                    Wires(0b110_1101),
                    Wires(0b010_1111),
                    Wires(0b000_1011),
                    Wires(0b011_1111),
                    Wires(0b111_1110),
                    Wires(0b011_0011),
                    Wires(0b101_1111),
                    Wires(0b000_0011)
                ],
                output: [
                    Wires(0b011_1110),
                    Wires(0b010_1111),
                    Wires(0b011_1110),
                    Wires(0b010_1111)
                ]
            }]
        );
    }

    #[test_case(EXAMPLE1 => 0)]
    #[test_case(EXAMPLE2 => 26)]
    fn test_part_1(input: &str) -> usize {
        let displays = parse(input).unwrap();
        part_1(&displays)
    }

    #[test_case(EXAMPLE1 => 5_353)]
    #[test_case(EXAMPLE2 => 61_229)]
    fn test_part_2(input: &str) -> u32 {
        let displays = parse(input).unwrap();
        part_2(&displays)
    }
}
