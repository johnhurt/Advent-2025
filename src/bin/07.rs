use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{Compass, Grid};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<u8> = Grid::parse_lines(input);

    let start = grid
        .data
        .iter()
        .copied()
        .enumerate()
        .find_map(|(i, d)| (d == b'S').then_some(i));

    let mut queue = VecDeque::from_iter(start);
    let mut seen = HashSet::new();
    let mut splits = 0;

    while let Some(curr) = queue.pop_front() {
        if !seen.insert(curr) {
            continue;
        }

        if grid.data[curr] == b'^' {
            queue.extend(
                grid.step_from_index(curr, Compass::W)
                    .into_iter()
                    .chain(grid.step_from_index(curr, Compass::E)),
            );
            splits += 1;
        } else {
            queue.extend(grid.step_from_index(curr, Compass::S));
        }
    }

    Some(splits)
}

fn paths_down_from(
    grid: &Grid,
    i: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(known) = memo.get(&i).copied() {
        return known;
    };

    let result = match grid.data[i] {
        b'^' => [Compass::E, Compass::W]
            .into_iter()
            .filter_map(|d| grid.step_from_index(i, d))
            .map(|j| paths_down_from(grid, j, memo))
            .sum(),
        _ => {
            if let Some(j) = grid.step_from_index(i, Compass::S) {
                paths_down_from(grid, j, memo)
            } else {
                1
            }
        }
    };

    memo.insert(i, result);

    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Grid = Grid::parse_lines(input);
    let mut memo = HashMap::new();

    let start = grid
        .data
        .iter()
        .copied()
        .enumerate()
        .find_map(|(i, d)| (d == b'S').then_some(i))
        .unwrap();

    Some(paths_down_from(&grid, start, &mut memo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
