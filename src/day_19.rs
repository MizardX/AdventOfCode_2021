use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, rest) = s.split_once(',').ok_or(ParseError::SyntaxError)?;
        let (y, z) = rest.split_once(',').ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scanner {
    id: usize,
    beacons: Vec<Point>,
}

impl FromStr for Scanner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .strip_prefix("--- scanner ")
            .ok_or(ParseError::SyntaxError)?
            .strip_suffix(" ---")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        let beacons = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { id, beacons })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    Negative,
    Zero,
    Positive,
}

impl From<Sign> for i32 {
    fn from(value: Sign) -> Self {
        match value {
            Sign::Negative => -1,
            Sign::Zero => 0,
            Sign::Positive => 1,
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Negative => f.write_str("-1"),
            Self::Zero => f.write_str(" 0"),
            Self::Positive => f.write_str(" 1"),
        }
    }
}

#[derive(Clone, Copy)]
struct Rotation {
    m11: Sign,
    m12: Sign,
    m13: Sign,
    m21: Sign,
    m22: Sign,
    m23: Sign,
    m31: Sign,
    m32: Sign,
    m33: Sign,
}

impl Rotation {
    const fn new([m11, m12, m13, m21, m22, m23, m31, m32, m33]: [Sign; 9]) -> Self {
        Self {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        }
    }

    fn rotate(self, p: Point) -> Point {
        Point {
            x: p.x * i32::from(self.m11) + p.y * i32::from(self.m12) + p.z * i32::from(self.m13),
            y: p.x * i32::from(self.m21) + p.y * i32::from(self.m22) + p.z * i32::from(self.m23),
            z: p.x * i32::from(self.m31) + p.y * i32::from(self.m32) + p.z * i32::from(self.m33),
        }
    }

    const fn all() -> [Self; 24] {
        const P: Sign = Sign::Positive;
        const Z: Sign = Sign::Zero;
        const N: Sign = Sign::Negative;
        [
            Self::new([P, Z, Z, Z, P, Z, Z, Z, P]),
            Self::new([P, Z, Z, Z, N, Z, Z, Z, N]),
            Self::new([P, Z, Z, Z, Z, P, Z, N, Z]),
            Self::new([P, Z, Z, Z, Z, N, Z, P, Z]),
            Self::new([N, Z, Z, Z, P, Z, Z, Z, N]),
            Self::new([N, Z, Z, Z, N, Z, Z, Z, P]),
            Self::new([N, Z, Z, Z, Z, P, Z, P, Z]),
            Self::new([N, Z, Z, Z, Z, N, Z, N, Z]),
            Self::new([Z, P, Z, P, Z, Z, Z, Z, N]),
            Self::new([Z, P, Z, N, Z, Z, Z, Z, P]),
            Self::new([Z, P, Z, Z, Z, P, P, Z, Z]),
            Self::new([Z, P, Z, Z, Z, N, N, Z, Z]),
            Self::new([Z, N, Z, P, Z, Z, Z, Z, P]),
            Self::new([Z, N, Z, N, Z, Z, Z, Z, N]),
            Self::new([Z, N, Z, Z, Z, P, N, Z, Z]),
            Self::new([Z, N, Z, Z, Z, N, P, Z, Z]),
            Self::new([Z, Z, P, P, Z, Z, Z, P, Z]),
            Self::new([Z, Z, P, N, Z, Z, Z, N, Z]),
            Self::new([Z, Z, P, Z, P, Z, N, Z, Z]),
            Self::new([Z, Z, P, Z, N, Z, P, Z, Z]),
            Self::new([Z, Z, N, P, Z, Z, Z, N, Z]),
            Self::new([Z, Z, N, N, Z, Z, Z, P, Z]),
            Self::new([Z, Z, N, Z, P, Z, P, Z, Z]),
            Self::new([Z, Z, N, Z, N, Z, N, Z, Z]),
        ]
    }
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        } = self;
        write!(
            f,
            "[[{m11}, {m12}, {m13}], [{m21}, {m22}, {m23}], [{m31}, {m32}, {m33}]]"
        )
    }
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Result<Vec<Scanner>, ParseError> {
    input.split("\n\n").map(str::parse).collect()
}

#[aoc(day19, part1)]
fn part_1(input: &[Scanner]) -> usize {
    let (beacons, scanners) = find_beacons(input);
    println!();
    println!("BEACONS = {beacons:?}");
    println!();
    println!("SCANNERS = {scanners:?}");
    println!();
    beacons.len()
}

