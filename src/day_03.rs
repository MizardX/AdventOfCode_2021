use smallvec::SmallVec;

type Number = SmallVec<[u8; 12]>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Number> {
    input.lines().map(|s| s.as_bytes().into()).collect()
}

#[aoc(day3, part1)]
fn part_1(input: &[Number]) -> u64 {
    let mut counts = [0; 12];
    let mut total = 0;
    for num in input {
        total += 1;
        for (count, &bit) in counts.iter_mut().zip(num) {
            *count += u32::from(bit == b'1');
        }
    }
    let width = input[0].len();
    let gamma_rate = counts[..width]
        .iter()
        .fold(0, |sum, &count| (sum << 1) | u32::from(count * 2 >= total));
    let epsilon_rate = !(!0 << width) ^ gamma_rate;
    u64::from(gamma_rate) * u64::from(epsilon_rate)
}

#[aoc(day3, part2)]
fn part_2(input: &[Number]) -> u64 {
    let mut input = input.to_vec();
    input.sort_unstable();
    let oxygen_rating = get_rating(&input, true);
    let co2_rating = get_rating(&input, false);

    let oxygen_rating = oxygen_rating
        .into_iter()
        .fold(0, |sum, bit| (sum << 1) + u64::from(bit == b'1'));
    let co2_rating = co2_rating
        .into_iter()
        .fold(0, |sum, bit| (sum << 1) + u64::from(bit == b'1'));
    oxygen_rating * co2_rating
}

fn get_rating(mut numbers: &[Number], upper: bool) -> Number {
    for index in 0..numbers[0].len() {
        let zeros = numbers.iter().take_while(|num| num[index] == b'0').count();
        let ones = numbers.len() - zeros;
        if (zeros <= ones) ^ upper {
            numbers = &numbers[..zeros];
        } else {
            numbers = &numbers[zeros..];
        }
        if numbers.len() <= 1 {
            break;
        }
    }
    numbers[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010\
    ";

    #[test]
    fn test_part_1() {
        let numbers = parse(EXAMPLE);
        let result = part_1(&numbers);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_part_2() {
        let numbers = parse(EXAMPLE);
        let result = part_2(&numbers);
        assert_eq!(result, 230);
    }
}
