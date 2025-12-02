use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, u64},
    combinator::{map, map_parser, peek},
    IResult, Parser,
};

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
    min_digits: usize,
    max_digits: usize,
}

impl Range {
    fn invalid_ids(self) -> impl Iterator<Item = u64> {
        (2..=self.max_digits)
            .flat_map(move |base| self.invalid_ids_for_base(base))
            .unique() // <- ids like 222222 can appear multiple times
    }

    fn invalid_ids_for_base(&self, base: usize) -> impl Iterator<Item = u64> {
        let Self {
            start,
            end,
            min_digits,
            max_digits,
        } = *self;

        let bounds = start..=end;

        // If neither digit count is a multiple of the base, there are no
        // invalid ids, so it would make sense to early return here, but the
        // return type prevents that, so instead we just force the below
        // calculations to return an empty set
        let digit_count = if max_digits % base == 0 {
            max_digits
        } else if min_digits % base == 0 {
            min_digits
        } else {
            0
        };

        let group_digit_count = digit_count / base;
        let group_decimal_mask = 10_u64.pow(group_digit_count as u32);
        let top_group_decimal_mask = group_decimal_mask.pow(base as u32 - 1);

        // min and max of the most significant group when split into `base`
        // groups.
        let min_top =
            (start / top_group_decimal_mask).max(group_decimal_mask / 10);
        let max_top =
            (end / top_group_decimal_mask).min(group_decimal_mask - 1);

        (min_top..=max_top)
            .map(move |part| {
                // Create all the numbers by repeating the `part` `base` times
                (0..base).fold(0, |res, _| res * group_decimal_mask + part)
            })
            .filter(move |n| bounds.contains(n))
    }
}

// We need to know the number and decimal length of the number, so this function
// does both at the same time with nom
fn parse_number_with_digit_length(
    input: &str,
) -> IResult<&'_ str, (usize, u64)> {
    map_parser(digit1, (map(peek(digit1), |n: &'_ str| n.len()), u64))
        .parse(input)
}

fn parse_range(range: &str) -> Range {
    let result: IResult<&'_ str, _> = map(
        (
            parse_number_with_digit_length,
            tag("-"),
            parse_number_with_digit_length,
        ),
        |((left_len, left), _, (right_len, right))| Range {
            start: left,
            end: right,
            min_digits: left_len.min(right_len),
            max_digits: left_len.max(right_len),
        },
    )
    .parse(range);

    result.unwrap().1
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split(',')
            .map(parse_range)
            .flat_map(|r| r.invalid_ids_for_base(2))
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split(',')
            .map(parse_range)
            .flat_map(Range::invalid_ids)
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
