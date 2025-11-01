use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    B,
    C,
    F,
    H,
    K,
    N,
    O,
    P,
    S,
    V,
}

impl TryFrom<u8> for Element {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'B' => Self::B,
            b'C' => Self::C,
            b'F' => Self::F,
            b'H' => Self::H,
            b'K' => Self::K,
            b'N' => Self::N,
            b'O' => Self::O,
            b'P' => Self::P,
            b'S' => Self::S,
            b'V' => Self::V,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule {
    pair: (Element, Element),
    to_insert: Element,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a, b, b' ', b'-', b'>', b' ', insert] = *s.as_bytes() else {
            return Err(ParseError::SyntaxError);
        };
        Ok(Self {
            pair: (a.try_into()?, b.try_into()?),
            to_insert: insert.try_into()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instructions {
    initial: Vec<Element>,
    rules: Vec<Rule>,
}

impl FromStr for Instructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let initial = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .bytes()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        if lines.next() != Some("") {
            return Err(ParseError::SyntaxError);
        }
        let rules = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { initial, rules })
    }
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<Instructions, ParseError> {
    input.parse()
}

#[aoc(day14, part1)]
fn part_1(instructions: &Instructions) -> u64 {
    simulate(instructions, 10)
}

#[aoc(day14, part2)]
fn part_2(instructions: &Instructions) -> u64 {
    simulate(instructions, 40)
}

fn simulate(instructions: &Instructions, rounds: usize) -> u64 {
    const fn index(e1: Element, e2: Element) -> usize {
        e1 as usize * 10 + e2 as usize
    }
    let mut rules = vec![vec![]; 100];
    for rule in &instructions.rules {
        let ix_pair = index(rule.pair.0, rule.pair.1);
        let ix_left = index(rule.pair.0, rule.to_insert);
        let ix_right = index(rule.to_insert, rule.pair.1);
        rules[ix_pair].push(ix_left);
        rules[ix_pair].push(ix_right);
    }
    let mut counts = [0_u64; 100];
    for (&a, &b) in instructions.initial.iter().zip(&instructions.initial[1..]) {
        counts[index(a, b)] += 1;
    }
    let mut leading = index(instructions.initial[0], instructions.initial[1]);

    let mut next = [0_u64; 100];
    for _ in 0..rounds {
        next.fill(0);
        leading = rules[leading][0];
        for (ix, &count) in counts.iter().enumerate() {
            for &ix2 in &rules[ix] {
                next[ix2] += count;
            }
        }
        counts = next;
    }
    let mut element_counts = [0; 10];
    element_counts[leading / 10] += 1;
    for (pair_ix, count) in counts.into_iter().enumerate() {
        element_counts[pair_ix % 10] += count;
    }
    let (min, max) = element_counts
        .iter()
        .copied()
        .filter(|&x| x > 0)
        .fold((u64::MAX, 0), |(min, max), x| (min.min(x), max.max(x)));
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        NNCB\n\
        \n\
        CH -> B\n\
        HH -> N\n\
        CB -> H\n\
        NH -> C\n\
        HB -> C\n\
        HC -> B\n\
        HN -> C\n\
        NN -> C\n\
        BH -> H\n\
        NC -> B\n\
        NB -> B\n\
        BN -> B\n\
        BB -> N\n\
        BC -> B\n\
        CC -> N\n\
        CN -> C\
    ";

    #[test]
    fn test_parse() {
        use Element::*;
        fn rule(a: Element, b: Element, to_insert: Element) -> Rule {
            Rule {
                pair: (a, b),
                to_insert,
            }
        }
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.initial, [N, N, C, B]);
        assert_eq!(
            result.rules,
            [
                rule(C, H, B),
                rule(H, H, N),
                rule(C, B, H),
                rule(N, H, C),
                rule(H, B, C),
                rule(H, C, B),
                rule(H, N, C),
                rule(N, N, C),
                rule(B, H, H),
                rule(N, C, B),
                rule(N, B, B),
                rule(B, N, B),
                rule(B, B, N),
                rule(B, C, B),
                rule(C, C, N),
                rule(C, N, C),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let instructions = parse(EXAMPLE).unwrap();
        let result = part_1(&instructions);
        assert_eq!(result, 1_588);
    }

    #[test]
    fn test_part_2() {
        let instructions = parse(EXAMPLE).unwrap();
        let result = part_2(&instructions);
        assert_eq!(result, 2_188_189_693_529);
    }
}
