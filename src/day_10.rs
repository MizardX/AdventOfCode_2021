#[aoc(day10, part1)]
fn part_1(input: &[u8]) -> u64 {
    let mut stack = Vec::new();
    input
        .split(|&ch| ch == b'\n')
        .map(|line| {
            stack.clear();
            for &ch in line {
                match ch {
                    b'(' => stack.push(b')'),
                    b'[' => stack.push(b']'),
                    b'{' => stack.push(b'}'),
                    b'<' => stack.push(b'>'),
                    _ => {
                        if let Some(check) = stack.pop()
                            && ch != check
                        {
                            return match ch {
                                b')' => 3,
                                b']' => 57,
                                b'}' => 1197,
                                b'>' => 25137,
                                _ => unreachable!(),
                            };
                        }
                    }
                }
            }
            0
        })
        .sum()
}

#[aoc(day10, part2)]
fn part_2(input: &[u8]) -> u64 {
    let mut stack = Vec::new();
    let mut scores = input
        .split(|&ch| ch == b'\n')
        .filter_map(|line| {
            stack.clear();
            for &ch in line {
                match ch {
                    b'(' => stack.push(b')'),
                    b'[' => stack.push(b']'),
                    b'{' => stack.push(b'}'),
                    b'<' => stack.push(b'>'),
                    _ => {
                        if let Some(check) = stack.pop()
                            && ch != check
                        {
                            return None;
                        }
                    }
                }
            }
            let mut sum = 0;
            while let Some(ch) = stack.pop() {
                sum = sum * 5
                    + match ch {
                        b')' => 1,
                        b']' => 2,
                        b'}' => 3,
                        b'>' => 4,
                        _ => unreachable!(),
                    };
            }
            Some(sum)
        })
        .collect::<Vec<_>>();
    let n = scores.len();
    *scores.select_nth_unstable(n/2).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"\
        [({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]\
    ";

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 26_397);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 288_957);
    }
}
