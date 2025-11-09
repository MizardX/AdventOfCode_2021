use std::collections::VecDeque;
use std::fmt::{Display, Write};
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

type Value = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl Reg {
    const fn all() -> [Self; 4] {
        [Self::W, Self::X, Self::Y, Self::Z]
    }
}

impl FromStr for Reg {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::W => f.write_char('w'),
            Self::X => f.write_char('x'),
            Self::Y => f.write_char('y'),
            Self::Z => f.write_char('z'),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegOrValue {
    Reg(Reg),
    Value(Value),
}

impl FromStr for RegOrValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() && s.as_bytes()[0].is_ascii_lowercase() {
            Ok(Self::Reg(s.parse()?))
        } else {
            Ok(Self::Value(s.parse()?))
        }
    }
}

impl Display for RegOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(reg) => write!(f, "{reg}"),
            Self::Value(val) => write!(f, "{val}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Input(Reg),
    Add(Reg, RegOrValue),
    Mul(Reg, RegOrValue),
    Div(Reg, RegOrValue),
    Mod(Reg, RegOrValue),
    Eql(Reg, RegOrValue),
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once(' ').ok_or(ParseError::SyntaxError)?;
        Ok(match name {
            "inp" => Self::Input(rest.parse()?),
            "add" | "mul" | "div" | "mod" | "eql" => {
                let (a, b) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
                match name {
                    "add" => Self::Add(a.parse()?, b.parse()?),
                    "mul" => Self::Mul(a.parse()?, b.parse()?),
                    "div" => Self::Div(a.parse()?, b.parse()?),
                    "mod" => Self::Mod(a.parse()?, b.parse()?),
                    "eql" => Self::Eql(a.parse()?, b.parse()?),
                    _ => unreachable!(),
                }
            }
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(a) => write!(f, "{a} = input();"),
            Self::Add(a, b) => write!(f, "{a} += {b};"),
            Self::Mul(a, b) => write!(f, "{a} *= {b};"),
            Self::Div(a, b) => write!(f, "{a} /= {b};"),
            Self::Mod(a, b) => write!(f, "{a} %= {b};"),
            Self::Eql(a, b) => write!(f, "{a} = ({a} == {b});"),
        }
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Result<Vec<Op>, ParseError> {
    input.lines().map(str::parse).collect()
}

struct Machine<'a> {
    ops: &'a [Op],
    ip: usize,
    reg: [Value; Reg::all().len()],
    input: VecDeque<i64>,
}

impl<'a> Machine<'a> {
    const fn new(ops: &'a [Op]) -> Self {
        Self {
            ops,
            ip: 0,
            reg: [0; Reg::all().len()],
            input: VecDeque::new(),
        }
    }
}

impl Index<Reg> for Machine<'_> {
    type Output = Value;

    fn index(&self, index: Reg) -> &Self::Output {
        &self.reg[index as usize]
    }
}

impl IndexMut<Reg> for Machine<'_> {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.reg[index as usize]
    }
}

#[derive(Debug, Error)]
enum MachineError {
    #[error("End of input")]
    EndOfInput,
}

impl Machine<'_> {
    fn evaluate(&self, value: RegOrValue) -> i64 {
        match value {
            RegOrValue::Reg(reg) => self[reg],
            RegOrValue::Value(val) => val,
        }
    }

    const fn save(&self) -> (usize, [Value; Reg::all().len()]) {
        (self.ip, self.reg)
    }

    const fn revert(&mut self, state: (usize, [Value; Reg::all().len()])) {
        self.ip = state.0;
        self.reg = state.1;
    }

    fn step(&mut self) -> Result<bool, MachineError> {
        if self.ip >= self.ops.len() {
            return Ok(false);
        }
        print!("[{}] {}", self.ip, self.ops[self.ip]);
        match self.ops[self.ip] {
            Op::Input(a) => self[a] = self.input.pop_front().ok_or(MachineError::EndOfInput)?,
            Op::Add(a, b) => self[a] += self.evaluate(b),
            Op::Mul(a, b) => self[a] *= self.evaluate(b),
            Op::Div(a, b) => self[a] /= self.evaluate(b),
            Op::Mod(a, b) => self[a] %= self.evaluate(b),
            Op::Eql(a, b) => self[a] = i64::from(self[a] == self.evaluate(b)),
        }
        println!(" -- {:?}", self.reg);
        self.ip += 1;
        Ok(true)
    }

    fn run(&mut self) -> Result<(), MachineError> {
        while self.step()? {}
        Ok(())
    }

    fn run_until_input(&mut self) -> bool {
        loop {
            match self.step() {
                Ok(true) => (),
                Ok(false) => return false,
                Err(MachineError::EndOfInput) => return true,
            }
        }
    }
}

#[aoc(day24, part1)]
fn part_1(ops: &[Op]) -> Value {
    let mut machine = Machine::new(ops);
    let start = machine.save();
    let mut input1: VecDeque<i64> = [9, 9, 2, 9, 8, 9, 9, 3, 1, 9, 9, 8, 7, 3].into();
    while machine.run_until_input()
        && let Some(next) = input1.pop_front()
    {
        println!();
        println!("{:?}", machine.reg);
        println!();
        machine.input.push_back(next);
    }
    println!("{:?}", machine.reg);
    println!("---");
    machine.revert(start);

    let mut input2: VecDeque<i64> = [7, 3, 1, 8, 1, 2, 2, 1, 1, 9, 7, 1, 1, 1].into();
    while machine.run_until_input()
        && let Some(next) = input2.pop_front()
    {
        println!("{:?}", machine.reg);
        println!();
        machine.input.push_back(next);
    }
    println!("{:?}", machine.reg);

    0
}