fn find_beacons(input: &[Scanner]) -> (Vec<Point>, Vec<(Point, Rotation)>) {
    // Start with everything relative to scanner 0.
    let mut beacons = input[0].beacons.clone();
    beacons.sort_unstable();
    let mut scanners = HashMap::<usize, (Point, Rotation)>::new();
    scanners.insert(0, (Point::new(0, 0, 0), Rotation::all()[0]));
    let mut beacon_deltas = beacons
        .iter()
        .enumerate()
        .map(|(index, p1)| {
            let mut deltas = beacons
                .iter()
                .map(|p2| (p2.x - p1.x, p2.y - p1.y, p2.z - p1.z))
                .collect::<Vec<_>>();
            deltas.sort_unstable_by_key(|d| (d.0 * d.0 + d.1 * d.1 + d.2 * d.2, d.0, d.1, d.2));
            deltas.truncate(12);
            (index, deltas)
        })
        .collect::<Vec<_>>();

    let mut matched_scanners = HashSet::new();
    matched_scanners.insert(0);
    let total_scanners = scanners.len();
    while matched_scanners.len() < total_scanners {
        for s in input {
            if matched_scanners.contains(&s.id) {
                continue;
            }
            for rot in Rotation::all() {
                let mut pts = s.beacons.iter().map(|&p| rot.rotate(p)).collect::<Vec<_>>();
                pts.sort_unstable();
                let mut matches = HashMap::new();
                for (i, &p1) in pts.iter().enumerate() {
                    let mut vecs = pts
                        .iter()
                        .map(|p2| (p2.x - p1.x, p2.y - p1.y, p2.z - p1.z))
                        .collect::<Vec<_>>();
                    vecs.sort_unstable_by_key(|d| {
                        (d.0 * d.0 + d.1 * d.1 + d.2 * d.2, d.0, d.1, d.2)
                    });
                    if let Some(&match_) = beacon_deltas.iter().find_map(|(ix, v)| {
                        (v.iter().filter(|v1| vecs.contains(v1)).count() >= 4).then_some(ix)
                    }) {
                        matches.insert(i, match_);
                    }
                }
                if matches.len() > 3 {
                    matched_scanners.insert(s.id);
                    let dx = *matches
                        .iter()
                        .map(|(&pt_ix, &beacon_ix)| beacons[beacon_ix].x - pts[pt_ix].x)
                        .collect::<Vec<_>>()
                        .select_nth_unstable(matches.len() / 2)
                        .1;
                    let dy = *matches
                        .iter()
                        .map(|(&pt_ix, &beacon_ix)| beacons[beacon_ix].y - pts[pt_ix].y)
                        .collect::<Vec<_>>()
                        .select_nth_unstable(matches.len() / 2)
                        .1;
                    let dz = *matches
                        .iter()
                        .map(|(&pt_ix, &beacon_ix)| beacons[beacon_ix].z - pts[pt_ix].z)
                        .collect::<Vec<_>>()
                        .select_nth_unstable(matches.len() / 2)
                        .1;
                    scanners.insert(s.id, (Point::new(dx, dy, dz), rot));
                    beacons.extend(
                        pts.iter()
                            .map(|&pt| Point::new(pt.x + dx, pt.y + dy, pt.z + dz)),
                    );
                    beacons.sort_unstable();
                    beacons.dedup();
                    beacon_deltas = beacons
                        .iter()
                        .enumerate()
                        .map(|(index, p1)| {
                            let mut deltas = beacons
                                .iter()
                                .map(|p2| (p2.x - p1.x, p2.y - p1.y, p2.z - p1.z))
                                .collect::<Vec<_>>();
                            deltas.sort_unstable_by_key(|d| {
                                (d.0 * d.0 + d.1 * d.1 + d.2 * d.2, d.0, d.1, d.2)
                            });
                            deltas.truncate(12);
                            (index, deltas)
                        })
                        .collect::<Vec<_>>();
                    break; // Orientations
                }
            }
        }
    }
    let mut scanners_vec = scanners.into_iter().collect::<Vec<_>>();
    scanners_vec.sort_unstable_by_key(|t| t.0);
    let scanners_vec = scanners_vec.into_iter().map(|t| t.1).collect();
    (beacons, scanners_vec)
}
