use std::collections::BinaryHeap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

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

    fn pos_to_index(&self, pos: Pos) -> Option<usize> {
        ((0..self.width).contains(&pos.x) && (0..self.height).contains(&pos.y))
            .then_some(pos.x + self.width * pos.y)
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        let index = self.pos_to_index(pos).expect("Index out of range");
        &self.data[index]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let index = self.pos_to_index(pos).expect("Index out of range");
        &mut self.data[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    dist: usize,
    pos: Pos,
}

impl Node {
    const fn new(pos: Pos, dist: usize) -> Self {
        Self { dist, pos }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(grid: &Grid<u8>, start: Pos, goal: Pos) -> usize {
    let mut distance = Grid::new(vec![usize::MAX; grid.data.len()], grid.width, grid.height);
    let mut came_from = Grid::new(vec![None; grid.data.len()], grid.width, grid.height);
    distance[Pos { x: 0, y: 0 }] = 0;
    let mut pending = BinaryHeap::<Node>::new();
    pending.push(Node::new(start, 0));
    while let Some(node) = pending.pop() {
        if node.dist > distance[node.pos] {
            continue;
        }
        if node.pos == goal {
            break;
        }
        let Pos { x, y } = node.pos;
        for next in [
            x.checked_sub(1).map(|x1| Pos { x: x1, y }),
            (x + 1 < grid.width).then_some(Pos { x: x + 1, y }),
            y.checked_sub(1).map(|y1| Pos { x, y: y1 }),
            (y + 1 < grid.height).then_some(Pos { x, y: y + 1 }),
        ]
        .into_iter()
        .flatten()
        {
            let new_dist = node.dist + (grid[next] - b'0') as usize;
            if new_dist < distance[next] {
                let new_node = Node::new(next, new_dist);
                distance[next] = new_dist;
                came_from[next] = Some(node.pos);
                pending.push(new_node);
            }
        }
    }
    distance[goal]
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Grid<u8> {
    let mut data = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for row in input.lines() {
        height += 1;
        width = width.max(row.len());
        data.extend_from_slice(row.as_bytes());
    }
    Grid::new(data, width, height)
}

#[aoc(day15, part1)]
fn part_1(grid: &Grid<u8>) -> usize {
    shortest_path(
        grid,
        Pos { x: 0, y: 0 },
        Pos {
            x: grid.width - 1,
            y: grid.height - 1,
        },
    )
}

#[aoc(day15, part2)]
fn part_2(grid: &Grid<u8>) -> usize {
    let mut larger = Vec::with_capacity(25 * grid.data.len());
    for gy in 0..5 {
        for row in grid.data.chunks(grid.width) {
            for gx in 0..5 {
                for &ch in row {
                    larger.push((ch - b'1' + gx + gy) % 9 + b'1');
                }
            }
        }
    }
    let larger = Grid::new(larger, 5 * grid.width, 5 * grid.height);
    shortest_path(
        &larger,
        Pos { x: 0, y: 0 },
        Pos {
            x: larger.width - 1,
            y: larger.height - 1,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        1163751742\n\
        1381373672\n\
        2136511328\n\
        3694931569\n\
        7463417111\n\
        1319128137\n\
        1359912421\n\
        3125421639\n\
        1293138521\n\
        2311944581\
    ";

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE);
        let result = part_1(&grid);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE);
        let result = part_2(&grid);
        assert_eq!(result, 315);
    }
}
