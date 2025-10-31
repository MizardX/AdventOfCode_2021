use std::cmp;
use std::collections::BinaryHeap;
use std::ops::Index;

#[derive(Debug)]
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

    fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
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

#[aoc_generator(day9)]
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

#[aoc(day9, part1)]
fn part_1(grid: &Grid<u8>) -> u32 {
    let mut risk = 0;
    for r in 0..grid.height {
        for c in 0..grid.width {
            risk += match grid[[r, c]] {
                center if r > 0 && grid[[r - 1, c]] <= center => 0,
                center if c > 0 && grid[[r, c - 1]] <= center => 0,
                center if r + 1 < grid.height && grid[[r + 1, c]] <= center => 0,
                center if c + 1 < grid.width && grid[[r, c + 1]] <= center => 0,
                center => u32::from(center - b'0' + 1),
            };
        }
    }
    risk
}

#[aoc(day9, part2)]
fn part_2(grid: &Grid<u8>) -> u32 {
    let mut uf = UnionFind::new(grid.width * grid.height);
    for (r, row) in grid.rows().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            let index = grid.width * r + c;
            if cell != b'9' {
                if r > 0 && grid[[r - 1, c]] != b'9' {
                    uf.union(index - grid.width, index);
                }
                if c > 0 && row[c - 1] != b'9' {
                    uf.union(index - 1, index);
                }
            }
        }
    }
    let mut biggest = BinaryHeap::new();
    for size in uf.root_sizes() {
        biggest.push(cmp::Reverse(size));
        if biggest.len() > 3 {
            biggest.pop();
        }
    }
    biggest.iter().map(|&cmp::Reverse(sz)| sz).product()
}

#[derive(Debug, Clone, Copy)]
struct Node {
    parent: usize,
    size: u32,
}

#[derive(Debug, Clone)]
struct UnionFind {
    nodes: Vec<Node>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).map(|parent| Node { parent, size: 1 }).collect(),
        }
    }

    fn find(&mut self, mut index: usize) -> usize {
        let mut parent = self.nodes[index].parent;
        while index != parent {
            let grand_parent = self.nodes[parent].parent;
            self.nodes[index].parent = grand_parent;
            index = grand_parent;
            parent = self.nodes[index].parent;
        }
        index
    }

    fn union(&mut self, mut index1: usize, mut index2: usize) -> bool {
        index1 = self.find(index1);
        index2 = self.find(index2);
        if index1 == index2 {
            return false;
        }
        if self.nodes[index1].size < self.nodes[index2].size {
            (index1, index2) = (index2, index1);
        }
        self.nodes[index2].parent = index1;
        self.nodes[index1].size += self.nodes[index2].size;
        true
    }

    fn root_sizes(&self) -> impl Iterator<Item = u32> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(ix, node)| (node.parent == ix).then_some(node.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\
    ";

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE);
        let result = part_1(&grid);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE);
        let result = part_2(&grid);
        assert_eq!(result, 1134);
    }
}
