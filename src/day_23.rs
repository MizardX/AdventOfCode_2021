use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Write};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error("Expected more lines")]
    EndOfInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    Blocked = 7,
}

impl Cell {
    const fn from_u128(val: u128) -> Option<Self> {
        Some(match val {
            0 => Self::Empty,
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            7 => Self::Blocked,
            _ => return None,
        })
    }

    #[must_use]
    const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    #[must_use]
    const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }

    #[must_use]
    const fn is_occupied(self) -> bool {
        matches!(self, Self::A | Self::B | Self::C | Self::D)
    }

    const fn cost(self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1_000,
            _ => 0xFFFF,
        }
    }
}

impl TryFrom<u128> for Cell {
    type Error = ();

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Empty,
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            7 => Self::Blocked,
            _ => return Err(()),
        })
    }
}

impl TryFrom<u8> for Cell {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Empty,
            b'A' => Self::A,
            b'B' => Self::B,
            b'C' => Self::C,
            b'D' => Self::D,
            b'#' => Self::Blocked,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Empty => '.',
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::Blocked => '%',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Room {
    Hallway(Hallway),
    A(Level),
    B(Level),
    C(Level),
    D(Level),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    One,
    Two,
    Three,
    Four,
}

impl Level {
    const fn level(self) -> usize {
        self as usize + 1
    }

