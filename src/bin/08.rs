use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use i_key_sort::sort::one_key::OneKeySort;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u64, combinator::map,
    multi::separated_list1, sequence::delimited, IResult, Parser,
};

advent_of_code::solution!(8);

#[cfg(test)]
const CONNECTIONS: usize = 10;

#[cfg(not(test))]
const CONNECTIONS: usize = 1000;

fn parse_boxes(input: &str) -> IResult<&'_ str, Vec<[u64; 3]>> {
    separated_list1(
        tag("\n"),
        map(
            (u64, delimited(tag(","), u64, tag(",")), u64),
            |(x, y, z)| [x, y, z],
        ),
    )
    .parse(input)
}

/// Precompute and sort all the distances (squared) between all points. This is
/// a little wasteful for part 1, but useful for part 2
fn calc_distances(boxes: &[[u64; 3]]) -> Vec<(u64, (usize, usize))> {
    let mut distances = (0..boxes.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| {
            (
                boxes[i]
                    .iter()
                    .zip_eq(boxes[j].iter())
                    .map(|(c_1, c_2)| c_1.abs_diff(*c_2).pow(2))
                    .sum::<u64>(),
                (i, j),
            )
        })
        .collect_vec();

    distances.sort_by_one_key(true, |v| v.0);

    distances
}

/// Get the size of all the connected segments in the graph using the shortest
/// k edges
fn calc_segments(
    boxes: &[[u64; 3]],
    distances: &[(u64, (usize, usize))],
    k: usize,
) -> Vec<u64> {
    let mut edges = HashMap::new();
    distances[0..k]
        .iter()
        .copied()
        .flat_map(|(_, (i, j))| [(i, j), (j, i)])
        .for_each(|(i, j)| {
            edges
                .entry(i)
                .and_modify(|v: &mut Vec<_>| v.push(j))
                .or_insert(vec![j]);
        });

    let mut segments = vec![];

    let mut todo: HashSet<_> = (0..boxes.len()).collect();
    let mut queue = VecDeque::new();

    while !todo.is_empty() {
        queue.extend(todo.iter().next().copied());
        let mut segment_size = 0;

        while let Some(curr) = queue.pop_front() {
            if !todo.remove(&curr) {
                continue;
            }

            segment_size += 1;

            queue.extend(edges.get(&curr).iter().copied().flatten());
        }

        segments.push(segment_size);
    }

    segments
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = parse_boxes(input).unwrap().1;
    let distances = calc_distances(&boxes);

    let mut segments = calc_segments(&boxes, &distances, CONNECTIONS);
    segments.sort_unstable();

    Some(segments.iter().rev().take(3).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_boxes(input).unwrap().1;
    let distances = calc_distances(&boxes);

    let possible_k = (0..distances.len()).collect_vec();

    // Binary search on k to find the first edge that creates a totally
    // connected graph
    let min_k = possible_k
        .binary_search_by(|k| {
            let len = calc_segments(&boxes, &distances, *k).len();

            if len > 1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();

    let (_, (from, to)) = distances[min_k - 1];

    Some(boxes[from][0] * boxes[to][0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
