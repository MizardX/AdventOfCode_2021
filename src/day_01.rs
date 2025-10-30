use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part_1(depths: &[u32]) -> usize {
    depths
        .iter()
        .zip(&depths[1..])
        .filter(|&(&x, &y)| y > x)
        .count()
}

#[aoc(day1, part2)]
fn part_2(depths: &[u32]) -> usize {
    depths
        .iter()
        .zip(&depths[3..])
        .filter(|&(&x, &y)| y > x)
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        199\n\
        200\n\
        208\n\
        210\n\
        200\n\
        207\n\
        240\n\
        269\n\
        260\n\
        263\
    ";

    #[test]
    fn test_part_1() {
        let depths = parse(EXAMPLE).unwrap();
        let result = part_1(&depths);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_2() {
        let depths = parse(EXAMPLE).unwrap();
        let result = part_2(&depths);
        assert_eq!(result, 5);
    }
}