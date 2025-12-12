use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, usize},
    combinator::map,
    multi::{many0, many1, separated_list1},
    IResult, Parser,
};

advent_of_code::solution!(12);

#[derive(Debug)]
struct Present {
    _appearances: Vec<[u8; 3]>,
    weight: usize,
}

impl Present {
    fn parse(v: &str) -> IResult<&'_ str, Self> {
        map(
            (
                (many0(tag("\n")), digit1, tag(":\n")),
                separated_list1(
                    tag("\n"),
                    many1(map(alt((tag("."), tag("#"))), |c| match c {
                        "#" => 1_u8,
                        _ => 0,
                    })),
                ),
            ),
            |(_, chars)| Self::new(chars),
        )
        .parse(v)
    }

    fn new(mut bits: Vec<Vec<u8>>) -> Self {
        fn rotate(shape: &[Vec<u8>]) -> Vec<Vec<u8>> {
            let mut result = vec![vec![0; 3]; 3];

            (0..3).for_each(|i| {
                (0_usize..3).for_each(|j| {
                    //let col = i;
                    // let col_dir = (col != 2) as isize * 2 - 1;
                    // let col_offset = if col_dir < 0 { 2 } else { 0 };
                    result[j][2 - i] = shape[i][j];
                });
            });

            result
        }

        let weight = bits.iter().flatten().sum::<u8>() as usize;

        let mut all_appearances = vec![];

        for _ in 0..4 {
            all_appearances.push(bits.clone());
            bits = rotate(&bits);
        }

        bits.reverse();
        for _ in 0..4 {
            all_appearances.push(bits.clone());
            bits = rotate(&bits);
        }

        let _appearances = all_appearances
            .into_iter()
            .map(|shape| {
                shape
                    .into_iter()
                    .map(|line| {
                        line.into_iter().fold(0, |acc, c| (acc << 1) + c)
                    })
                    .collect_array()
                    .unwrap()
            })
            .unique()
            .collect_vec();

        Self {
            _appearances,
            weight,
        }
    }
}

#[derive(Debug)]
struct Opening {
    width: usize,
    height: usize,

    presents: Vec<usize>,
}

impl Opening {
    fn parse(input: &str) -> IResult<&'_ str, Self> {
        map(
            (
                usize,
                tag("x"),
                usize,
                tag(": "),
                separated_list1(tag(" "), usize),
            ),
            |(width, _, height, _, presents)| Self {
                width,
                height,
                presents,
            },
        )
        .parse(input)
    }

    fn space_left_over(&self, presents: &[Present]) -> Option<usize> {
        (self.height * self.width).checked_sub(
            self.presents
                .iter()
                .copied()
                .enumerate()
                .map(|(p, c)| presents[p].weight * c)
                .sum::<usize>(),
        )
    }
}

#[derive(Debug)]
struct Setup {
    presents: Vec<Present>,
    openings: Vec<Opening>,
}

impl Setup {
    fn parse(input: &str) -> IResult<&'_ str, Self> {
        map(
            (
                many1(Present::parse),
                ws(separated_list1(tag("\n"), Opening::parse)),
            ),
            |(presents, openings)| Self { presents, openings },
        )
        .parse(input)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let s = Setup::parse(input).unwrap().1;

    Some(
        s.openings
            .iter()
            .filter(|o| o.space_left_over(&s.presents).is_some())
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
