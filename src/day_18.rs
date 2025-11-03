use std::fmt::Display;
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailNumber {
    Single(u8),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl SnailNumber {
    fn magnitude(&self) -> u64 {
        match self {
            Self::Single(num) => u64::from(*num),
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    const fn as_single(&self) -> Option<u8> {
        if let Self::Single(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    fn add_left(&mut self, value: u8) {
        match self {
            Self::Single(x) => *x += value,
            Self::Pair(left, _) => left.add_left(value),
        }
    }

    fn add_right(&mut self, value: u8) {
        match self {
            Self::Single(x) => *x += value,
            Self::Pair(_, right) => right.add_right(value),
        }
    }

    fn explode(&mut self, depth: usize) -> (Option<u8>, Option<u8>) {
        match self {
            Self::Single(_) => (None, None),
            Self::Pair(left, right) if depth >= 4 => {
                let res = (left.as_single(), right.as_single());
                *self = Self::Single(0);
                res
            }
            Self::Pair(left, right) => {
                let (ll, lr) = left.explode(depth + 1);
                if let Some(lr) = lr {
                    right.add_left(lr);
                }
                let (rl, rr) = right.explode(depth + 1);
                if let Some(rl) = rl {
                    left.add_right(rl);
                }
                (ll, rr)
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Single(x) if *x >= 10 => {
                *self = Self::Pair(
                    Box::new(Self::Single(*x / 2)),
                    Box::new(Self::Single(x.div_ceil(2))),
                );
                true
            }
            Self::Single(_) => false,
            Self::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn add(a: Self, b: Self) -> Self {
        let mut res = Self::Pair(Box::new(a), Box::new(b));
        res.explode(0);
        while res.split() {
            res.explode(0);
        }
        res
    }
}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(x) => write!(f, "{x}"),
            Self::Pair(left, right) => write!(f, "[{left},{right}]"),
        }
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Vec<SnailNumber>, ParseError> {
    fn inner(input: &[u8], separator: &[usize]) -> Result<SnailNumber, ParseError> {
        let len = input.len();
        match input {
            [ch @ b'0'..=b'9'] => Ok(SnailNumber::Single(ch - b'0')),
            [b'[', .., b']'] => {
                let comma = separator[0];
                Ok(SnailNumber::Pair(
                    Box::new(inner(&input[1..comma], &separator[1..comma])?),
                    Box::new(inner(
                        &input[comma + 1..len - 1],
                        &separator[comma + 1..len - 1],
                    )?),
                ))
            }
            _ => Err(ParseError::SyntaxError),
        }
    }
    let mut result = Vec::new();
    for line in input.lines() {
        let input = line;
        let mut separator = vec![0; input.len()];
        let mut separator_stack: Vec<usize> = Vec::new();
        for (ix, ch) in input.bytes().enumerate().rev() {
            match ch {
                b']' => {
                    separator_stack.push(ix);
                }
                b',' => {
                    separator[ix] = separator_stack.pop().ok_or(ParseError::SyntaxError)? - ix;
                    separator_stack.push(ix);
                }
                b'[' => {
                    separator[ix] = separator_stack.pop().ok_or(ParseError::SyntaxError)? - ix;
                }
                _ => {}
            }
        }
        if !separator_stack.is_empty() {
            return Err(ParseError::SyntaxError);
        }
        result.push(inner(input.as_bytes(), &separator)?);
    }
    Ok(result)
}

#[aoc(day18, part1)]
fn part_1(numbers: &[SnailNumber]) -> u64 {
    let numbers = numbers.to_vec();
    numbers
        .into_iter()
        .reduce(SnailNumber::add)
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
fn part_2(numbers: &[SnailNumber]) -> u64 {
    let mut max_magnitude = 0;
    for (ix, num1) in numbers.iter().enumerate() {
        for num2 in &numbers[..ix] {
            let sum = SnailNumber::add(num1.clone(), num2.clone());
            let mag = sum.magnitude();
            max_magnitude = max_magnitude.max(mag);
            let sum = SnailNumber::add(num2.clone(), num1.clone());
            let mag = sum.magnitude();
            max_magnitude = max_magnitude.max(mag);
        }
    }
    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "\
        [1,2]\n\
        [[3,4],5]\
    ";

    const EXAMPLE2: &str = "\
        [[[[4,3],4],4],[7,[[8,4],9]]]\n\
        [1,1]\
    ";

    const EXAMPLE3: &str = "\
        [1,1]\n\
        [2,2]\n\
        [3,3]\n\
        [4,4]\
    ";

    const EXAMPLE4: &str = "\
        [1,1]\n\
        [2,2]\n\
        [3,3]\n\
        [4,4]\n\
        [5,5]\
    ";

    const EXAMPLE5: &str = "\
        [1,1]\n\
        [2,2]\n\
        [3,3]\n\
        [4,4]\n\
        [5,5]\n\
        [6,6]\
    ";

    const EXAMPLE6: &str = "\
        [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n\
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n\
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n\
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n\
        [7,[5,[[3,8],[1,4]]]]\n\
        [[2,[2,2]],[8,[8,1]]]\n\
        [2,9]\n\
        [1,[[[9,3],9],[[9,0],[0,7]]]]\n\
        [[[5,[7,4]],7],1]\n\
        [[[[4,2],2],6],[8,7]]\
    ";

    const EXAMPLE7: &str = "\
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
        [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
        [[[[5,4],[7,7]],8],[[8,3],8]]\n\
        [[9,3],[[9,9],[6,[4,9]]]]\n\
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\
    ";

    #[test_case(EXAMPLE1 => 143)]
    #[test_case(EXAMPLE2 => 1384)]
    #[test_case(EXAMPLE3 => 445)]
    #[test_case(EXAMPLE4 => 791)]
    #[test_case(EXAMPLE5 => 1137)]
    #[test_case(EXAMPLE6 => 3488)]
    #[test_case(EXAMPLE7 => 4140)]
    fn test_part_1(input: &str) -> u64 {
        let numbers = parse(input).unwrap();
        part_1(&numbers)
    }

    #[test_case(EXAMPLE7 => 3993)]
    fn test_part_2(input: &str) -> u64 {
        let numbers = parse(input).unwrap();
        part_2(&numbers)
    }
}
