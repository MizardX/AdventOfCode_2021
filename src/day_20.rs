use std::fmt::{Display, Write};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}
#[derive(Debug, Error)]
enum ResizeError {
    #[error("New size must be larger")]
    TooSmall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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

    fn expand(&mut self, width: usize, height: usize, offset: Pos) -> Result<(), ResizeError>
    where
        T: Copy + Default,
    {
        if width < self.width
            || height < self.height
            || offset.x + self.width > width
            || offset.y + self.height > height
        {
            return Err(ResizeError::TooSmall);
        }
        self.data.resize_with(width * height, Default::default);
        for y in (0..self.height).rev() {
            self.data.copy_within(
                self.width * y..self.width * (y + 1),
                (offset.y + y) * width + offset.x,
            );
            self.data
                [self.width * y..(self.width * (y + 1)).min((offset.y + y) * width + offset.x)]
                .fill_with(Default::default);
        }
        self.width = width;
        self.height = height;
        Ok(())
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

impl Display for Grid<Pixel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row1, row2) in self
            .data
            .chunks(self.width)
            .zip(self.data.chunks(self.width).skip(1))
            .step_by(2)
        {
            for (&cell1, &cell2) in row1.iter().zip(row2) {
                match (cell1, cell2) {
                    (Pixel::Off, Pixel::Off) => f.write_char(' ')?,
                    (Pixel::Off, Pixel::On) => f.write_char('▄')?,
                    (Pixel::On, Pixel::Off) => f.write_char('▀')?,
                    (Pixel::On, Pixel::On) => f.write_char('█')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Pixel {
    #[default]
    Off,
    On,
}

impl TryFrom<u8> for Pixel {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Off,
            b'#' => Self::On,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[derive(Debug, Clone)]
struct Input {
    algorithm: Vec<Pixel>,
    image: Grid<Pixel>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let algorithm = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .flat_map(|l| l.bytes().map(Pixel::try_from))
            .collect::<Result<Vec<_>, _>>()?;
        let width = lines.clone().next().ok_or(ParseError::SyntaxError)?.len();
        let height = lines.clone().count();
        let pixels = lines
            .flat_map(|l| l.bytes().map(Pixel::try_from))
            .collect::<Result<Vec<_>, _>>()?;
        let image = Grid::new(pixels, width, height);
        Ok(Self { algorithm, image })
    }
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Result<Input, ParseError> {
    input.parse()
}

#[aoc(day20, part1)]
fn part_1(input: &Input) -> usize {
    transform_n(input, 2)
}

#[aoc(day20, part2)]
fn part_2(input: &Input) -> usize {
    transform_n(input, 50)
}

fn transform_n(input: &Input, cycles: usize) -> usize {
    let mut image = input.image.clone();
    image
        .expand(
            image.width + 2 * cycles,
            image.height + 2 * cycles,
            Pos::new(cycles, cycles),
        )
        .unwrap();
    let mut next = image.clone();
    next.data.fill(Pixel::Off);
    let mut outside = Pixel::Off;
    for _ in (0..cycles).step_by(2) {
        outside = transform_once(&image, &mut next, &input.algorithm, outside);
        outside = transform_once(&next, &mut image, &input.algorithm, outside);
    }
    image.data.iter().filter(|p| matches!(p, Pixel::On)).count()
}

fn transform_once(
    source: &Grid<Pixel>,
    target: &mut Grid<Pixel>,
    algorithm: &[Pixel],
    outside: Pixel,
) -> Pixel {
    for y in 0..target.height {
        for x in 0..target.width {
            let pos = Pos::new(x, y);
            let mut pixels = [outside; 9];
            if let Some(y1) = y.checked_sub(1) {
                if let Some(x1) = x.checked_sub(1) {
                    pixels[0] = source[Pos::new(x1, y1)];
                }
                pixels[1] = source[Pos::new(x, y1)];
                if x + 1 < source.width {
                    pixels[2] = source[Pos::new(x + 1, y1)];
                }
            }
            if let Some(x1) = x.checked_sub(1) {
                pixels[3] = source[Pos::new(x1, y)];
            }
            pixels[4] = source[Pos::new(x, y)];
            if x + 1 < source.width {
                pixels[5] = source[Pos::new(x + 1, y)];
            }
            if y + 1 < source.height {
                if let Some(x1) = x.checked_sub(1) {
                    pixels[6] = source[Pos::new(x1, y + 1)];
                }
                pixels[7] = source[Pos::new(x, y + 1)];
                if x + 1 < source.width {
                    pixels[8] = source[Pos::new(x + 1, y + 1)];
                }
            }
            let index = pixels
                .into_iter()
                .fold(0, |s, px| (s << 1) | usize::from(px == Pixel::On));
            target[pos] = algorithm[index];
        }
    }
    algorithm[match outside {
        Pixel::Off => 0b000_000_000,
        Pixel::On => 0b111_111_111,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\n\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\n\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\n\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\n\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\n\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\n\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
        \n\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###\
    ";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE).unwrap();
        let result = part_2(&input);
        assert_eq!(result, 3_351);
    }
}
