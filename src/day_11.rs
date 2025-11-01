use std::collections::VecDeque;
use std::fmt::{Display, Write};
use std::ops::Index;

#[derive(Debug, Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            data,
            width,
            height,
        }
    }

    fn pos_to_index(&self, row: usize, col: usize) -> Option<usize> {
        ((0..self.width).contains(&col) && (0..self.height).contains(&row))
            .then_some(self.width * row + col)
    }

    fn index_to_pos(&self, index: usize) -> Option<[usize; 2]> {
        (0..self.width * self.height)
            .contains(&index)
            .then_some([index / self.width, index % self.width])
    }
}

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        assert!(
            (0..self.width).contains(&col) && (0..self.height).contains(&row),
            "Index out of range"
        );
        &self.data[row * self.width + col]
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.chunks(self.width) {
            for &cell in row {
                if cell == b'0' {
                    f.write_str("\x1b[97m0\x1b[0m")?;
                } else {
                    let ch = cell as char;
                    write!(f, "\x1b[90m{ch}\x1b[0m")?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[aoc_generator(day11)]
fn parse(input: &[u8]) -> Grid<u8> {
    let mut data = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for row in input.split(|&ch| ch == b'\n') {
        width = row.len();
        height += 1;
        data.extend_from_slice(row);
    }
    Grid::new(data, width, height)
}

#[aoc(day11, part1)]
fn part_1(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut queue = VecDeque::new();
    let mut total = 0;
    for _ in 0..100 {
        let flashes = step(&mut grid, &mut queue);
        total += flashes;
    }
    total
}

#[aoc(day11, part2)]
fn part_2(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut queue = VecDeque::new();
    for t in 1.. {
        let flashes = step(&mut grid, &mut queue);
        if flashes == grid.width * grid.height {
            return t;
        }
    }
    unreachable!()
}

fn step(grid: &mut Grid<u8>, queue: &mut VecDeque<usize>) -> usize {
    queue.clear();
    let mut flashes = 0;
    for (index, cell) in grid.data.iter_mut().enumerate() {
        *cell += 1;
        if *cell == b':' {
            *cell = b'0';
            queue.push_back(index);
            flashes += 1;
        }
    }
    while let Some(index) = queue.pop_front() {
        let [row, col] = grid.index_to_pos(index).unwrap();
        for r in row.saturating_sub(1)..(row + 2).min(grid.height) {
            for c in col.saturating_sub(1)..(col + 2).min(grid.width) {
                let neighbor_index = grid.pos_to_index(r, c).unwrap();
                let neighbor = &mut grid.data[neighbor_index];
                if *neighbor != b'0' {
                    *neighbor += 1;
                    if *neighbor == b':' {
                        *neighbor = b'0';
                        queue.push_back(neighbor_index);
                        flashes += 1;
                    }
                }
            }
        }
    }
    flashes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"\
        5483143223\n\
        2745854711\n\
        5264556173\n\
        6141336146\n\
        6357385478\n\
        4167524645\n\
        2176841721\n\
        6882881134\n\
        4846848554\n\
        5283751526\
    ";

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE);
        let result = part_1(&grid);
        assert_eq!(result, 1656);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE);
        let result = part_2(&grid);
        assert_eq!(result, 195);
    }
}
