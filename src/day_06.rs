use std::num::ParseIntError;

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

#[aoc(day6, part1)]
fn part_1(fishes: &[u8]) -> u64 {
    simulate(fishes, 80)
}

#[aoc(day6, part2)]
fn part_2(fishes: &[u8]) -> u64 {
    simulate(fishes, 256)
}

fn simulate(fishes: &[u8], time: usize) -> u64 {
    let mut counts = [0; 9];
    for &f in fishes {
        counts[f as usize] += 1;
    }
    for t in 0..time {
        counts[(t + 7) % 9] += counts[t % 9];
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test_case(EXAMPLE, 80 => 5_934)]
    #[test_case(EXAMPLE, 256 => 26_984_457_539)]
    fn test_simulate(input: &str, time: usize) -> u64 {
        let fishes = parse(input).unwrap();
        simulate(&fishes, time)
    }
}
