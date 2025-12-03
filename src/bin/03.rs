advent_of_code::solution!(3);

fn max_in_range(values: &[u8], start: usize, end: usize) -> (u8, usize) {
    values[start..end].iter().copied().enumerate().fold(
        (b'0', 0),
        |(max, max_p), (p, c)| {
            if c > max {
                (c, p + start)
            } else {
                (max, max_p)
            }
        },
    )
}

fn max_value_with_digits(line: &str, digits: usize) -> u64 {
    let len = line.len();
    let bytes = line.as_bytes();

    let mut start = 0;

    (0..digits)
        .map(|d| {
            let (max, pos) = max_in_range(bytes, start, len - digits + d + 1);
            start = pos + 1;
            max
        })
        .fold(0, |acc, v| acc * 10 + (v - b'0') as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| max_value_with_digits(line, 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| max_value_with_digits(line, 12))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
