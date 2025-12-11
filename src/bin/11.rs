use std::collections::HashMap;

use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    multi::{many0, many1, separated_list1},
    sequence::terminated,
    IResult, Parser,
};

advent_of_code::solution!(11);

fn parse_graph(input: &str) -> (HashMap<&'_ str, usize>, Vec<Vec<usize>>) {
    let parsed: IResult<&'_ str, _> = separated_list1(
        tag("\n"),
        (
            ws(terminated(alpha1, tag(":"))),
            many1(terminated(alpha1, many0(tag(" ")))),
        ),
    )
    .parse(input);

    let graph = parsed.unwrap().1;

    let nodes = graph
        .iter()
        .flat_map(|(n, es)| Some(*n).into_iter().chain(es.iter().copied()))
        .unique()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    let mut edges = vec![vec![]; nodes.len()];

    graph.iter().for_each(|(n, es)| {
        edges[*nodes.get(n).unwrap()] = es
            .iter()
            .filter_map(|e| nodes.get(e))
            .copied()
            .collect_vec();
    });

    (nodes, edges)
}

fn paths_to(
    from: usize,
    to: usize,
    graph: &[Vec<usize>],
    memo: &mut [Option<usize>],
) -> usize {
    if from == to {
        return 1;
    }

    if let Some(stored) = memo[from] {
        return stored;
    }

    let paths = graph[from]
        .iter()
        .copied()
        .map(|next| paths_to(next, to, graph, memo))
        .sum();

    memo[from] = Some(paths);

    paths
}

fn all_paths(
    from: &str,
    to: &str,
    nodes: &HashMap<&str, usize>,
    edges: &[Vec<usize>],
) -> usize {
    let mut memo = vec![None; nodes.len()];

    paths_to(
        nodes.get(from).copied().unwrap(),
        nodes.get(to).copied().unwrap(),
        edges,
        &mut memo,
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let (nodes, edges) = parse_graph(input);
    Some(all_paths("you", "out", &nodes, &edges))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (nodes, edges) = parse_graph(input);

    debug_assert_eq!(all_paths("dac", "fft", &nodes, &edges), 0);
    debug_assert_eq!(all_paths("out", "dac", &nodes, &edges), 0);
    debug_assert_eq!(all_paths("out", "fft", &nodes, &edges), 0);

    Some(
        all_paths("svr", "fft", &nodes, &edges)
            * all_paths("fft", "dac", &nodes, &edges)
            * all_paths("dac", "out", &nodes, &edges),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        // Example input doesn't work for this one
    }
}
