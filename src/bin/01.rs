use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64,
    combinator::map, sequence::preceded, IResult, Parser,
};

advent_of_code::solution!(1);

fn parse_rotation(line: &str) -> i64 {
    let result: IResult<&'_ str, _> = alt((
        preceded(tag("R"), i64),
        map(preceded(tag("L"), i64), |left| -left),
    ))
    .parse(line);

    result.unwrap().1
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_rotation)
            .fold((50, 0), |(mut pos, mut count), curr| {
                pos += curr;
                if pos.rem_euclid(100) == 0 {
                    count += 1;
                }
                (pos, count)
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (pos, count) = input.lines().map(parse_rotation).fold(
        (50, 0),
        |(mut pos, mut count), mut curr| {
            let cycles = curr / 100;

            count += cycles.abs();
            curr %= 100;

            pos += curr;

            match pos {
                ..=-1 => {
                    count += 1;
                    pos += 100;
                }
                100.. => {
                    count += 1;
                    pos -= 100;
                }
                _ => {}
            }

            (pos, count)
        },
    );

    Some(count + (pos == 0) as i64)
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
        assert_eq!(result, Some(6));
    }
}
