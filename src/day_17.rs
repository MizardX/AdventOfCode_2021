use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TargetArea {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl FromStr for TargetArea {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //target area: x=70..125, y=-159..-121
        let (x_low, rest) = s
            .strip_prefix("target area: x=")
            .ok_or(ParseError::SyntaxError)?
            .split_once("..")
            .ok_or(ParseError::SyntaxError)?;
        let (x_high, rest) = rest.split_once(", y=").ok_or(ParseError::SyntaxError)?;
        let (y_low, y_high) = rest.split_once("..").ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            x_range: (x_low.parse()?, x_high.parse()?),
            y_range: (y_low.parse()?, y_high.parse()?),
        })
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<TargetArea, ParseError> {
    input.parse()
}

#[aoc(day17, part1)]
const fn part_1(target_area: &TargetArea) -> i32 {
    // X-direction does not matter, since x and y are independant.
    // If I fire upwards at velocity `dy`, it will eventually come back
    // down at exactly `y = 0` at some time `t`. The next tick (time `t + 1`),
    // it will reach down to `-dy`. The faster we fire, the higher the arc, and
    // the further down we will hit in the next tick.
    // Optimally, we want to hit as far down as possible, giving us the highest
    // arc. So fiering at dy=-y1 will make it reach the top at y=dy*(dy-1)/2, and
    // fall down to hit y=y1.
    let dy = -target_area.y_range.0;
    // arc height:
    dy * (dy - 1) / 2
}

#[aoc(day17, part2)]
fn part_2(target_area: &TargetArea) -> usize {
    // X-velocity must be chosen small enough that it doesn't overshoot the area in the first tick.
    // Y-velocity must be chosen large enough that it doesn't overshoot the y-range in the first tick,
    //   and small enough that an arching shot doesn't pass trough the range. (from part 1).

    let stride = target_area.x_range.1 + 1;
    let offset = -target_area.y_range.0 * stride;
    let size = usize::try_from(-target_area.y_range.0 * 2 * stride).unwrap();
    let mut seen = vec![false; size];
    // index = dx + dy * stride + offset
    // 0 <= dx + dy * stride + offset < size

    let mut count = 0;
    let max_time = -2 * target_area.y_range.0;
    for time in 1..=max_time {
        for y in target_area.y_range.0..=target_area.y_range.1 {
            // projected y position at time
            // t*dy - t*(t - 1)/2 = y
            // solve for dy
            // dy = (t^2 - t + 2*y)/(2*t)
            // The dy required to reach this y-position at this time t.
            if (time * (time - 1) + 2 * y) % (2 * time) == 0 {
                let dy0 = (time * (time - 1) + 2 * y) / (2 * time);
                // Projected x-position when reaching vertical
                // x = t*dx - t*(t - 1)/2
                // set equal to the next tick t + 1
                // t*dx - t*(t - 1)/2 = (t + 1)*dx - (t + 1)*t/2
                // solve for t
                // t = dx
                // insert back into the projection
                // x = dx^2 - dx*(dx - 1)/2 = (dx + 1)*dx/2
                // set equal to the target range x1..=x2, and solve for dx
                // dx = -1/2 + sqrt(1 + 8*x1)/2..-1/2 + sqrt(1 + 8*x2)/2
                // Iterate over integers in this range
                let m1 = ceil_isqrt(1 + 8 * target_area.x_range.0);
                let m2 = i32::isqrt(1 + 8 * target_area.x_range.1);
                for m in m1..=m2 {
                    if m % 2 == 0 {
                        continue;
                    }
                    let dx0 = (m - 1) / 2;
                    if dx0 < time {
                        let seen_this =
                            &mut seen[usize::try_from(dx0 + dy0 * stride + offset).unwrap()];
                        count += usize::from(!*seen_this);
                        *seen_this = true;
                    }
                }
                for x in target_area.x_range.0..=target_area.x_range.1 {
                    if (time * (time - 1) + 2 * x) % (2 * time) == 0 {
                        let dx0 = (time * (time - 1) + 2 * x) / (2 * time);
                        if dx0 >= time {
                            let seen_this =
                                &mut seen[usize::try_from(dx0 + dy0 * stride + offset).unwrap()];
                            count += usize::from(!*seen_this);
                            *seen_this = true;
                        }
                    }
                }
            }
        }
    }
    count
}

const fn ceil_isqrt(val: i32) -> i32 {
    let res = val.isqrt();
    if res * res < val { res + 1 } else { res }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part_1() {
        let target_area = parse(EXAMPLE).unwrap();
        let result = part_1(&target_area);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part_2() {
        let target_area = parse(EXAMPLE).unwrap();
        let result = part_2(&target_area);
        assert_eq!(result, 112);
    }
}
