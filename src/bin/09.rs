use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::preceded, IResult, Parser,
};

advent_of_code::solution!(9);

fn parse_tiles(input: &str) -> IResult<&'_ str, Vec<(u64, u64)>> {
    separated_list1(tag("\n"), (u64, preceded(tag(","), u64))).parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input).unwrap().1;

    tiles
        .iter()
        .copied()
        .cartesian_product(tiles.iter().copied())
        .map(|((x1, y1), (x2, y2))| {
            (x2.abs_diff(x1) + 1) * (y1.abs_diff(y2) + 1)
        })
        .max()
}

struct Rect {
    area: u64,
    x_min: u64,
    x_max: u64,
    y_min: u64,
    y_max: u64,
}

impl Rect {
    fn new(p1: (u64, u64), p2: (u64, u64)) -> Self {
        // We are only interested in the interior of the rectangle, so we
        // mangle the coordinates a little so that we are effectively giving
        // the center of the point that contained the corner.
        //
        // We precompute the area so we don't have to reverse all the mangling
        // later to find it
        Self {
            area: (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1),
            x_min: p1.0.min(p2.0) * 2 + 1,
            x_max: p1.0.max(p2.0) * 2 - 1,
            y_min: p1.1.min(p2.1) * 2 + 1,
            y_max: p1.1.max(p2.1) * 2 - 1,
        }
    }

    fn contains_any_of(&self, s: &Segment) -> bool {
        match s {
            Segment::Horizontal { y, x_min, x_max } => {
                if *y > self.y_max || *y < self.y_min {
                    return false;
                }
                if self.x_min > *x_max || self.x_max < *x_min {
                    return false;
                }
            }
            Segment::Vertical { x, y_min, y_max } => {
                if *x > self.x_max || *x < self.x_min {
                    return false;
                }
                if self.y_min > *y_max || self.y_max < *y_min {
                    return false;
                }
            }
        }

        true
    }
}

enum Segment {
    Horizontal { y: u64, x_min: u64, x_max: u64 },
    Vertical { x: u64, y_min: u64, y_max: u64 },
}

impl Segment {
    fn new(p1: (u64, u64), p2: (u64, u64)) -> Self {
        // Just like with the rectangle we double the scale of the coordinate
        // system so that we distinguish the interior of shapes from segments.
        if p1.0 == p2.0 {
            Self::Vertical {
                x: p1.0 * 2,
                y_min: p1.1.min(p2.1) * 2,
                y_max: p1.1.max(p2.1) * 2,
            }
        } else {
            Self::Horizontal {
                y: p1.1 * 2,
                x_min: p1.0.min(p2.0) * 2,
                x_max: p1.0.max(p2.0) * 2,
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = parse_tiles(input).unwrap().1;

    let segments = tiles
        .iter()
        .copied()
        .circular_tuple_windows::<(_, _)>()
        .map(|(p1, p2)| Segment::new(p1, p2))
        .collect::<Vec<_>>();

    tiles
        .iter()
        .copied()
        .cartesian_product(tiles.iter().copied())
        .map(|(s1, s2)| Rect::new(s1, s2))
        .sorted_by_key(|r| r.area)
        .rev()
        .filter(|r| !segments.iter().any(|s| r.contains_any_of(s)))
        .map(|r| r.area)
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