    const fn from_level(level: usize) -> Option<Self> {
        Some(match level {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hallway {
    Left1,
    Left2,
    Mid1,
    Mid2,
    Mid3,
    Right1,
    Right2,
}

impl Hallway {
    const fn column(self) -> usize {
        match self {
            Self::Left1 => 0,
            Self::Left2 => 1,
            Self::Mid1 => 3,
            Self::Mid2 => 5,
            Self::Mid3 => 7,
            Self::Right1 => 9,
            Self::Right2 => 10,
        }
    }
    const fn all() -> [Self; 7] {
        [
            Self::Left1,
            Self::Left2,
            Self::Mid1,
            Self::Mid2,
            Self::Mid3,
            Self::Right1,
            Self::Right2,
        ]
    }
}

impl Room {
    const fn index(self) -> usize {
        match self {
            Self::Hallway(h) => h as usize,
            Self::A(l) => l as usize + 7,
            Self::B(l) => l as usize + 11,
            Self::C(l) => l as usize + 15,
            Self::D(l) => l as usize + 19,
        }
    }
    const fn column(self) -> usize {
        match self {
            Self::Hallway(h) => h.column(),
            Self::A(_) => 2,
            Self::B(_) => 4,
            Self::C(_) => 6,
            Self::D(_) => 8,
        }
    }
    const fn level(self) -> usize {
        match self {
            Self::Hallway(_) => 0,
            Self::A(l) | Self::B(l) | Self::C(l) | Self::D(l) => l.level(),
        }
    }
    const fn set_level(self, level: Level) -> Option<Self> {
        Some(match self {
            Self::Hallway(_) => return None,
            Self::A(_) => Self::A(level),
            Self::B(_) => Self::B(level),
            Self::C(_) => Self::C(level),
            Self::D(_) => Self::D(level),
        })
    }
    const fn distance(self, other: Self) -> usize {
        self.column().abs_diff(other.column()) + self.level().abs_diff(other.level())
    }
    const fn all() -> [Self; 23] {
        [
            Self::Hallway(Hallway::Left1),
            Self::Hallway(Hallway::Left2),
            Self::Hallway(Hallway::Mid1),
            Self::Hallway(Hallway::Mid2),
            Self::Hallway(Hallway::Mid3),
            Self::Hallway(Hallway::Right1),
            Self::Hallway(Hallway::Right2),
            Self::A(Level::One),
            Self::A(Level::Two),
            Self::A(Level::Three),
            Self::A(Level::Four),
            Self::B(Level::One),
            Self::B(Level::Two),
            Self::B(Level::Three),
            Self::B(Level::Four),
            Self::C(Level::One),
            Self::C(Level::Two),
            Self::C(Level::Three),
            Self::C(Level::Four),
            Self::D(Level::One),
            Self::D(Level::Two),
            Self::D(Level::Three),
            Self::D(Level::Four),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct State {
    bits: u128,
}

impl State {
    const fn room(self, room: Room) -> Cell {
        let shift = room.index() * 3;
        Cell::from_u128((self.bits >> shift) & 0b111).unwrap()
    }

    const fn set_room(&mut self, room: Room, cell: Cell) {
        let shift = room.index() * 3;
        self.bits = self.bits & !(!(!0 << 3) << shift) | (cell as u128) << shift;
    }

    const fn moves(self) -> Moves {
        Moves::new(self)
    }

    const fn is_goal(self) -> bool {
        self.bits == GOAL1.bits || self.bits == GOAL2.bits
    }
}

const GOAL1: State = {
    let mut st = State { bits: 0 };
    st.set_room(Room::A(Level::One), Cell::A);
    st.set_room(Room::A(Level::Two), Cell::A);
    st.set_room(Room::A(Level::Three), Cell::Blocked);
    st.set_room(Room::A(Level::Four), Cell::Blocked);

    st.set_room(Room::B(Level::One), Cell::B);
    st.set_room(Room::B(Level::Two), Cell::B);
    st.set_room(Room::B(Level::Three), Cell::Blocked);
    st.set_room(Room::B(Level::Four), Cell::Blocked);

    st.set_room(Room::C(Level::One), Cell::C);
    st.set_room(Room::C(Level::Two), Cell::C);
    st.set_room(Room::C(Level::Three), Cell::Blocked);
    st.set_room(Room::C(Level::Four), Cell::Blocked);

    st.set_room(Room::D(Level::One), Cell::D);
    st.set_room(Room::D(Level::Two), Cell::D);
    st.set_room(Room::D(Level::Three), Cell::Blocked);
    st.set_room(Room::D(Level::Four), Cell::Blocked);
    st
};

const GOAL2: State = {
    let mut st = State { bits: 0 };
    st.set_room(Room::A(Level::One), Cell::A);
    st.set_room(Room::A(Level::Two), Cell::A);
    st.set_room(Room::A(Level::Three), Cell::A);
    st.set_room(Room::A(Level::Four), Cell::A);

    st.set_room(Room::B(Level::One), Cell::B);
    st.set_room(Room::B(Level::Two), Cell::B);
    st.set_room(Room::B(Level::Three), Cell::B);
    st.set_room(Room::B(Level::Four), Cell::B);

    st.set_room(Room::C(Level::One), Cell::C);
    st.set_room(Room::C(Level::Two), Cell::C);
    st.set_room(Room::C(Level::Three), Cell::C);
    st.set_room(Room::C(Level::Four), Cell::C);

    st.set_room(Room::D(Level::One), Cell::D);
    st.set_room(Room::D(Level::Two), Cell::D);
    st.set_room(Room::D(Level::Three), Cell::D);
    st.set_room(Room::D(Level::Four), Cell::D);
    st
};

impl FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        if let Some(line) = lines.next()
            && line != "#############"
        {
            return Err(ParseError::SyntaxError);
        }
        let mut state = Self::default();
        if let [
            b'#',
            l1,
            l2,
            b'.',
            m1,
            b'.',
            m2,
            b'.',
            m3,
            b'.',
            r1,
            r2,
            b'#',
        ] = *lines.next().ok_or(ParseError::EndOfInput)?.as_bytes()
        {
            state.set_room(Room::Hallway(Hallway::Left1), l1.try_into()?);
            state.set_room(Room::Hallway(Hallway::Left2), l2.try_into()?);
            state.set_room(Room::Hallway(Hallway::Mid1), m1.try_into()?);
            state.set_room(Room::Hallway(Hallway::Mid2), m2.try_into()?);
            state.set_room(Room::Hallway(Hallway::Mid3), m3.try_into()?);
            state.set_room(Room::Hallway(Hallway::Right1), r1.try_into()?);
            state.set_room(Room::Hallway(Hallway::Right2), r2.try_into()?);
        } else {
            return Err(ParseError::SyntaxError);
        }
        for lev in 1..=4 {
            let level = Level::from_level(lev).unwrap();
            let line = lines.next().ok_or(ParseError::EndOfInput)?;
            if let [a, b'#', b, b'#', c, b'#', d] = *line.trim_matches([' ', '~', '#']).as_bytes() {
                state.set_room(Room::A(level), a.try_into()?);
                state.set_room(Room::B(level), b.try_into()?);
                state.set_room(Room::C(level), c.try_into()?);
                state.set_room(Room::D(level), d.try_into()?);
            } else if lev > 2 && line.trim_matches(['~', ' ']) == "#########" {
                for lev1 in lev..=4 {
                    let level = Level::from_level(lev1).unwrap();
                    state.set_room(Room::A(level), Cell::Blocked);
                    state.set_room(Room::B(level), Cell::Blocked);
                    state.set_room(Room::C(level), Cell::Blocked);
                    state.set_room(Room::D(level), Cell::Blocked);
                }
                break;
            } else {
                return Err(ParseError::SyntaxError);
            }
        }
        Ok(state)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#{}", self.room(Room::Hallway(Hallway::Left1)))?;
        write!(f, "{}", self.room(Room::Hallway(Hallway::Left2)))?;
        write!(f, ".{}", self.room(Room::Hallway(Hallway::Mid1)))?;
        write!(f, ".{}", self.room(Room::Hallway(Hallway::Mid2)))?;
        write!(f, ".{}", self.room(Room::Hallway(Hallway::Mid3)))?;
        write!(f, ".{}", self.room(Room::Hallway(Hallway::Right1)))?;
        writeln!(f, "{}#", self.room(Room::Hallway(Hallway::Right2)))?;
        write!(f, "###{}", self.room(Room::A(Level::One)))?;
        write!(f, "#{}", self.room(Room::B(Level::One)))?;
        write!(f, "#{}", self.room(Room::C(Level::One)))?;
        writeln!(f, "#{}###", self.room(Room::D(Level::One)))?;
        write!(f, "  #{}", self.room(Room::A(Level::Two)))?;
        write!(f, "#{}", self.room(Room::B(Level::Two)))?;
        write!(f, "#{}", self.room(Room::C(Level::Two)))?;
        writeln!(f, "#{}#", self.room(Room::D(Level::Two)))?;
        write!(f, "  #{}", self.room(Room::A(Level::Three)))?;
        write!(f, "#{}", self.room(Room::B(Level::Three)))?;
        write!(f, "#{}", self.room(Room::C(Level::Three)))?;
        writeln!(f, "#{}#", self.room(Room::D(Level::Three)))?;
        write!(f, "  #{}", self.room(Room::A(Level::Four)))?;
        write!(f, "#{}", self.room(Room::B(Level::Four)))?;
        write!(f, "#{}", self.room(Room::C(Level::Four)))?;
        writeln!(f, "#{}#", self.room(Room::D(Level::Four)))?;
        writeln!(f, "  #########")
    }
}

struct Moves {
    state: State,
    source: Room,
    destination: Room,
}

impl Moves {
    const fn new(state: State) -> Self {
        Self {
            state,
            source: Room::all()[0],
            destination: Room::all()[0],
        }
    }

    const fn next_source(&mut self) -> bool {
        let ix = self.source.index() + 1;
        if ix >= Room::all().len() {
            return false;
        }
        self.source = Room::all()[ix];
        self.destination = Room::all()[0];
        true
    }
    const fn next_destination(&mut self) -> bool {
        let ix = self.destination.index() + 1;
        if ix >= Room::all().len() {
            return false;
        }
        self.destination = Room::all()[ix];
        true
    }
    const fn step(&mut self) -> bool {
        self.next_destination() || self.next_source()
    }

    /// Only move between hallway to room, or the reverse
    const fn only_between_room_and_hallway(&self) -> bool {
        !matches!(
            (self.source, self.destination),
            (Room::Hallway(_), Room::Hallway(_))
                | (
                    Room::A(_) | Room::B(_) | Room::C(_) | Room::D(_),
                    Room::A(_) | Room::B(_) | Room::C(_) | Room::D(_),
                )
        )
    }

    /// Only move if the hallway in between is empty
    fn hallway_empty(&self) -> bool {
        for h in Hallway::all() {
            let col1 = self.source.column();
            let col2 = self.destination.column();
            let col = h.column();
            if (col1 < col && col < col2 || col2 < col && col < col1)
                && self.state.room(Room::Hallway(h)).is_occupied()
            {
                return false;
            }
        }
        true
    }

    /// Only move if the path out of a room is empty
    fn empty_path_out(&self) -> bool {
        if let Room::A(l1) | Room::B(l1) | Room::C(l1) | Room::D(l1) = self.source {
            for l in 1..l1.level() {
                let room1 = self
                    .source
                    .set_level(Level::from_level(l).unwrap())
                    .unwrap();
                if !self.state.room(room1).is_empty() {
                    return false;
                }
            }
        }
        true
    }

    /// Don't move an amphipods out of its room if doesn't need to make way for some
    /// different color below it.
    fn dont_move_settled_amphipod(&self) -> bool {
        let cell = self.state.room(self.source);
        if let (Cell::A, Room::A(l1))
        | (Cell::B, Room::B(l1))
        | (Cell::C, Room::C(l1))
        | (Cell::D, Room::D(l1)) = (cell, self.source)
        {
            let mut only_matching = true;
            for l in l1.level() + 1..=4 {
                let room1 = self
                    .source
                    .set_level(Level::from_level(l).unwrap())
                    .unwrap();
                let cell1 = self.state.room(room1);
                if cell1.is_blocked() {
                    break;
                }
                if cell1 != cell {
                    only_matching = false;
                    break;
                }
            }
            if only_matching {
                return false;
            }
        }
        true
    }

    /// Only move if the path into a room is empty
    fn empty_path_in(&self) -> bool {
        if let Room::A(l2) | Room::B(l2) | Room::C(l2) | Room::D(l2) = self.destination {
            for l in 1..l2.level() {
                let room2 = self
                    .destination
                    .set_level(Level::from_level(l).unwrap())
                    .unwrap();
                if !self.state.room(room2).is_empty() {
                    return false;
                }
            }
        }
        true
    }

    fn room_matching_color(&self) -> bool {
        let cell = self.state.room(self.source);
        if let Room::A(l2) | Room::B(l2) | Room::C(l2) | Room::D(l2) = self.destination {
            for l in l2.level() + 1..=4 {
                let room2 = self
                    .destination
                    .set_level(Level::from_level(l).unwrap())
                    .unwrap();
                let cell2 = self.state.room(room2);
                if cell2.is_blocked() {
                    break;
                }
                if cell2 != cell {
                    return false;
                }
            }
        }
        true
    }
    /// Only move into room if it is empty or contains only amphipods of same color
    const fn room_maching_color2(&self) -> bool {
        let cell = self.state.room(self.source);
        match (cell, self.destination) {
            (Cell::A, Room::A(_))
                if matches!(
                    (
                        self.state.room(Room::A(Level::One)),
                        self.state.room(Room::A(Level::Two)),
                        self.state.room(Room::A(Level::Three)),
                        self.state.room(Room::A(Level::Four))
                    ),
                    (
                        Cell::Empty | Cell::A | Cell::Blocked,
                        Cell::Empty | Cell::A | Cell::Blocked,
                        Cell::Empty | Cell::A | Cell::Blocked,
                        Cell::Empty | Cell::A | Cell::Blocked
                    )
                ) => {}
            (Cell::B, Room::B(_))
                if matches!(
                    (
                        self.state.room(Room::B(Level::One)),
                        self.state.room(Room::B(Level::Two)),
                        self.state.room(Room::B(Level::Three)),
                        self.state.room(Room::B(Level::Four))
                    ),
                    (
                        Cell::Empty | Cell::B | Cell::Blocked,
                        Cell::Empty | Cell::B | Cell::Blocked,
                        Cell::Empty | Cell::B | Cell::Blocked,
                        Cell::Empty | Cell::B | Cell::Blocked
                    )
                ) => {}
            (Cell::C, Room::C(_))
                if matches!(
                    (
                        self.state.room(Room::C(Level::One)),
                        self.state.room(Room::C(Level::Two)),
                        self.state.room(Room::C(Level::Three)),
                        self.state.room(Room::C(Level::Four))
                    ),
                    (
                        Cell::Empty | Cell::C | Cell::Blocked,
                        Cell::Empty | Cell::C | Cell::Blocked,
                        Cell::Empty | Cell::C | Cell::Blocked,
                        Cell::Empty | Cell::C | Cell::Blocked
                    )
                ) => {}
            (Cell::D, Room::D(_))
                if matches!(
                    (
                        self.state.room(Room::D(Level::One)),
                        self.state.room(Room::D(Level::Two)),
                        self.state.room(Room::D(Level::Three)),
                        self.state.room(Room::D(Level::Four))
                    ),
                    (
                        Cell::Empty | Cell::D | Cell::Blocked,
                        Cell::Empty | Cell::D | Cell::Blocked,
                        Cell::Empty | Cell::D | Cell::Blocked,
                        Cell::Empty | Cell::D | Cell::Blocked
                    )
                ) => {}
            (_, Room::A(_) | Room::B(_) | Room::C(_) | Room::D(_)) => {
                return false;
            }
            _ => (),
        }
        true
    }
}

impl Iterator for Moves {
    type Item = (usize, State);

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            while !self.state.room(self.source).is_occupied() {
                if !self.next_source() {
                    return None;
                }
            }
            while !self.state.room(self.destination).is_empty() {
                if !self.next_destination() {
                    if !self.next_source() {
                        return None;
                    }
                    continue 'outer;
                }
            }
            let valid = self.only_between_room_and_hallway()
                && self.hallway_empty()
                && self.empty_path_out()
                && self.dont_move_settled_amphipod()
                && self.empty_path_in()
                && self.room_matching_color()
                && self.room_maching_color2();

            if valid {
                let cell = self.state.room(self.source);
                let mut new_state = self.state;
                new_state.set_room(self.source, Cell::Empty);
                new_state.set_room(self.destination, cell);
                let dist = self.source.distance(self.destination);
                self.step();
                return Some((dist * cell.cost(), new_state));
            }
            self.step();
        }
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<State, ParseError> {
    input.parse()
}

#[aoc(day23, part1)]
fn part_1(state: &State) -> usize {
    find_solution(state)
}

fn find_solution(state: &State) -> usize {
    let mut visited = HashMap::new();
    let mut pending = BinaryHeap::new();
    pending.push((Reverse(0), *state));
    visited.insert(*state, 0);

    let mut goal_distance = None;
    while let Some((Reverse(cost), state)) = pending.pop() {
        if goal_distance.is_some_and(|g| g <= cost) {
            continue;
        }
        if let Some(&best_cost) = visited.get(&state)
            && best_cost < cost
        {
            continue;
        }

        if state.is_goal() {
            goal_distance = Some(cost);
            continue;
        }
        for (delta, next) in state.moves() {
            match visited.entry(next) {
                Entry::Occupied(o) if *o.get() <= cost + delta => continue,
                Entry::Occupied(mut o) => {
                    o.insert(cost + delta);
                }
                Entry::Vacant(v) => {
                    v.insert(cost + delta);
                }
            }
            pending.push((Reverse(cost + delta), next));
        }
    }

    goal_distance.expect("No solution found")
}

#[aoc(day23, part2)]
fn part_2(state: &State) -> usize {
    let mut state = *state;
    state.set_room(Room::A(Level::Four), state.room(Room::A(Level::Two)));
    state.set_room(Room::B(Level::Four), state.room(Room::B(Level::Two)));
    state.set_room(Room::C(Level::Four), state.room(Room::C(Level::Two)));
    state.set_room(Room::D(Level::Four), state.room(Room::D(Level::Two)));
    state.set_room(Room::A(Level::Three), Cell::D);
    state.set_room(Room::B(Level::Three), Cell::B);
    state.set_room(Room::C(Level::Three), Cell::A);
    state.set_room(Room::D(Level::Three), Cell::C);
    state.set_room(Room::A(Level::Two), Cell::D);
    state.set_room(Room::B(Level::Two), Cell::C);
    state.set_room(Room::C(Level::Two), Cell::B);
    state.set_room(Room::D(Level::Two), Cell::A);

    find_solution(&state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "\
        #############\n\
        #...........#\n\
        ###B#C#B#D###\n\
        ~~#A#D#C#A#\n\
        ~~#########\
    ";

    const MOVE1: &str = "\
        #############\n\
        #...B.......#\n\
        ###B#C#.#D###\n\
        ~~#A#D#C#A#\n\
        ~~#########\
    ";

    const MOVE2A: &str = "\
        #############\n\
        #...B.C.....#\n\
        ###B#.#.#D###\n\
        ~~#A#D#C#A#\n\
        ~~#########\
    ";

    const MOVE2B: &str = "\
        #############\n\
        #...B.......#\n\
        ###B#.#C#D###\n\
        ~~#A#D#C#A#\n\
        ~~#########\
    ";

    const MOVE3A: &str = "\
        #############\n\
        #...B.D.....#\n\
        ###B#.#C#D###\n\
        ~~#A#.#C#A#\n\
        ~~#########\
    ";

    const MOVE3B: &str = "\
        #############\n\
        #.....D.....#\n\
        ###B#.#C#D###\n\
        ~~#A#B#C#A#\n\
        ~~#########\
    ";

    const MOVE4A: &str = "\
        #############\n\
        #...B.D.....#\n\
        ###.#.#C#D###\n\
        ~~#A#B#C#A#\n\
        ~~#########\
    ";

    const MOVE4B: &str = "\
        #############\n\
        #.....D.....#\n\
        ###.#B#C#D###\n\
        ~~#A#B#C#A#\n\
        ~~#########\
    ";

    const MOVE5A: &str = "\
        #############\n\
        #.....D.D...#\n\
        ###.#B#C#.###\n\
        ~~#A#B#C#A#\n\
        ~~#########\
    ";

    const MOVE5B: &str = "\
        #############\n\
        #.....D.D.A.#\n\
        ###.#B#C#.###\n\
        ~~#A#B#C#.#\n\
        ~~#########\
    ";

    const MOVE6A: &str = "\
        #############\n\
        #.....D...A.#\n\
        ###.#B#C#.###\n\
        ~~#A#B#C#D#\n\
        ~~#########\
    ";

    const MOVE6B: &str = "\
        #############\n\
        #.........A.#\n\
        ###.#B#C#D###\n\
        ~~#A#B#C#D#\n\
        ~~#########\
    ";

    const MOVE7: &str = "\
        #############\n\
        #...........#\n\
        ###A#B#C#D###\n\
        ~~#A#B#C#D#\n\
        ~~#########\
    ";

    const TEST1: &str = "\
        #############\n\
        #DB........B#\n\
        ###A#C#.#.###\n\
        ~~#A#D#C#.#\n\
        ~~#########\
      ";
    const TEST2A: &str = "\
        #############\n\
        #DB...C....B#\n\
        ###A#.#.#.###\n\
        ~~#A#D#C#.#\n\
        ~~#########\
      ";
    const TEST3A: &str = "\
        #############\n\
        #DB........B#\n\
        ###A#.#C#.###\n\
        ~~#A#D#C#.#\n\
        ~~#########\
      ";
    const TEST4A: &str = "\
        #############\n\
        #DB...D....B#\n\
        ###A#.#C#.###\n\
        ~~#A#.#C#.#\n\
        ~~#########\
      ";
    const TEST5: &str = "\
        #############\n\
        #D....D....B#\n\
        ###A#.#C#.###\n\
        ~~#A#B#C#.#\n\
        ~~#########\
      ";

    const TEST2B: &str = "\
        #############\n\
        #DB.....C..B#\n\
        ###A#.#.#.###\n\
        ~~#A#D#C#.#\n\
        ~~#########\
      ";
    const TEST3B: &str = "\
        #############\n\
        #DB...D.C..B#\n\
        ###A#.#.#.###\n\
        ~~#A#.#C#.#\n\
        ~~#########\
      ";
    const TEST4B: &str = "\
        #############\n\
        #D....D.C..B#\n\
        ###A#.#.#.###\n\
        ~~#A#B#C#.#\n\
        ~~#########\
      ";

    #[test_case(&[EXAMPLE,MOVE1,MOVE2A,MOVE2B,MOVE3A,MOVE3B,MOVE4A,MOVE4B,MOVE5A,MOVE5B,MOVE6A,MOVE6B,MOVE7] => 12_521)]
    #[test_case(&[TEST1,TEST2A,TEST3A,TEST4A,TEST5] => 3_450)]
    #[test_case(&[TEST1,TEST2B,TEST3B,TEST4B,TEST5] => 3_650)]
    fn test_moves(states: &[&str]) -> usize {
        let mut prev = parse(states[0]).unwrap();
        let mut total_cost = 0;
        for next in &states[1..] {
            let next = parse(next).unwrap();
            let (cost, count) = prev
                .moves()
                .filter_map(|(cst, step)| (step == next).then_some(cst))
                .fold((0, 0), |(cst, cnt), new_cost| (cst + new_cost, cnt + 1));
            assert_eq!(count, 1, "Only one matching move");
            total_cost += cost;
            prev = next;
        }
        total_cost
    }

    #[test]
    fn test_cost() {
        let mut prev = parse(EXAMPLE).unwrap();
        for (substeps, expected_cost) in [
            ([MOVE1].as_slice(), 40),
            ([MOVE2A, MOVE2B].as_slice(), 400),
            ([MOVE3A, MOVE3B].as_slice(), 3_030),
            ([MOVE4A, MOVE4B].as_slice(), 40),
            ([MOVE5A, MOVE5B].as_slice(), 2_003),
            ([MOVE6A, MOVE6B].as_slice(), 7_000),
            ([MOVE7].as_slice(), 8),
        ] {
            let mut sum_cost = 0;
            for next in substeps {
                let next = parse(next).unwrap();
                let (cost, count) = prev
                    .moves()
                    .filter_map(|(cst, step)| (step == next).then_some(cst))
                    .fold((0, 0), |(cst, cnt), new_cost| (cst + new_cost, cnt + 1));
                assert_eq!(count, 1, "Only one matching move");
                sum_cost += cost;
                prev = next;
            }
            assert_eq!(sum_cost, expected_cost);
        }
    }

    #[test]
    fn test_part_1() {
        let state = parse(EXAMPLE).unwrap();
        let result = part_1(&state);
        assert_eq!(result, 12_521);
    }

    #[test]
    fn test_part_2() {
        let state = parse(EXAMPLE).unwrap();
        let result = part_2(&state);
        assert_eq!(result, 44_169);
    }
}
