use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use smallvec::SmallVec;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cave {
    Start,
    End,
    Large(u8),
    Small(u8),
}

impl Cave {
    #[must_use]
    fn into_index(self) -> usize {
        match self {
            Self::Start => 0,
            Self::End => 1,
            Self::Large(ix) | Self::Small(ix) => usize::from(ix),
        }
    }

    #[must_use]
    const fn is_large(self) -> bool {
        matches!(self, Self::Large(..))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CaveSystem {
    caves: SmallVec<[Cave; 16]>,
    neighbors: SmallVec<[u16; 16]>,
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

impl FromStr for CaveSystem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lookup = HashMap::new();
        let mut caves = SmallVec::<[Cave; 16]>::new();
        let mut neighbors = smallvec::smallvec![0_u16; 16];
        caves.push(Cave::Start);
        caves.push(Cave::End);
        lookup.insert("start", Cave::Start);
        lookup.insert("end", Cave::End);
        for line in s.lines() {
            let (first, second) = line.split_once('-').ok_or(ParseError::SyntaxError)?;
            let first = *lookup.entry(first).or_insert_with_key(|name| {
                let ix = u8::try_from(caves.len()).unwrap();
                let cave = if name.bytes().all(|b| b.is_ascii_uppercase()) {
                    Cave::Large(ix)
                } else {
                    Cave::Small(ix)
                };
                caves.push(cave);
                cave
            });
            let second = *lookup.entry(second).or_insert_with_key(|name| {
                let ix = u8::try_from(caves.len()).unwrap();
                let cave = if name.bytes().all(|b| b.is_ascii_uppercase()) {
                    Cave::Large(ix)
                } else {
                    Cave::Small(ix)
                };
                caves.push(cave);
                cave
            });
            neighbors[first.into_index()] |= 1 << second.into_index();
            if first != Cave::Start {
                neighbors[second.into_index()] |= 1 << first.into_index();
            }
        }
        neighbors.truncate(caves.len());
        Ok(Self { caves, neighbors })
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<CaveSystem, ParseError> {
    input.parse()
}

#[aoc(day12, part1)]
fn part_1(caves: &CaveSystem) -> usize {
    count_paths(caves, false)
}

#[aoc(day12, part2)]
fn part_2(caves: &CaveSystem) -> usize {
    count_paths(caves, true)
}

fn count_paths(caves: &CaveSystem, visit_twice: bool) -> usize {
    let mut pending = VecDeque::new();
    pending.push_back((Cave::Start, 0_u16, visit_twice));
    let mut count = 0;
    while let Some((cave, visited, visit_twice)) = pending.pop_back() {
        if cave == Cave::End {
            count += 1;
            continue;
        }
        for &next in &caves.caves[1..] {
            let bit = 1 << next.into_index();
            if caves.neighbors[cave.into_index()] & bit != 0 {
                if next.is_large() || visited & bit == 0 {
                    pending.push_back((next, visited | bit, visit_twice));
                } else if visit_twice {
                    pending.push_back((next, visited | bit, false));
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "\
        start-A\n\
        start-b\n\
        A-c\n\
        A-b\n\
        b-d\n\
        A-end\n\
        b-end\
    ";

    const EXAMPLE2: &str = "\
        dc-end\n\
        HN-start\n\
        start-kj\n\
        dc-start\n\
        dc-HN\n\
        LN-dc\n\
        HN-end\n\
        kj-sa\n\
        kj-HN\n\
        kj-dc\
    ";

    const EXAMPLE3: &str = "\
        fs-end\n\
        he-DX\n\
        fs-he\n\
        start-DX\n\
        pj-DX\n\
        end-zg\n\
        zg-sl\n\
        zg-pj\n\
        pj-he\n\
        RW-he\n\
        fs-DX\n\
        pj-RW\n\
        zg-RW\n\
        start-pj\n\
        he-WI\n\
        zg-he\n\
        pj-fs\n\
        start-RW\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(
            result.caves.as_slice(),
            [
                Cave::Start,
                Cave::End,
                Cave::Large(2),
                Cave::Small(3),
                Cave::Small(4),
                Cave::Small(5)
            ]
        );
        assert_eq!(
            result.neighbors.as_slice(),
            [
                //dc_bA$^
                0b00_1100, // start -> A(2), b(3)
                0b00_1100, // end -> A(2), b(3)
                0b01_1010, // A(2) -> end, b(3), c(4)
                0b10_0110, // b(3) -> end, A(2), d(5)
                0b00_0100, // c(4) -> A(2)
                0b00_1000, // d(5) -> b(3)
            ]
        );
    }

    #[test_case(EXAMPLE1, false => 10)]
    #[test_case(EXAMPLE2, false => 19)]
    #[test_case(EXAMPLE3, false => 226)]
    #[test_case(EXAMPLE1, true => 36)]
    fn test_count_paths(input: &str, visit_twice: bool) -> usize {
        let caves = parse(input).unwrap();
        count_paths(&caves, visit_twice)
    }
}
