use advent_of_code::{ws, Grid};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64,
    combinator::{iterator, map},
    multi::many1,
    IResult, Parser,
};

advent_of_code::solution!(6);

fn parse_number_from_line(input: &str) -> IResult<&'_ str, Vec<u64>> {
    many1(ws(u64)).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add(u64),
    Mul(u64),
}

impl Op {
    fn apply(&mut self, val: u64) {
        match self {
            Op::Add(v) => *v += val,
            Op::Mul(v) => *v *= val,
        }
    }

    fn into_inner(self) -> u64 {
        match self {
            Op::Add(v) | Op::Mul(v) => v,
        }
    }
}

fn parse_op(input: &str) -> IResult<&'_ str, (Op, usize)> {
    (
        alt((map(tag("*"), |_| Op::Mul(1)), map(tag("+"), |_| Op::Add(0)))),
        map(many1(tag(" ")), |spaces| spaces.len()),
    )
        .parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().rev();

    let mut accumulators = iterator(lines.next().unwrap(), parse_op)
        .map(|(acc, _)| acc)
        .collect_vec();

    lines.for_each(|line| {
        accumulators
            .iter_mut()
            .zip_eq(parse_number_from_line(line).unwrap().1)
            .for_each(|(acc, v)| acc.apply(v))
    });

    Some(accumulators.into_iter().map(Op::into_inner).sum())
}

fn eval_grid_region(
    grid: &Grid<u8>,
    start: usize,
    columns: usize,
    mut acc: Op,
) -> u64 {
    (0..columns)
        .map(|i| start + i)
        .map(|col| {
            grid.column(col).fold(0, |acc, (_, curr)| match curr {
                b'0'..=b'9' => acc * 10 + (*curr - b'0') as u64,
                _ => acc,
            })
        })
        .for_each(|v| acc.apply(v));

    acc.into_inner()
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid<u8> = Grid::parse_lines(input);
    let last_line = input.lines().next_back().unwrap();

    let mut start = 0;

    let result = iterator(last_line, parse_op)
        .map(|(op, right_spaces)| {
            let mut result = (start, right_spaces);

            // The last column doesn't have any padding, so we have to read up
            // to the end
            if start + right_spaces + 1 == grid.width {
                result.1 += 1;
            }

            start += right_spaces + 1;
            (op, result)
        })
        .map(|(op, (start, len))| eval_grid_region(&grid, start, len, op))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
