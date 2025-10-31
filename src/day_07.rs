use std::num::ParseIntError;

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<u16>, ParseIntError> {
    let mut res = input
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    res.sort_unstable();
    Ok(res)
}

#[aoc(day7, part1)]
fn part_1(positions: &[u16]) -> u32 {
    let n = positions.len();
    let target = positions[n / 2];
    positions
        .iter()
        .map(|&x| u32::from(x.abs_diff(target)))
        .sum()
}

#[aoc(day7, part2)]
fn part_2(positions: &[u16]) -> u32 {
    let n = u32::try_from(positions.len()).unwrap();
    let sum = positions.iter().copied().map(u32::from).sum::<u32>();
    let target = sum / n;
    (target..=target + 1)
        .map(|target| {
            positions
                .iter()
                .map(|&x| u32::from(x).abs_diff(target))
                .map(|dx| dx * (1 + dx) / 2)
                .sum()
        })
        .min()
        .unwrap()
}
