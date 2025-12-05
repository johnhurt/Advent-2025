advent_of_code::solution!(5);

use std::{collections::BTreeMap, ops::RangeInclusive};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u64, combinator::map,
    multi::separated_list1, sequence::terminated, IResult, Parser,
};

fn parse_ranges(input: &str) -> (&'_ str, Vec<RangeInclusive<u64>>) {
    let result: IResult<&'_ str, _> = terminated(
        separated_list1(
            tag("\n"),
            map((u64, tag("-"), u64), |(start, _, end)| start..=end),
        ),
        tag("\n"),
    )
    .parse(input);

    result.unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rest, ranges) = parse_ranges(input);

    Some(
        rest.lines()
            .filter_map(|line| line.parse::<u64>().ok())
            .filter(|v| ranges.iter().any(|r| r.contains(v)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, ranges) = parse_ranges(input);

    let mut starts = BTreeMap::new();
    let mut ends = BTreeMap::new();

    for (range_id, range) in ranges.into_iter().enumerate() {
        let mut start = *range.start();
        let mut end = *range.end();

        // If this range is fully contained by another, do nothing
        if starts
            .range(..(start, 0_usize))
            .next_back()
            .filter(|((outer_start, _), outer_end)| {
                let outer_range = *outer_start..=**outer_end;
                outer_range.contains(&start) && outer_range.contains(&end)
            })
            .is_some()
        {
            continue;
        }

        // any ranges that start within the current range
        let starts_contained = starts
            .range((start, 0)..(end, usize::MAX))
            .map(|((start, index), end)| (*index, *start, *end))
            .collect_vec();

        // any ranges that end within the current range
        let ends_contained = ends
            .range((start, 0)..(end, usize::MAX))
            .map(|((end, index), start)| (*index, *start, *end))
            .collect_vec();

        // Join all the overlapping ranges and remove any previously existing
        // ranges
        for (i, s, e) in starts_contained.into_iter().chain(ends_contained) {
            starts.remove(&(s, i));
            ends.remove(&(e, i));

            start = start.min(s);
            end = end.max(e);
        }

        // add the new joined range
        starts.insert((start, range_id), end);
        ends.insert((end, range_id), start);
    }

    Some(starts.into_iter().map(|((s, _), e)| e - s + 1).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
